// TOPIC 10 — Vec and Iteration
// EXERCISE TYPE: Extend
// =============================================
// Add two functions that use more advanced iterator methods.
//
// ADD:
// 1. fn running_totals(v: &[i32]) -> Vec<i32>
//    Returns a Vec where each element is the cumulative sum up to that position.
//    [1, 2, 3, 4] -> [1, 3, 6, 10]
//    Hint: use .scan() or a fold-based approach
//
// 2. fn flatten_vecs(v: &[Vec<i32>]) -> Vec<i32>
//    Flattens a slice of Vecs into one Vec.
//    [[1,2],[3],[4,5,6]] -> [1, 2, 3, 4, 5, 6]
//    Hint: v.iter().flatten().copied().collect()
//
// Expected output:
//   running totals of [1,2,3,4,5]: [1, 3, 6, 10, 15]
//   running totals of [5,5,5]: [5, 10, 15]
//   flattened: [1, 2, 3, 4, 5, 6]
//
// Run: cargo run --bin extend

fn main() {
    let v = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let sum: i32 = v.iter().sum();
    let evens: Vec<i32> = v.iter().filter(|&&x| x % 2 == 0).copied().collect();
    println!("sum: {}", sum);
    println!("evens: {:?}", evens);

    // TODO: add running_totals and flatten_vecs, then test here
}
