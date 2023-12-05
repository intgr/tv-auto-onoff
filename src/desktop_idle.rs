use futures::Stream;
use futures::StreamExt;
use log::{debug, warn};
use zbus::{dbus_proxy, Connection};

use crate::util::BoxError;
use crate::LoopEvent;

// https://docs.rs/zbus/latest/zbus/attr.dbus_proxy.html
// https://dbus2.github.io/zbus/client.html#watching-for-changes
#[dbus_proxy(
    default_service = "org.gnome.Mutter.DisplayConfig",
    default_path = "/org/gnome/Mutter/DisplayConfig",
    interface = "org.gnome.Mutter.DisplayConfig"
)]
trait DisplayConfig {
    #[dbus_proxy(property)]
    fn power_save_mode(&self) -> zbus::Result<i32>;
}

/**
 * Monitor D-Bus org.gnome.Mutter.DisplayConfig PowerSaveMode property changes.
 */
pub async fn desktop_events() -> Result<impl Stream<Item = LoopEvent>, BoxError> {
    let connection = Connection::session().await?;

    let display_config_proxy = DisplayConfigProxy::new(&connection).await?;
    let changes_stream = display_config_proxy.receive_power_save_mode_changed().await;

    Ok(changes_stream.then(|msg| async move {
        let value = msg.get().await.expect("Error reading PowerSaveMode");
        debug!("PowerSaveMode value: {value}");

        // https://github.com/jadahl/gnome-monitor-config/blob/04b854e6411cd9ca75582c108aea63ae3c202f0e/src/org.gnome.Mutter.DisplayConfig.xml#L255-L283
        // - 0: on
        // - 1: standby
        // - 2: suspend
        // - 3: off
        // - -1: unknown (unsupported)
        match value {
            0 => LoopEvent::ScreenSaver(false),
            1..=3 => LoopEvent::ScreenSaver(true),
            _ => {
                warn!("Unknown PowerSaveMode: {value}; ignoring");
                LoopEvent::Noop
            }
        }
    }))
}
