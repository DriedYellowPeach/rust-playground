pub mod distri;
pub mod sorting_vectors;

use rand::{thread_rng, Rng};
//
// there are two high level way to generate random values
fn get_random_values() {
    let mut rng = rand::thread_rng();

    let x = rng.gen::<i32>();
    let y = rng.gen::<bool>();
    let z = rng.gen::<f64>();

    println!("x: {x}, y: {y}, z:{z}");
}

fn get_random_range() {
    let mut rng = rand::thread_rng();
    let mut decimal;
    let mut cnt = [0; 10];
    let upbound = 100000;

    for _ in 0..upbound {
        decimal = rng.gen_range(0..10);
        cnt[decimal as usize] += 1;
    }

    cnt.iter()
        .map(|&x| println!("{:05.2}%", x as f64 / upbound as f64 * 100f64))
        .last();
}

use rand::distributions::{Distribution, Uniform};
use rand_distr::Standard;

fn faster_get_range() {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(0..5);
    let mut cnt = [0; 5];
    let upbound = 100000;

    for _ in 0..upbound {
        cnt[die.sample(&mut rng) as usize] += 1;
    }

    cnt.iter()
        .map(|&x| println!("{:05.2}%", x as f64 / upbound as f64 * 100f64))
        .last();
}

use std::iter::repeat_with;
use std::time::Instant;

fn timer<F>(test_fn: F)
where
    F: Fn(),
{
    let now = Instant::now();
    // do something here
    test_fn();

    println!("{} seconds have elapsed", now.elapsed().as_secs_f64());
}

#[test]
fn test_timer() {
    use std::time::Duration;
    timer(|| std::thread::sleep(Duration::from_secs(2)));
}

// the faster version do execute faster!
#[test]
fn test_get_random_values() {
    // get_random_values();
    println!("============");
    timer(get_random_range);
    println!("============");
    timer(faster_get_range);
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (x, y) = rng.gen();
        Point { x, y }
    }
}

#[test]
fn test_gen_random_point() {
    use rand::thread_rng;
    let mut rng = thread_rng();
    let p1 = rng.gen::<Point>();

    let standard = Standard;
    let p2 = Distribution::<Point>::sample(&standard, &mut rng);

    let p3 = rng.sample::<Point, Standard>(Standard);
    let p4 = rng.sample::<Point, _>(Standard);

    println!("{p1:?}, {p2:?}, {p3:?}, {p4:?}");
}

fn generate_random_passwd() {
    use rand::distributions::Alphanumeric;
    let rand_bytes: Vec<_> = thread_rng().sample_iter(&Alphanumeric).take(40).collect();
    let rand_string = String::from_utf8(rand_bytes).expect("invalid utf-8 encodings");
    println!("{rand_string}");
}

#[test]
fn test_generate_random_passwd() {
    generate_random_passwd();
}

fn get_random_passwd_from_char_set(charset: &[u8], sz: usize) -> String {
    let mut rng = thread_rng();
    let rand_string: String = repeat_with(|| {
        let idx = rng.gen_range(0..charset.len());
        charset[idx] as char
    })
    .take(sz)
    .collect();
    rand_string
}

#[test]
fn test_get_rand_passwd_from_charset() {
    let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                    abcdefghijklmnopqrstuvwxyz\
                    0123456789\
                    )(*&^%$#@!~";

    println!("{}", get_random_passwd_from_char_set(charset, 40));
}
