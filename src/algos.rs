/**
Binary search
 **/

fn binary_search<T: PartialOrd>(a: Vec<T>, target: T) -> Option<i8> {
    let mut left: i8 = 0;
    let mut right: i8 = a.len() as i8 - 1;

    while left <= right {
        let mid: i8 = (left + right)/2;
        if a[mid as usize] == target{
            return Some(mid);
        }
        if a[mid as usize] > target {
            right = mid - 1;
        }
        else{
            left = mid + 1
        }
    }
    return None;
}
