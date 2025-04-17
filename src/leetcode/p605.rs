#![allow(dead_code)]

struct Solution;

impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        let mut i = 0;

        while i < flowerbed.len() {
            let left = i == 0 || flowerbed[i - 1] == 0;
            let right = i == flowerbed.len() - 1 || flowerbed[i + 1] == 0;
            if left && right && flowerbed[i] == 0 {
                flowerbed[i] = 1;
                n -= 1;
            }

            i += 1;
        }

        n <= 0
    }
}
