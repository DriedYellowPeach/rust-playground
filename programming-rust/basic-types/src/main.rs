fn main() {
    println!("Hello, world!");
}

// mut for variables:
// in rust, symbols representing values on the stack never change the address
// so mut is to describe symbols and the block on the stack.
// by default, you can't change a single bit in that block
// with mut, you can change in that block, either by reassign which means move, or by invoke method.
#[test]
fn test_mut() {
    let mut x = 5;
    let y = 10;

    //y = x; /* can't  assign twice to immutable variable*/
}

#[test]
fn test_integer_types() {
    // literal with suffix
    let _a = -1i8;
    let _b = -1i16;
    let _c = -1i32;
    let _d = -1i64;
    let _e = -1isize;
         
    let _f = 1u8;
    let _g = 1u16;
    let _h = 1u32;
    let _i = 1u64;
    let _j = 1usize;

    // overflow and wrapping add
    let big_val = std::i32::MAX;
    // let x = big_val + 1; // error when test
    let y = big_val.wrapping_add(1);
    assert_eq!(y, std::i32::MIN);

    // underscore
    let _birthday = 1999_09_21;
    let _mask = 0b1010_1010_u8;

    // byte literal
    let _quote = b'\'';

    // convert
    assert_eq!(10_i8 as u16, 10_u16); // in range
    assert_eq!(-1_i16 as i32, 0xffff_ffff_u32 as i32); // sign-extended
    assert_eq!(65535_u16 as i32, 0xffff_i32); // zero-extended

    // method for integers
    assert_eq!(2u16.pow(4), 16);
    assert_eq!((-4i32).abs(), 4);
    assert_eq!(0b1010_1010_u8.count_ones(), 4);

}

#[test]
fn test_float_point_types() {
    // float literals
    let _a = -1.256;
    let _b = 2.;
    let _c = 1e4;
    let _d = 40f32;
    let _e = 19.99_09_21;

    let f = -5.678e3;
    assert_eq!(f, -5_678_f64);

    // constans
    assert_eq!(-1./0., std::f64::NEG_INFINITY);
    assert_eq!(1./0., std::f64::INFINITY);
    //assert_eq!(0./0., std::f64::NAN);

    // methods
    assert_eq!(5f32.sqrt() * 5f32.sqrt(), 5f32);
}

#[test]
fn test_split_at() {
    // split_at -> (&str, &str)
    let (head, tail) = "hello world".split_at(5);
    assert_eq!(head, "hello");
    assert_eq!(tail, " world");
}

// everywhere comma used, rust support trailing comma
#[test]
fn test_extral_trailing_comma() {
    // in struct definition
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32, // here is a trailing comma
    }

    let p = Point {x: 1, y: 2,};
    assert_eq!(p.x + p.y, 3);
    
    // in enum definition
    enum Score {
        TwoPoint(Point),
        ThreePoint(Point),
        AndOne,
        FreeThrow,
    }

    let shot = Score::ThreePoint(Point{x: 5, y: 5});
    
    match shot {
        Score::TwoPoint(p) => {
            println!("get two point from {:?}", p);
        },
        Score::ThreePoint(p) => {
            println!("get three point from {:?}", p);
        },
        _ => {
            println!("get one point");
        }
    }
   
    // in arrary initialization and function argument
    let mut arr = [1, 2, 3, 4,];
    arr.swap(1, 2,);
}
