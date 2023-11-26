use std::error::Error;

use log::debug;
use zbus::export::ordered_stream::OrderedStreamExt;
use zbus::{dbus_proxy, Connection};

// https://docs.rs/zbus/latest/zbus/attr.dbus_proxy.html
// TODO try org.freedesktop.ScreenSaver?
#[dbus_proxy(
    default_service = "org.gnome.ScreenSaver",
    default_path = "/org/gnome/ScreenSaver",
    interface = "org.gnome.ScreenSaver"
)]
trait ScreenSaver {
    // https://gitlab.gnome.org/GNOME/gnome-shell/-/blob/main/data/dbus-interfaces/org.gnome.ScreenSaver.xml
    #[dbus_proxy(signal)]
    fn active_changed(&self, new_value: bool) -> fdo::Result<()>;
}

/**
 * Monitor D-Bus ScreenSaver for activation/deactivation.
 */
pub async fn desktop_idle() -> Result<(), Box<dyn Error>> {
    let connection = Connection::session().await?;

    // https://dbus2.github.io/zbus/client.html#signals
    let screen_saver = ScreenSaverProxy::new(&connection).await?;
    let mut changes_stream = screen_saver.receive_active_changed().await?;

    while let Some(msg) = changes_stream.next().await {
        let new_value: bool = msg.body()?;
        debug!("ScreenSaver active: {:?}", new_value);
    }

    // TODO should be error?
    Ok(())
}
