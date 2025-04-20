/// `binary_search` performs a binary search on a sorted array to find the index of a target value
/// a vector also implements a binary_search method on it
pub fn binary_search(arr: &[i32], target: i32) -> i32 {
    let mut left: i32 = 0;
    let mut right: i32 = arr.len() as i32 - 1;

    while left <= right {
        let mid = (left + right) / 2;
        if arr[mid as usize] == target {
            return mid;
        } else {
            match arr[mid as usize] < target {
                true => {
                    left = mid + 1;
                }
                false => {
                    right = mid - 1;
                }
            }
        }
    }

    -1
}

/// `efficient_binary_search` performs a binary search on a sorted array to find the index of a target value
/// This implementation uses a different approach than traditional binary search by using power-of-2 steps
/// to find the target value. This can be more efficient in some cases, particularly for large arrays,
/// as it reduces the number of comparisons needed.
pub fn efficient_binary_search(arr: &[i32], target: i32) -> i32 {
    if arr.is_empty() {
        return -1;
    }

    let mut k = 0;
    let mut b = arr.len() / 2;

    while b >= 1 {
        while k + b < arr.len() && arr[k + b] <= target {
            k += b;
        }
        b /= 2;
    }

    if arr[k] == target {
        k as i32
    } else {
        -1
    }
}

pub fn lower_bound(arr: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left as i32
}

pub fn upper_bound(arr: &[i32], target: i32) -> i32 {
    let mut left = 0;
    let mut right = arr.len();

    while left < right {
        let mid = left + (right - left) / 2;
        if arr[mid] <= target {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(binary_search(&arr, 1), 0);
        assert_eq!(binary_search(&arr, 5), 4);
        assert_eq!(binary_search(&arr, 10), 9);
        assert_eq!(binary_search(&arr, 11), -1);
        assert_eq!(binary_search(&arr, 0), -1);

        let arr = vec![1];
        assert_eq!(binary_search(&arr, 1), 0);
        assert_eq!(binary_search(&arr, 2), -1);

        let arr = vec![];
        assert_eq!(binary_search(&arr, 1), -1);
    }

    #[test]
    fn test_efficient_binary_search() {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        assert_eq!(efficient_binary_search(&arr, 1), 0);
        assert_eq!(efficient_binary_search(&arr, 5), 4);
        assert_eq!(efficient_binary_search(&arr, 10), 9);
        assert_eq!(efficient_binary_search(&arr, 11), -1);
        assert_eq!(efficient_binary_search(&arr, 0), -1);

        let arr = vec![1];
        assert_eq!(efficient_binary_search(&arr, 1), 0);
        assert_eq!(efficient_binary_search(&arr, 2), -1);

        let arr = vec![];
        assert_eq!(efficient_binary_search(&arr, 1), -1);
    }
}
