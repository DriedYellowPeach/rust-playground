
#[derive(Default)]
struct Player {
    hp: i32,
    mp: i32,
    name: String,
    hair: String,
    eye: String,
    skin: String,
}


#[test]
fn test_default_populate() {
    let p = Player {
        hp: 100,
        ..Default::default()
    };

    let p2 = Player {
        mp: 20,
        ..Player::default()
    };

    let p3 = Player {
        hp: 100,
        mp: 100,
        name: "Neil".to_string(),
        ..<Player as Default>::default()
    };

    assert_eq!(p.hp, 100);
    assert_eq!(p.mp, 0);
    assert_eq!(p.name, "");
    assert_eq!(p2.hp, 100);
    assert_eq!(p3.name, "Neil");
}

