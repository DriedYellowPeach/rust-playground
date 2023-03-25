fn main() {
    unimplemented!();
}

#[allow(dead_code)]
fn check_arithmetic_subarrays(nums: Vec<i32>, l: Vec<i32>, r: Vec<i32>) -> Vec<bool> {
    let mut out = vec![];
    let mut buffer = vec![0; nums.len()];

    for (left, right) in l.into_iter().zip(r.into_iter()) {
        let mut valid = true;
        let left = left as usize;
        let right = right as usize;
        let seen = &mut buffer[left..=right];
        let great = *nums[left..=right].iter().max().unwrap();
        let small = *nums[left..=right].iter().min().unwrap();

        if (great - small) % (right - left) as i32 != 0 {
            out.push(false);
            continue;
        }
        let gap = (great - small) / (right - left) as i32;
        for (i, num) in nums[left..=right].iter().enumerate() {
            if gap == 0 {
                if num != &small {
                    valid = false;
                    break;
                }
                seen[i] = 1;
                continue;
            }
            if (num - small) % gap != 0 {
                valid = false;
                break;
            }
            let index = (num - small) / gap;
            seen[index as usize] = 1;
        }

        let _x = 1;

        // if gap == 0 {
        //     out.push(valid);
        //     continue;
        // }

        if valid && seen.iter().all(|x| *x == 1) {
            out.push(true);
        } else {
            out.push(false);
        }
        seen.iter_mut().for_each(|x| *x = 0);
    }

    out
}

#[test]
fn test_check_arithmetic_subarrays() {
    // let nums = vec![4, 6, 5, 9, 3, 7];
    // let l = vec![0, 0, 2];
    // let r = vec![2, 3, 5];
    // let out = check_arithmetic_subarrays(nums, l, r);
    // assert_eq!(out, vec![true, false, true]);

    let nums = vec![-3,-6,-8,-4,-2,-8,-6,0,0,0,0];
    let l = vec![5,4,3,2,4,7,6,1,7];
    let r = vec![6,5,6,3,7,10,7,4,10];
    let out = check_arithmetic_subarrays(nums, l, r);
    assert_eq!(out, vec![true,true,true,true,false,true,true,true,true]);
}
