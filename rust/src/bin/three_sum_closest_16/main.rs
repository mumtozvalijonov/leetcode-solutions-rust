use crate::solution::Solution;

mod solution;

#[cfg(test)]
mod tests;

fn main() {
    Solution::three_sum_closest(vec![10, 20, 30, 40, 50, 60, 70, 80, 90], 1);
}
