use std::mem;

fn main() {
    println!("hello 1233");
}

pub fn remove_subfolders(folder: Vec<String>) -> Vec<String> {
    if folder.len() <= 1 {
        return folder;
    }

    let mut folder = folder;
    let mut to_deleted = vec![false; folder.len()];
    let mut final_folder = vec![];
    folder.sort_by_key(|a| a.len());

    for i in 0..folder.len() - 1 {
        let mut del_cnt = 0;
        for j in i + 1..folder.len() {
            if to_deleted[j] {
                del_cnt += 1;
                continue;
            }
            if folder[i].len() == folder[j].len() {
                continue;
            }
            if folder[j].starts_with(&folder[i]) && folder[j].as_bytes()[folder[i].len()] == b'/' {
                to_deleted[j] = true;
                del_cnt += 1;
            }
        }
        if del_cnt + i == folder.len() - 1 {
            break;
        }
    }

    for i in 0..folder.len() {
        if to_deleted[i] {
            continue;
        }
        let keep = mem::take(&mut folder[i]);
        final_folder.push(keep);
    }

    final_folder
}

pub fn remove_subfolders_v2(folder: Vec<String>) -> Vec<String> {
    let mut folder = folder;
    let mut root_idx: Vec<usize> = vec![];
    folder.sort();

    root_idx.push(0);
    for i in 1..folder.len() {
        let last = &folder[*root_idx.last().unwrap()];
        let cur = &folder[i];
        if last.len() < cur.len() && cur.starts_with(last) && cur.as_bytes()[last.len()] == b'/' {
            continue;
        } 
        root_idx.push(i);
    }

    let mut final_folder = vec![String::new();root_idx.len()];
    for (i, &idx) in root_idx.iter().enumerate() {
        std::mem::swap(&mut final_folder[i], &mut folder[idx]);
    }

    final_folder
}

pub fn remove_subfolders_v3(folder: Vec<String>) -> Vec<String> {
    let mut folder = folder;
    let mut root_idx: Vec<usize> = vec![];
    folder.sort();

    root_idx.push(0);
    for i in 1..folder.len() {
        let last = &folder[*root_idx.last().unwrap()];
        let cur = &folder[i];
        if last.len() < cur.len() && cur.starts_with(last) && cur.as_bytes()[last.len()] == b'/' {
            continue;
        } 
        root_idx.push(i);
    }

    // let mut final_folder = vec![String::new();root_idx.len()];
    for (i, &idx) in root_idx.iter().enumerate() {
        if i != idx {
            folder.swap(i, idx);
        }
    }

    folder.truncate(root_idx.len());
    folder
}

#[test]
fn test_remove_subfolders() {
    let to_folders = |s: &str| -> Vec<String> { s.split(' ').map(|s| s.to_string()).collect() };

    let input = "/a /a/b /c/d /c/d/e /c/f";
    let output = "/a /c/d /c/f";
    assert_eq!(remove_subfolders(to_folders(input)), to_folders(output));

    let input = "/a /a/b/c /a/b/d";
    let output = "/a";
    assert_eq!(remove_subfolders(to_folders(input)), to_folders(output));

    let input = "/a/b/c /a/b/ca /a/b/d";
    let output = "/a/b/c /a/b/ca /a/b/d";
    assert_eq!(remove_subfolders(to_folders(input)), to_folders(output));
}

#[test]
fn test_remove_subfolders_v2() {
    let to_folders = |s: &str| -> Vec<String> { s.split(' ').map(|s| s.to_string()).collect() };

    let input = "/a /a/b /c/d /c/d/e /c/f";
    let output = "/a /c/d /c/f";
    assert_eq!(remove_subfolders_v2(to_folders(input)), to_folders(output));

    let input = "/a /a/b/c /a/b/d";
    let output = "/a";
    assert_eq!(remove_subfolders_v2(to_folders(input)), to_folders(output));

    let input = "/a/b/c /a/b/ca /a/b/d";
    let output = "/a/b/c /a/b/ca /a/b/d";
    assert_eq!(remove_subfolders_v2(to_folders(input)), to_folders(output));
}

#[test]
fn test_remove_subfolders_v3() {
    let to_folders = |s: &str| -> Vec<String> { s.split(' ').map(|s| s.to_string()).collect() };

    let input = "/a /a/b /c/d /c/d/e /c/f";
    let output = "/a /c/d /c/f";
    assert_eq!(remove_subfolders_v3(to_folders(input)), to_folders(output));

    let input = "/a /a/b/c /a/b/d";
    let output = "/a";
    assert_eq!(remove_subfolders_v3(to_folders(input)), to_folders(output));

    // let input = "/a/b/c /a/b/ca /a/b/d";
    // let output = "/a/b/c /a/b/ca /a/b/d";
    // assert_eq!(remove_subfolders_v3(to_folders(input)), to_folders(output));
}
