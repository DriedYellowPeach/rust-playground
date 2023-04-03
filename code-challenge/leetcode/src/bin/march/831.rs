fn main() {
    unimplemented!();
}

fn mask_phone(chs: &[char]) -> String {
    let country_length = chs.iter().filter(|x| x.is_numeric()).count() - 10;

    // build country code
    let mut out = if country_length == 0 {
        String::new()
    } else {
        format!("+{}-", "*".repeat(country_length))
    };

    // build
    out.push_str("***-***-");

    let mut personal = String::with_capacity(4);
    let mut i = 0;
    for c in chs.iter().rev() {
        if c.is_numeric() {
            personal.push(*c);
            i += 1;
        }

        if i == 4 {
            break;
        }
    }

    for c in personal.chars().rev() {
        out.push(c);
    }

    out
}

fn mask_email(chs: &[char]) -> String {
    let at_position = chs.iter().position(|c| *c == '@').unwrap();
    let mut out = String::new();
    out.push(chs[0].to_ascii_lowercase());
    out.push_str(&"*".repeat(5));
    out.push(chs[at_position - 1].to_ascii_lowercase());
    out.push('@');

    for c in chs.iter().skip(at_position + 1) {
        if c.is_ascii_alphabetic() {
            out.push(c.to_ascii_lowercase());
        } else {
            out.push(*c);
        }
    }

    out
}

fn mask_pii(s: String) -> String {
    let mut chs = s.chars().collect::<Vec<_>>();

    if chs.first().unwrap().is_alphabetic() {
        return mask_email(&chs);
    }

    return mask_phone(&chs);
}

#[test]
fn test_mask_pii() {
    assert_eq!(
        mask_pii("NeiL@microsoft.com".to_string()),
        "n*****l@microsoft.com"
    );
    assert_eq!(mask_pii("1(234)567-890".to_string()), "***-***-7890");
    assert_eq!(mask_pii("+(501321)-50-23431".to_string()), "***-***-7890");
}
