use std::{io, io::Write, thread, time};

// visuliser could be spinner, progress bar, dot etc
fn visualizer(pattern: &str) -> impl Iterator<Item = String> + '_ {
    pattern.chars().map(String::from).cycle()
}

// percent should be a trait that let worker report current progress
fn percent(progress: &[usize]) -> impl Iterator<Item = usize> + '_ {
    progress.iter().copied().cycle()
}

fn backspace(cnt: usize) -> String {
    (0..cnt).map(|_| 8u8 as char).collect()
}

fn main() {
    let pattern = visualizer(r"-\|/");
    let progress = percent(&[1, 0, 2, 1, 0, 1]);
    let mut sum = 0;

    for (v, p) in pattern.zip(progress) {
        sum += p;
        let buf = format!("{} {}%", v, sum);

        print!("{buf}");
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(100));
        print!("{}", backspace(buf.len()));
    }
}

#[allow(dead_code)]
fn spin() {
    // let backspace = 8u8 as char;
    let spiner = r"-\|/".chars().cycle();
    let dots = "....".to_string();
    for (i, ch) in spiner.enumerate() {
        // let buf = format!("{}{}", ch, &dots[..i%dots.len()+1]);
        let buf = format!("{}{}", ch, &dots[..i % dots.len() + 1]);
        print!("{buf}");
        io::stdout().flush().unwrap();
        thread::sleep(time::Duration::from_millis(1000));
        print!("{}", backspace(buf.len()));
    }
}

#[allow(dead_code)]
fn impl_lifetime<'a>(pattern: &'a str, disturb: &str) -> impl Iterator<Item = char> + 'a {
    if disturb == "endless" {
        println!("log");
    }

    pattern.chars().cycle()
}
