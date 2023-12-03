use futures::Stream;
use futures::StreamExt;
use zbus::{dbus_proxy, Connection};

use crate::util::BoxError;
use crate::LoopEvent;

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
pub async fn desktop_events() -> Result<impl Stream<Item = LoopEvent>, BoxError> {
    let connection = Connection::session().await?;

    // https://dbus2.github.io/zbus/client.html#signals
    let screen_saver = ScreenSaverProxy::new(&connection).await?;
    let changes_stream = screen_saver.receive_active_changed().await?;

    Ok(changes_stream.map(|msg| {
        let new_value: bool = msg.body().expect("Unexpected message from D-Bus");
        LoopEvent::ScreenSaver(new_value)
    }))
}
