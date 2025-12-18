use std::env;
use std::io;
use std::process::Command;

fn main() {
    let mut args = env::args().skip(1);
    let action = match args.next() {
        Some(a) => a.to_lowercase(),
        None => {
            eprintln!(
                "Usage: {} <enable|disable> [device-substring]",
                env::args().next().unwrap_or_default()
            );
            std::process::exit(1);
        }
    };

    let enabled = match action.as_str() {
        "enable" => true,
        "disable" => false,
        _ => {
            eprintln!("Unknown action '{}'. Use 'enable' or 'disable'.", action);
            std::process::exit(1);
        }
    };

    let query = args.next().unwrap_or_else(|| "touchpad".to_string());

    match find_xinput_ids(&query) {
        Ok(ids) if !ids.is_empty() => {
            for id in ids {
                match set_device_enabled(id, enabled) {
                    Ok(()) => println!(
                        "{}d device id {}",
                        if enabled { "Enable" } else { "Disable" },
                        id
                    ),
                    Err(e) => {
                        eprintln!("Failed to set device {}: {}", id, e);
                        std::process::exit(3);
                    }
                }
            }
        }
        Ok(_) => {
            eprintln!("No device matching '{}' found", query);
            std::process::exit(2);
        }
        Err(e) => {
            eprintln!("Error running xinput: {}", e);
            std::process::exit(4);
        }
    }
}

fn find_xinput_ids(query: &str) -> Result<Vec<u32>, io::Error> {
    let out = Command::new("xinput").arg("--list").output()?;
    if !out.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!("xinput failed: {}", String::from_utf8_lossy(&out.stderr)),
        ));
    }

    let s = String::from_utf8_lossy(&out.stdout);
    let mut ids = Vec::new();
    let q_low = query.to_lowercase();

    for line in s.lines() {
        if line.to_lowercase().contains(&q_low) {
            if let Some(pos) = line.find("id=") {
                let rest = &line[pos + 3..];
                let mut digits = String::new();
                for c in rest.chars() {
                    if c.is_ascii_digit() {
                        digits.push(c);
                    } else {
                        break;
                    }
                }
                if !digits.is_empty() {
                    if let Ok(id) = digits.parse::<u32>() {
                        ids.push(id);
                    }
                }
            }
        }
    }

    Ok(ids)
}

fn set_device_enabled(id: u32, enabled: bool) -> Result<(), io::Error> {
    let status = Command::new("xinput")
        .arg("set-prop")
        .arg(id.to_string())
        .arg("Device Enabled")
        .arg(if enabled { "1" } else { "0" })
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("xinput set-prop failed for id {}", id),
        ))
    }
}
