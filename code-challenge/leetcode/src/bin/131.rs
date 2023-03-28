// type: dp, palindrome
// 
// solution: 
//// double dp, dp a tbl to check if s[i..j] is a palindrome at O(1)
//// then use dp to collect partition method(using slice pair(i, j))
//// f(i) -> all the patition methods of s[0..i]
//// 
//// f(i) = f(0).iter().map( one_method append s[1..i]) IF s[1..i] is a palindrome + 
////      = f(1).iter().map( one_method append s[2..i]) IF s[2..i] is a palindrome + 
////      ...
////      = f(j).iter().map( one_method append s[j+1..j]) IF s[j..i] is a palindrome
////

fn main() {
    println!("partition string into palindrome, and find all the partition solution!");
}

#[allow(dead_code)]
fn get_palindrome_tbl(s: &str) -> Vec<Vec<bool>> {
    let s = s.as_bytes();
    let length = s.len();
    let mut tbl: Vec<Vec<bool>> = vec![vec![true; length]; length];

    for i in (0..length-1).rev() {
        for j in i+1..length {
            tbl[i][j] = (s[i] == s[j]) && (tbl[i+1][j-1]);
        }
    }

    tbl
}

#[test]
fn test_get_palindrome_tbl() {
    let s = "aba";
    println!("{:#?}", get_palindrome_tbl(s));
}

#[allow(dead_code)]
fn partition(s: String) -> Vec<Vec<String>> {
    let tbl = get_palindrome_tbl(&s);
    let s = s.as_bytes();
    type PartSolution = Vec<(usize, usize)>;
    let mut dp: Vec<Vec<PartSolution>> = vec![Vec::new(); s.len()];

    dp[0].push(vec![(0, 0);1]);

    for i in 1..s.len() {
        if tbl[0][i] {
            dp[i].push(vec![(0, i); 1]);
        }
        for j in 0..i {
            if tbl[j+1][i] {
                let (front, back) = dp.split_at_mut(i);
                let (prev, cur) = (&front[j], &mut back[0]);
                for k in prev {
                    let mut sol = k.clone();
                    sol.push((j+1, i));
                    cur.push(sol);
                }
            }
        }
    }

    // println!("{:?}", dp.last());

    // make pairs [(0, 0), (1, 1), (2, 2)] -> ["a", "b", "c"] PartSolutions to Strings
    let pairs_to_string = |prs: &[(usize, usize)]| -> Vec<String> {
        prs.iter().map(
            |pair| {
                std::str::from_utf8(&s[pair.0..pair.1+1])
                .expect("invalid utf-8!").to_string()
            }
        )
        .collect::<Vec<String>>()
    };
    
    // iter over dp last as vec of pairs, [pairs1, pairs2, pairs3] -> [Strings, Strings, Strings]
    dp.last().unwrap().iter().map(
        |prs| {
            pairs_to_string(prs)
        }
    ).collect()
}

#[test]
fn test_partition() {
    let s = "abab".to_string();
    let p = partition(s);
    println!("{:?}", p);
}
