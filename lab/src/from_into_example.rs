use std::net::Ipv4Addr;

fn text_into_vector() {
    let text = "hello".to_string();
    // let v: Vec<u8> = text.into();
    // let v = <String as Into<Vec<u8>>>::into(text);
    let v = Into::<Vec<u8>>::into(text);
    println!("{:?}", v);
}

fn ping<A>(a: A)
where
    A: Into<Ipv4Addr>,
{
    let addr = a.into();
    println!("ping {}", addr);
}

#[test]
fn test_pring() {
    ping(Ipv4Addr::new(127, 0, 0, 1));
    ping([127, 0, 0, 1]);
    ping(0x7f00_0001);
}
