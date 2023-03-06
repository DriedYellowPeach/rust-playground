use nix::sys::signal;
use nix::unistd;

use std::thread;
use std::time;

fn main() {
    thread::sleep(time::Duration::from_secs(3));
    unsafe { signal::signal(signal::Signal::SIGTTOU, signal::SigHandler::SigIgn) }.unwrap();
    unistd::tcsetpgrp(0, unistd::getpgrp()).unwrap();
    loop {
        let pid = unistd::tcgetpgrp(0).unwrap();
        println!("foreground process group is: {pid}");

        let my_grp = unistd::getpgrp();
        println!("my process group is: {my_grp}");

        if pid != my_grp {
            unistd::tcsetpgrp(0, my_grp).unwrap();
        }
        thread::sleep(time::Duration::from_secs(1));
    }
}
