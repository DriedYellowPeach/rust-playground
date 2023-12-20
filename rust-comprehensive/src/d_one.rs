use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::sync::Mutex;

static mut USERS: Option<&'static HashMap<usize, &'static str>> = None;

fn get_or_init() -> &'static HashMap<usize, &'static str> {
    unsafe {
        match USERS {
            Some(u) => u,
            None => {
                let u = Box::leak(Box::new(
                    vec![(1, "kevin"), (2, "neil"), (3, "peach"), (4, "hello")]
                        .into_iter()
                        .collect::<HashMap<_, _>>(),
                ));
                USERS = Some(u);
                u
            }
        }
    }
}

static USERS2: Mutex<Option<&'static HashMap<usize, &'static str>>> = Mutex::new(None);

fn get_or_init2() -> &'static HashMap<usize, &'static str> {
    let mut u = USERS2.lock().unwrap();
    match &*u {
        Some(x) => x,
        None => {
            let res = Box::leak(Box::new(
                vec![(1, "kevin"), (2, "neil"), (3, "peach"), (4, "hello")]
                    .into_iter()
                    .collect::<HashMap<_, _>>(),
            ));
            *u = Some(res);
            res
        }
    }
}

#[test]
fn test_my_users() {
    let users = get_or_init();
    for id in 0..5 {
        match users.get(&id) {
            Some(name) => println!("user: {name} of uid {id}"),
            None => println!("no such users of uid {id}"),
        }
    }
}

fn collatz_conjecture() {
    let mut rng = thread_rng();
    // let mut x = rng.gen::<u32>();
    let mut x = rng.gen_range(1..1000);
    print!("{x}");

    while x != 1 {
        if x % 2 == 0 {
            x /= 2;
        } else {
            x = 3 * x + 1;
        }

        print!("-> {x}");
    }
    println!();
}

// lots of fmt: rustup doc std::fmt
// basic: place holder {}
// debug output {:?}
// place holder using variable
// place holder using key/value
// doulbe place holder
// width and filling
// beutiful output

#[test]
fn format() {
    println!("hello {}!", 32);
    println!("{:?}", (1, (2, 3)));
    println!("{:#?}", (1, (2, 3)));
    let x = (3, 4);
    println!("{x:?}");
    println!("value: {value}", value = 4);
    println!("{} {}", x.0, x.1);
    println!("{:08}", x.0);
}

#[test]
fn test_collatz() {
    collatz_conjecture();
}

// raw string r"", don't need to escape many things
// raw string r##, allow you embed even double quote
#[test]
fn test_raw_strint() {
    println!(r"not new line\n");
    println!(r#"hello, this is "world""#);
}

fn fizzbuzz(n: i32) -> String {
    if n % 15 == 0 {
        return "fizzbuzz".to_string();
    }

    if n % 3 == 0 {
        return "fizz".to_string();
    }

    if n % 5 == 0 {
        return "buzz".to_string();
    }

    n.to_string()
}

#[test]
fn test_fzbz() {
    for i in 1..20 {
        println!("{}", fizzbuzz(i));
    }
}

fn pickone<T>(a: T, b: T) -> T {
    if std::process::id() % 2 == 0 {
        a
    } else {
        b
    }
}

#[test]
fn test_pickone() {
    println!("{}", pickone("apple", "banana"));
    println!("{}", pickone(&1, &2));
}

fn multiply(x: i16, y: i16) -> i16 {
    x * y
}

#[test]
fn test_multiply() {
    let x: i8 = 15;
    let y: i16 = 4;

    println!("{}", multiply(x.into(), y));
    println!("{}", multiply(i16::from(x), y));
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut ret = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            ret[i][j] = matrix[j][i];
        }
    }
    ret
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    for r in matrix {
        for c in r {
            print!("{c}, ");
        }
        println!();
    }
}

use std::fmt::Debug;

// AsRef is a convertion that &T -> &U
// [T;N] impl AsRef<[T]>, which means &[T;N] -> &[T]
// Vector impl AsRef<[T]> which means &Vector -> &[T]
fn pretty_print_v2<T, Line, Matrix>(matrix: Matrix)
where
    T: Debug,
    Line: AsRef<[T]>,
    Matrix: AsRef<[Line]>,
{
    for r in matrix.as_ref() {
        for c in r.as_ref() {
            print!("{c:?}, ");
        }
    }
}

#[test]
fn test_matrix_func() {
    let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    println!("{:?}", matrix);
    // println!("{:?}", transponse(matrix));
    pretty_print(&matrix);
    pretty_print(&transpose(matrix));
}

#[test]
fn test_loop_label() {
    let v = [10, 20, 30];
    let mut vit = v.iter();

    'outer: while let Some(&x) = vit.next() {
        let mut i = 0;
        while i < x {
            println!("x: {x}, i: {i}");
            i += 1;
            if i == 3 {
                break 'outer;
            }
        }
    }
}

// const will be inlined, while static do in the memory when the program is running
// const function is runned at compile time, the line between compiler and run time is blurred
// for static, the variable lives for the entire liftime of the program
const fn toy_square(x: i32) -> i32 {
    x * x
}

fn give_const_value() {
    const NUMBER: i32 = 5;
    const SQUARE: i32 = toy_square(5);
}

// usage of shadow, very useful for data that in Result<T>
#[test]
fn shadow() {
    let a = Result::<i32, &'static str>::Ok(5);
    let a = a.unwrap();
    println!("hello {a}");
}

#[derive(Debug)]
enum CoinFlip {
    Heads,
    Tails,
}

fn flip_coin() -> CoinFlip {
    let random_number = 5;
    if random_number % 2 == 0 {
        CoinFlip::Heads
    } else {
        CoinFlip::Tails
    }
}

// enum inline struct, and I will try to impl for it
// inline struct is different, it's still a variant, you can impl method for it, you can impl trait
// on it
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i32, y: i32 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unload"),
        WebEvent::KeyPress(ch) => println!("pressed '{}'.", ch),
        WebEvent::Paste(s) => println!("paste {s}"),
        WebEvent::Click { x, y } => println!("click {x},{y}"),
    }
}

// use of std::any::type_name, std::mem::{ size_of, aligh_of }
// both these function are run at compile time
use std::any::type_name;
use std::mem::{align_of, size_of};

fn dbg_size<T>() {
    println!(
        "{}: size {}, align {}",
        type_name::<T>(),
        size_of::<T>(),
        align_of::<T>()
    );
}

// we see that enum Foo takes up just one byte
enum Foo {
    A,
    B,
}

// later I will use macro to create a enum have more 256 variant
enum Bar {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
}

#[repr(u32)]
enum Fuzz {
    A,
    B = 10000,
    C,
}

enum Fizz {
    A(i32),
    B(i32),
}

// size is 2 bytes, one byte for the variant tag and one byte for the bool
enum MyBool {
    TRUE(bool),
    FALSE(bool),
}

// However, some times, there is optimization
// i32 takes up 4 bytes, while Option<i32> takes up 8 bytes, 4 bytes aer used to store variable tag
// bool takes up 1 bytes, while MyBool takes up 2 bytes, 1 byte are used to store variant tag
// Optin<i64> will take up 16 bytes.
// However, some pointer type don't need space to store variant tag, Option is such an example
// when types are chain together, they won't grow any larger, for example Some(Some(true))
// when there are many used bits, rust will try to pick them up

#[test]
fn test_dbg_size() {
    dbg_size::<Foo>();
    dbg_size::<Bar>();
    dbg_size::<Fuzz>();
    dbg_size::<Fizz>();
    dbg_size::<MyBool>();
    dbg_size::<Option<bool>>();
    dbg_size::<Option<&i32>>();
    dbg_size::<Option<i32>>();
    dbg_size::<Option<u32>>();
    dbg_size::<Option<i64>>();
    println! {"A: {}, B: {}, C:: {}", Fuzz::A as u32, Fuzz::B as u32, Fuzz::C as u32};
}

use std::mem::transmute;

macro_rules! dbg_bits {
    ($e: expr, $bit_type: ty) => {
        println!("- {}: {:#x}", stringify!($e), transmute::<_, $bit_type>($e))
    };
}

// use of if let with Option, Result
#[test]
fn test_if_let() {
    let r: Result<_, &'static str> = Ok(5);
    if let Ok(v) = r {
        println!("ok({v})");
    } else {
        println!("some error may happen");
    }

    let err: Result<i32, i32> = Err(-1);
    if let Err(e) = err {
        println!("error code: {e}");
    }
}

// let else allow you to do c style early return
// when using with Option, very suitable, you can't do any with None
// when using with Result, becareful that this early return can't print error information
// to get error explicitly, you need to use match
#[test]
fn test_let_else() {
    let mut it = "hello world my friend".split(' ');
    let Some(item) = it.next() else {
        println!("no item");
        return;
    };

    println!("item: {item}");

    // let else also support complext pattern match
    let (Some(_), Some(_)) = (it.next(), it.next()) else {
        println!("there are not many items left!");
        return;
    };
}

// while let and loop break
#[test]
fn test_while_let() {
    let v = [1, 2, 3, 4, 5];
    let mut vit = v.iter();
    while let Some(x) = vit.next() {
        println!("x: {x}");
    }

    vit = v.iter();
    loop {
        let Some(x) = vit.next() else {
            break;
        };
        println!("x: {x}");
    }
}

// test Option as_deref
#[test]
fn test_as_deref() {
    let smstr = Some("hello".to_owned());

    match smstr.as_deref() {
        Some("hello") => println!("hello back"),
        Some("Sad") => println!("cheer up"),
        None => println!("say something"),
        _ => println!("Have a nice day, stranger!"),
    }
}

struct DoublePairs {
    x: (i32, i32),
    y: (i32, i32),
}

// match a struct
// .. can expand to anything you want
#[test]
fn test_match_a_struct() {
    let dp = DoublePairs {
        x: (1, 2),
        y: (3, 4),
    };
    match dp {
        DoublePairs {
            x: (1, b),
            y: (3, d),
        } => println!("first {b} {d}"),
        DoublePairs {
            x,
            ..
        } => println!("first {x:?}"),
    }
}

// destruct array working with array of fixed width and slice with unknow length
// double period .. will match a sub array, and a@.. will copy the array into a
#[test]
fn test_match_an_array() {
    let arr = [1, 2, 3, 4, 5];
    let slc = &arr[1..];

    match arr {
        [1, ..] => println!("first item is 1"),
        [2, ..] => println!("first item is 2"),
        _ => println!("other"),
    }

    match slc {
        [1] => println!("1"),
        [2, 3] => println!("2 3"),
        [2, 3, 4, 5] => println!("2 3 4 5"),
        _ => println!("I don't konw"),
    }

    match arr {
        [a@.., b, c] => println!("{a:?}, {b}, {c}"),
        _ => println!("other arr"),
    }

    match slc {
        [a@.., b, c] => println!("{a:?}, {b}, {c}"),
        _ => println!("other slc"),
    }
}

// match guard
//
// this function is intentionaly buggy!
fn is_longer(x: &str, y: &str) -> bool {
    x.len() - y.len()  > 0
}

// unsigned vulnerabilities
#[test]
fn test_unsign_vulnerable() {
    assert!(!is_longer("hello", "hello world"));
}
