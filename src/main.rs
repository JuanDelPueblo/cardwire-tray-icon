use futures_util::StreamExt;
use ksni::{Tray, TrayMethods};
use rfd::MessageDialog;
use std::{collections::HashMap, fs, time::Duration};
use tokio::sync::mpsc;
use zbus::{Connection, proxy};

// Define the DBus proxy interface
#[proxy(
    interface = "com.github.opengamingcollective.cardwire",
    default_service = "com.github.opengamingcollective.cardwire",
    default_path = "/com/github/opengamingcollective/cardwire"
)]
trait Cardwire {
    /// Mode property
    #[zbus(property)]
    fn mode(&self) -> zbus::Result<u32>;

    #[zbus(property)]
    fn set_mode(&self, mode: u32) -> zbus::Result<()>;

    /// ListDevices method
    fn list_devices(
        &self,
    ) -> zbus::Result<HashMap<u64, (u32, String, String, u32, u32, bool, bool, bool, String)>>;
}

#[derive(Debug, Clone)]
struct GpuInfo {
    id: u32,
    name: String,
    card: u32,
    is_default: bool,
    blocked: bool,
}

struct CardwireTray {
    mode: u32,
    gpus: Vec<GpuInfo>,
    action_tx: mpsc::Sender<TrayAction>,
}

enum TrayAction {
    SetMode(u32),
    Quit,
}

impl Tray for CardwireTray {
    fn id(&self) -> String {
        "cardwire-tray".to_string()
    }

    fn title(&self) -> String {
        "Cardwire".to_string()
    }

    fn menu(&self) -> Vec<ksni::MenuItem<Self>> {
        let mut items = Vec::new();

        // Header
        items.push(ksni::MenuItem::Standard(ksni::menu::StandardItem {
            label: "id - Name - Power state - default - blocked".to_string(),
            enabled: false,
            ..Default::default()
        }));

        // GPUs
        for gpu in &self.gpus {
            let power_state = fs::read_to_string(format!(
                "/sys/class/drm/card{}/device/power_state",
                gpu.card
            ))
            .unwrap_or_else(|_| "unknown".to_string())
            .trim()
            .to_string();

            let default_str = if gpu.is_default { "(*)" } else { "( )" };
            let label = format!(
                "{} - {} - {} - {} - {}",
                gpu.id, gpu.name, power_state, default_str, gpu.blocked
            );

            items.push(ksni::MenuItem::Standard(ksni::menu::StandardItem {
                label,
                enabled: false,
                ..Default::default()
            }));
        }

        items.push(ksni::MenuItem::Separator);

        // Modes
        let options = vec![
            ksni::menu::RadioItem {
                label: "Integrated Mode".to_string(),
                icon_name: "preferences-system-windows".into(), // Placeholder
                ..Default::default()
            },
            ksni::menu::RadioItem {
                label: "Hybrid Mode".to_string(),
                icon_name: "preferences-system-windows".into(), // Placeholder
                ..Default::default()
            },
            ksni::menu::RadioItem {
                label: "Manual Mode".to_string(),
                icon_name: "preferences-system-windows".into(), // Placeholder
                ..Default::default()
            },
        ];

        let selected_mode_index = if self.mode <= 2 {
            self.mode as usize
        } else {
            0
        };

        items.push(
            ksni::menu::RadioGroup {
                selected: selected_mode_index,
                select: Box::new(|this: &mut Self, index: usize| {
                    let _ = this.action_tx.try_send(TrayAction::SetMode(index as u32));
                }),
                options,
            }
            .into(),
        );

        items.push(ksni::MenuItem::Separator);

        items.push(ksni::MenuItem::Standard(ksni::menu::StandardItem {
            label: "Quit".to_string(),
            icon_name: "application-exit".into(), // Placeholder
            activate: Box::new(|this: &mut Self| {
                let _ = this.action_tx.try_send(TrayAction::Quit);
            }),
            ..Default::default()
        }));

        items
    }
}

async fn get_connection() -> Connection {
    loop {
        match Connection::system().await {
            Ok(conn) => return conn,
            Err(_) => {
                let dialog = MessageDialog::new()
                    .set_level(rfd::MessageLevel::Warning)
                    .set_title("Cardwire DBus Connection")
                    .set_description("Failed to connect to the DBus system bus.")
                    .set_buttons(rfd::MessageButtons::OkCancelCustom(
                        "Retry".into(),
                        "Quit".into(),
                    ));

                if dialog.show() == rfd::MessageDialogResult::Custom("Quit".to_string()) {
                    std::process::exit(1);
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = get_connection().await;

    // Retry loop for the specific service proxy
    let proxy = loop {
        match CardwireProxy::new(&conn).await {
            Ok(p) => {
                if p.mode().await.is_ok() {
                    break p;
                }
            }
            Err(_) => {}
        }

        let dialog = MessageDialog::new()
            .set_level(rfd::MessageLevel::Warning)
            .set_title("Cardwire DBus Service")
            .set_description("Could not access the cardwire DBus service. Is it running?")
            .set_buttons(rfd::MessageButtons::OkCancelCustom(
                "Retry".into(),
                "Quit".into(),
            ));

        if dialog.show() == rfd::MessageDialogResult::Custom("Quit".to_string()) {
            std::process::exit(1);
        }
    };

    let initial_mode = proxy.mode().await.unwrap_or(0);

    let mut gpus = Vec::new();
    if let Ok(devs) = proxy.list_devices().await {
        for (_, (id, name, _, _, card, is_default, blocked, _, _)) in devs {
            gpus.push(GpuInfo {
                id,
                name,
                card,
                is_default,
                blocked,
            });
        }
        gpus.sort_by_key(|g| g.id);
    }

    let (action_tx, mut action_rx) = mpsc::channel(10);
    let tray = CardwireTray {
        mode: initial_mode,
        gpus,
        action_tx,
    };

    let tray_handle = tray.spawn().await.expect("Failed to spawn tray");

    let mut mode_stream = proxy.receive_mode_changed().await;

    let proxy_clone = proxy.clone();
    let handle_clone = tray_handle.clone();

    tokio::spawn(async move {
        loop {
            tokio::select! {
                Some(changed) = mode_stream.next() => {
                    if let Ok(new_mode) = changed.get().await {
                        let _ = handle_clone.update(|tray: &mut CardwireTray| {
                            tray.mode = new_mode;
                        }).await;
                    }
                }
                Some(action) = action_rx.recv() => {
                    match action {
                        TrayAction::SetMode(mode) => {
                            let _ = proxy_clone.set_mode(mode).await;
                        }
                        TrayAction::Quit => {
                            std::process::exit(0);
                        }
                    }
                }
                _ = tokio::time::sleep(Duration::from_secs(5)) => {
                    if let Ok(devs) = proxy_clone.list_devices().await {
                        let mut new_gpus = Vec::new();
                        for (_, (id, name, _, _, card, is_default, blocked, _, _)) in devs {
                            new_gpus.push(GpuInfo {
                                id,
                                name,
                                card,
                                is_default,
                                blocked,
                            });
                        }
                        new_gpus.sort_by_key(|g| g.id);

                        let _ = handle_clone.update(|tray: &mut CardwireTray| {
                            tray.gpus = new_gpus;
                        }).await;
                    }
                }
            }
        }
    });

    // Keeping the main thread alive
    loop {
        std::thread::park();
    }
}
