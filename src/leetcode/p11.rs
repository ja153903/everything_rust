#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut area = 0;
        let mut left_itr = height.iter().enumerate().peekable();
        let mut right_itr = height.iter().enumerate().rev().peekable();

        while let (Some(&(left_idx, &left_height)), Some(&(right_idx, &right_height))) =
            (left_itr.peek(), right_itr.peek())
        {
            if right_idx <= left_idx {
                break;
            }

            area = area.max((right_idx - left_idx) as i32 * left_height.min(right_height));
            if left_height < right_height {
                left_itr.next();
            } else {
                right_itr.next();
            }
        }

        area
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_area() {
        assert_eq!(Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
        assert_eq!(Solution::max_area(vec![1, 1]), 1);
        assert_eq!(Solution::max_area(vec![4, 3, 2, 1, 4]), 16);
        assert_eq!(Solution::max_area(vec![1, 2, 1]), 2);
    }
}
