use args_helper::ExecutableArgs;
use futures_util::stream::StreamExt;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use swayipc_async::{
    Connection, Event, EventType, Fallible, Node, ShellType, WindowChange, WindowEvent,
};

#[async_std::main]
async fn main() {
    let args = ExecutableArgs::new();

    if !args.has_executable() {
        eprintln!("executable not valid");
        std::process::exit(1);
    }

    match args.command.as_deref().unwrap_or_default() {
        "wayeyes" => {
            if args.args_count() > 0 {
                eprintln!("invalid arguments for wayeyes");
                print_usage(&args);
                std::process::exit(3);
            }

            if let Err(e) = run_wayeyes().await {
                eprintln!("{:?}", e);
                std::process::exit(2);
            }
        }
        _ => {
            print_usage(&args);
        }
    }
}

async fn run_wayeyes() -> Fallible<()> {
    wayeyes_format(None);
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
                wayeyes_format(Some(&container));
            }
        }
    }
    Ok(())
}

// TODO: I suppose this could do without hard coded stuff
fn wayeyes_format(container: Option<&Node>) {
    let mut output = "";
    let mut tooltip = "unknown";
    let mut class = "unknown";
    let mut percentage = 0;
    if let Some(n) = container {
        match n.shell {
            Some(ShellType::XdgShell) => {
                output = "";
                tooltip = "wayland native";
                class = "wayland";
                percentage = 100;
            }
            Some(ShellType::Xwayland) => {
                output = "";
                tooltip = "xwayland";
                class = "xwayland";
                percentage = 50;
            }
            _ => {
                output = "";
                tooltip = "unknown";
                class = "unknown";
                percentage = 0;
            }
        }
    }
    println!(
        "{{\"text\" : \"{}\", \"tooltip\" : \"{}\", \"class\" : \"{}\", \"percentage\" : \"{}\" }}",
        output, tooltip, class, percentage
    )
}

fn print_usage(args: &ExecutableArgs) {
    let exe = args.executable.as_deref().unwrap_or_default();
    println!("{:?} wayeyes", exe);
}
