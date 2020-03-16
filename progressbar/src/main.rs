use log::{info, warn};

fn main() {
    env_logger::init();
    info!("starting up!");
    let pb = indicatif::ProgressBar::new(100);
    warn!("oops");
    for i in 0..100 {
        pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}
