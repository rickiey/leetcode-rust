mod rob;
pub use  rob::resolve::Solution;
fn main() {
    let c = vec![1, 2, 3, 4, 5];
    let rst = Solution::rob(c);
    println!("result {}", rst);
}
