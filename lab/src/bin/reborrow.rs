

fn main() {
    let mut s = "hello".to_string();
    let a = &mut s;
    let b: &mut String = a;
    // a.push_str("world");
    // b.push_str("!");
}


// fn last_or_push<'a>(vec: &'a mut Vec<String>) -> &'a String {

//     if let Some(s) = vec.last() { // borrows vec
//         // returning s here forces vec to be borrowed
//         // for the rest of the function, even though it
//         // shouldn't have to be
//         return s; 
//     }
//     
//     // Because vec is borrowed, this call to vec.push gives
//     // an error!
//     vec.push("".to_string()); // ERROR
//     vec.last().unwrap()
// }
