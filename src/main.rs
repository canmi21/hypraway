use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct Level {
    timeout: u64,
    command: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    ac: Levels,
    battery: Levels,
    check_interval: u64,
    enable_notifications: bool,
    ac_path: String,
    battery_path: String,
}

#[derive(Debug, Deserialize)]
struct Levels {
    level1: Level,
    level2: Level,
    level3: Level,
}

impl Config {
    fn load() -> Result<Self> {
        let config_path = dirs::home_dir()
            .context("Could not find home directory")?
            .join(".config/hypr/hypraway.conf");
        
        if !config_path.exists() {
            Self::create_default_config(&config_path)?;
        }

        let contents = std::fs::read_to_string(&config_path)
            .context("Could not read config file")?;
        
        serde_yaml::from_str(&contents)
            .context("Could not parse config file")
    }

    fn create_default_config(path: &PathBuf) -> Result<()> {
        let default_config = r#"# Hypraway v1.1 configuration file
check_interval: 5

# Enable notifications for power state changes
enable_notifications: true

# AC power mode
ac_path: "/sys/class/power_supply/AC0/online"
battery_path: "/sys/class/power_supply/BAT0/status"

ac:
  # Level 1: Notify user away
  level1:
    timeout: 600  # 10 minutes
    command: "notify-send -i /path/to/nonexistent/icon \"Hypraway\" -t 2100 \"You have been away\""
  # Level 2: Lock screen
  level2:
    timeout: 1200  # 20 minutes
    command: "hyprlock"
  # Level 3: Suspend or Hibernate
  level3:
    timeout: 0  # disabled
    command: "systemctl hibernate"

# Battery mode
battery:
  # Level 1: Notify user away
  level1:
    timeout: 300  # 5 minutes
    command: "notify-send -i /path/to/nonexistent/icon \"Hypraway\" -t 2100 \"You have been away\""
  # Level 2: Lock screen
  level2:
    timeout: 600  # 10 minutes
    command: "hyprlock"
  # Level 3: Suspend or Hibernate
  level3:
    timeout: 900  # 15 minutes
    command: "systemctl hibernate""#;

        std::fs::create_dir_all(path.parent().unwrap())?;
        std::fs::write(path, default_config)?;
        Ok(())
    }

    fn is_on_battery(&self) -> Result<bool> {
        if let Ok(ac_status) = std::fs::read_to_string(&self.ac_path) {
            let ac_status = ac_status.trim();
            if ac_status == "1" {
                return Ok(false);
            }
        }

        if let Ok(bat_status) = std::fs::read_to_string(&self.battery_path) {
            let bat_status = bat_status.trim();
            if bat_status == "Discharging" {
                return Ok(true);
            }
        }

        if let Ok(ac_status) = std::fs::read_to_string("/sys/class/power_supply/AC0/online") {
            println!("Debug: AC0 online status: {}", ac_status.trim());
        }
        if let Ok(bat_status) = std::fs::read_to_string("/sys/class/power_supply/BAT0/status") {
            println!("Debug: BAT0 status: {}", bat_status.trim());
        }

        println!("Warning: Could not determine power source clearly, defaulting to AC mode");
        Ok(false)
    }

    async fn run(&self) -> Result<()> {
        let mut last_battery_state = self.is_on_battery()?;
        let mut levels = if last_battery_state {
            if self.enable_notifications {
                println!("Running in Battery mode");
            }
            &self.battery
        } else {
            if self.enable_notifications {
                println!("Running in AC power mode");
            }
            &self.ac
        };

        loop {
            let mut command = String::from("swayidle");
            let timeouts = [
                (&levels.level1.timeout, &levels.level1.command),
                (&levels.level2.timeout, &levels.level2.command),
                (&levels.level3.timeout, &levels.level3.command),
            ];

            for (timeout, cmd) in timeouts {
                if *timeout > 0 {
                    command.push_str(&format!(" timeout {} '{}'", timeout, cmd));
                }
            }

            let mut child = tokio::process::Command::new("sh")
                .arg("-c")
                .arg(&command)
                .spawn()?;

            println!("Hypraway started successfully");

            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(self.check_interval)).await;
                
                let current_battery_state = self.is_on_battery()?;
                
                if current_battery_state != last_battery_state {
                    last_battery_state = current_battery_state;
                    levels = if current_battery_state {
                        if self.enable_notifications {
                            println!("Switching to Battery mode");
                            let _ = std::process::Command::new("notify-send")
                                .args([
                                    "-i",
                                    "/path/to/nonexistent/icon",
                                    "Hypraway",
                                    "-t",
                                    "2100",
                                    "Switched to Battery Power Mode"
                                ])
                                .spawn();
                        }
                        &self.battery
                    } else {
                        if self.enable_notifications {
                            println!("Switching to AC power mode");
                            let _ = std::process::Command::new("notify-send")
                                .args([
                                    "-i", 
                                    "/path/to/nonexistent/icon",
                                    "Hypraway",
                                    "-t",
                                    "2100", 
                                    "Switched to AC Power Mode"
                                ])
                                .spawn();
                        }
                        &self.ac
                    };

                    let _ = child.kill().await;
                    break;
                }

                if self.check_interval == 0 {
                    let _ = child.kill().await;
                    return Ok(());
                }
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load()?;
    println!("Loaded configuration: {:?}", config);
    config.run().await?;
    Ok(())
}
