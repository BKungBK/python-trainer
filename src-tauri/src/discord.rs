use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;
use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};

pub enum PresenceMsg {
    Update {
        details: String,
        state: String,
    },
    Shutdown,
}

pub struct DiscordPresenceManager {
    sender: Sender<PresenceMsg>,
}

impl DiscordPresenceManager {
    pub fn new(client_id: &'static str) -> Self {
        let (tx, rx) = channel::<PresenceMsg>();
        
        thread::spawn(move || {
            let connect = |id: &str| -> Option<DiscordIpcClient> {
                let mut cli = DiscordIpcClient::new(id);
                if cli.connect().is_ok() {
                    println!("Discord Rich Presence connected successfully.");
                    Some(cli)
                } else {
                    None
                }
            };
            
            let mut client = connect(client_id);
            let mut current_details = "กำลังเลือกโจทย์".to_string();
            let mut current_state = "".to_string();
            let start_timestamp = chrono::Utc::now().timestamp();
            let mut last_update_success = false;
            
            // Set initial activity
            if let Some(ref mut cli) = client {
                let assets = activity::Assets::new()
                    .large_image("iconapp")
                    .large_text("Python Trainer");
                let timestamps = activity::Timestamps::new().start(start_timestamp);
                let payload = activity::Activity::new()
                    .details(&current_details)
                    .state(&current_state)
                    .assets(assets)
                    .timestamps(timestamps);
                if cli.set_activity(payload).is_ok() {
                    last_update_success = true;
                }
            }
            
            loop {
                // If not connected or last update failed, wait 10s before retrying.
                // Otherwise wait longer or block on message.
                let timeout = if client.is_none() || !last_update_success {
                    Duration::from_secs(10)
                } else {
                    Duration::from_secs(60)
                };
                
                match rx.recv_timeout(timeout) {
                    Ok(PresenceMsg::Shutdown) => {
                        if let Some(mut cli) = client.take() {
                            let _ = cli.close();
                        }
                        break;
                    }
                    Ok(PresenceMsg::Update { details, state }) => {
                        current_details = details;
                        current_state = state;
                        
                        if client.is_none() {
                            client = connect(client_id);
                        }
                        
                        if let Some(ref mut cli) = client {
                            let assets = activity::Assets::new()
                                .large_image("iconapp")
                                .large_text("Python Trainer");
                            let timestamps = activity::Timestamps::new().start(start_timestamp);
                            let payload = activity::Activity::new()
                                .details(&current_details)
                                .state(&current_state)
                                .assets(assets)
                                .timestamps(timestamps);
                            if cli.set_activity(payload).is_err() {
                                last_update_success = false;
                                let _ = cli.close();
                                client = None;
                            } else {
                                last_update_success = true;
                            }
                        }
                    }
                    Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
                        // Periodic connection retry
                        if client.is_none() {
                            client = connect(client_id);
                        }
                        
                        if let Some(ref mut cli) = client {
                            let assets = activity::Assets::new()
                                .large_image("iconapp")
                                .large_text("Python Trainer");
                            let timestamps = activity::Timestamps::new().start(start_timestamp);
                            let payload = activity::Activity::new()
                                .details(&current_details)
                                .state(&current_state)
                                .assets(assets)
                                .timestamps(timestamps);
                            if cli.set_activity(payload).is_err() {
                                last_update_success = false;
                                let _ = cli.close();
                                client = None;
                            } else {
                                last_update_success = true;
                            }
                        }
                    }
                    Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                        if let Some(mut cli) = client.take() {
                            let _ = cli.close();
                        }
                        break;
                    }
                }
            }
        });
        
        Self { sender: tx }
    }
    
    pub fn update(&self, details: String, state: String) {
        let _ = self.sender.send(PresenceMsg::Update { details, state });
    }
}
