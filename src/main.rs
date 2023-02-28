use notify::{RecursiveMode, Result, Watcher};
use std::path::Path;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel();
    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap();
    let runtime = std::sync::Arc::new(runtime);

    println!("start chating in background...");
    let chat_task = runtime.spawn(chat());

    println!("start watching...");
    if let Err(err) = watch(runtime, rx) {
        println!("Failed to watch. error: {err}");
    }

    chat_task.abort();
    println!("stopped.");
}

async fn chat() {
    loop {
        println!("I am chating!");
        tokio::time::sleep(std::time::Duration::from_secs(4)).await;
    }
}

async fn smile() {
    println!("I am smilling.");
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}

async fn cry() {
    println!("I am crying.");
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
}

fn watch(
    runtime: std::sync::Arc<tokio::runtime::Runtime>,
    rx: std::sync::mpsc::Receiver<()>,
) -> Result<()> {
    // Automatically select the best implementation for your platform.
    let mut watcher = notify::recommended_watcher(move |res| match res {
        Ok(event) => {
            println!("event: {:?}", event);
            runtime.block_on(smile());
            runtime.block_on(cry());
        }
        Err(e) => println!("watch error: {:?}", e),
    })?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(Path::new("."), RecursiveMode::Recursive)?;

    // Hold current thread until Ctrl-C.
    rx.recv().expect("Could not receive from channel.");

    Ok(())
}
