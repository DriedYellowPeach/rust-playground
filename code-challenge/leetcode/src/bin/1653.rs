
fn main() {

}

fn min_deletions(s: String) -> i32  {
    // a,b stat
    let na = s.bytes().filter(|&x| x == b'a').count();

    // init some var
    let mut ca = 0;
    let mut cb = 0;
    let mut delete_cnt = na;
    let mut min_delete = delete_cnt;

    for bt in s.bytes() {
        if bt == b'a' {
            ca += 1;
        } else {
            cb += 1;
        }

        delete_cnt = na - ca + cb;
        min_delete = std::cmp::min(min_delete, delete_cnt);
    }

    min_delete as i32
}

#[test]
fn test_min_deletions() {
    let s = "aababbab".to_string();
    assert_eq!(min_deletions(s), 2);

    let s = "bbaaaaabb".to_string();
    assert_eq!(min_deletions(s), 2);
}
