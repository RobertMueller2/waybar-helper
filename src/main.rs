use anyhow::anyhow;
use futures_util::stream::StreamExt;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use swayipc_async::{
    Connection, Event, EventType, Fallible, Node, ShellType, WindowChange, WindowEvent,
};

#[async_std::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let exe: String = args[0].clone();

    if args.len() < 2 {
        eprintln!("Error: not enough arguments");
        print_usage(&exe);
        std::process::exit(1);
    }

    match args[1].as_str() {
        "wayeyes" => match Wayeyes::new_from_args(&args[2..]) {
            Ok(wayeyes) => {
                if let Err(e) = wayeyes.run().await {
                    eprintln!("{e:?}");
                    std::process::exit(2);
                }
            }
            Err(e) => {
                eprintln!("{e:?}");
                print_usage(&exe);
                std::process::exit(2);
            }
        },
        "help" => {
            print_usage(&exe);
        }
        _ => {
            print_usage(&exe);
            std::process::exit(2);
        }
    }
}

#[derive(Default)]
struct OutputItem {
    icon: Option<String>,
    tooltip: Option<String>,
    class: Option<String>,
    percentage: u32,
}

impl OutputItem {
    fn format(&self, fmt: &str) -> String {
        fmt.replace("{icon}", self.icon.as_ref().unwrap_or(&"".to_owned()))
            .replace("{tooltip}", self.tooltip.as_ref().unwrap_or(&"".to_owned()))
            .replace("{class}", self.class.as_ref().unwrap_or(&"".to_owned()))
            .replace("{percentage}", &self.percentage.to_string())
    }
}

struct Wayeyes {
    xdg: OutputItem,
    xwayland: OutputItem,
    unknown: OutputItem,
    format: String,
}

impl Default for Wayeyes {
    fn default() -> Self {
        Wayeyes {
            xdg: OutputItem {
                icon: Some("".into()),
                tooltip: Some("wayland native".into()),
                class: Some("wayland".into()),
                percentage: 100,
            },
            xwayland: OutputItem {
                icon: Some("".into()),
                tooltip: Some("xwayland".into()),
                class: Some("xwayland".into()),
                percentage: 50,
            },
            unknown: OutputItem {
                icon: Some("".into()),
                tooltip: Some("unknown".into()),
                class: Some("unknown".into()),
                percentage: 0,
            },
            format: "{\"text\" : \"{icon}\", \"tooltip\" : \"{tooltip}\", \"class\" : \"{class}\", \"percentage\" : \"{percentage}\" }".into()
        }
    }
}

impl Wayeyes {
    fn new_from_args(args: &[String]) -> Result<Self, anyhow::Error> {
        let mut wayeyes = Self::default();

        let mut i = 0;
        while i < args.len() {
            let a = &args[i];
            i += 1;
            match a.as_str() {
                "--format" => {
                    wayeyes.format = args[i].clone();
                    i += 1;
                }
                "--xdg-icon" => {
                    wayeyes.xdg.icon = Some(args[i].clone());
                    i += 1;
                }
                "--xdg-tooltip" => {
                    wayeyes.xdg.tooltip = Some(args[i].clone());
                    i += 1;
                }
                "--xdg-class" => {
                    wayeyes.xdg.class = Some(args[i].clone());
                    i += 1;
                }
                "--xdg-percentage" => {
                    match args[i].parse::<u32>() {
                        Ok(p) => wayeyes.xdg.percentage = p,
                        Err(e) => {
                            return Err(anyhow!(format!("xdg-percentage parsing error: {}", e)));
                        }
                    }
                    i += 1;
                }
                "--xwayland-icon" => {
                    wayeyes.xwayland.icon = Some(args[i].clone());
                    i += 1;
                }
                "--xwayland-tooltip" => {
                    wayeyes.xwayland.tooltip = Some(args[i].clone());
                    i += 1;
                }
                "--xwayland-class" => {
                    wayeyes.xwayland.class = Some(args[i].clone());
                    i += 1;
                }
                "--xwayland-percentage" => {
                    match args[i].parse::<u32>() {
                        Ok(p) => wayeyes.xwayland.percentage = p,
                        Err(e) => {
                            return Err(anyhow!(format!(
                                "xwayland-percentage parsing error: {}",
                                e
                            )));
                        }
                    }
                    i += 1;
                }
                "--unknown-icon" => {
                    wayeyes.unknown.icon = Some(args[i].clone());
                    i += 1;
                }
                "--unknown-tooltip" => {
                    wayeyes.unknown.tooltip = Some(args[i].clone());
                    i += 1;
                }
                "--unknown-class" => {
                    wayeyes.unknown.class = Some(args[i].clone());
                    i += 1;
                }
                "--unknown-percentage" => {
                    match args[i].parse::<u32>() {
                        Ok(p) => wayeyes.unknown.percentage = p,
                        Err(e) => {
                            return Err(anyhow!(format!(
                                "unknown-percentage parsing error: {}",
                                e
                            )));
                        }
                    }
                    i += 1;
                }
                x => return Err(anyhow!(format!("argument parse error: {}", x))), //TODO error/usage
            }
        }

        Ok(wayeyes)
    }

    async fn run(&self) -> Fallible<()> {
        println!("{}", self.wayeyes_format(None));
        let mut events = Connection::new()
            .await?
            .subscribe([EventType::Window])
            .await?;

        let running = Arc::new(AtomicBool::new(true));
        let r = running.clone();
        ctrlc_async::set_handler(move || {
            r.store(false, Ordering::SeqCst);
        })
        .expect("Error setting Ctrl-C handler");

        // adding the ctrlc check to the while condition would look better, but then rustc complains about let in that position is an unstable feature. Change this later.
        while let Some(Ok(event)) = events.next().await {
            if !running.load(Ordering::SeqCst) {
                break;
            }
            if let Event::Window(w) = event {
                let WindowEvent {
                    change, container, ..
                } = *w;

                if change == WindowChange::Focus {
                    println!("{}", self.wayeyes_format(Some(&container)));
                }
            }
        }
        Ok(())
    }

    fn wayeyes_format(&self, container: Option<&Node>) -> String {
        if let Some(n) = container {
            match n.shell {
                Some(ShellType::XdgShell) => self.xdg.format(&self.format),
                Some(ShellType::Xwayland) => self.xwayland.format(&self.format),
                _ => self.unknown.format(&self.format),
            }
        } else {
            self.unknown.format(&self.format)
        }
    }
}

fn print_usage(exe: &str) {
    println!("{exe:?} wayeyes");
    println!("\t\t\t[--format <format string>]");
    println!("\t\t\t[--xdg-icon <icon for xdg window>]");
    println!("\t\t\t[--xdg-tooltip <tooltip for xdg window>]");
    println!("\t\t\t[--xdg-class <class for xdg window>]");
    println!("\t\t\t[--xdg-percentage <percentage for xdg window>]");
    println!("\t\t\t[--xwayland-icon <icon for xwayland window>]");
    println!("\t\t\t[--xwayland-tooltip <tooltip for xwayland window>]");
    println!("\t\t\t[--xwayland-class <class for xwayland window>]");
    println!("\t\t\t[--xwayland-percentage <percentage for xwayland window>]");
    println!("\t\t\t[--unknown-icon <icon for xwayland window>]");
    println!("\t\t\t[--unknown-tooltip <tooltip for unknown window>]");
    println!("\t\t\t[--unknown-class <class for unknown window>]");
    println!("\t\t\t[--unknown-percentage <percentage for unknown window>]");
}
