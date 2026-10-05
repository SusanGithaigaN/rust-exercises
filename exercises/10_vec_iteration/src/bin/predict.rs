// TOPIC 10 — Vec and Iteration
// EXERCISE TYPE: Predict
// =============================================
// BEFORE running, write down the output of each println.
//
// Questions to answer first:
//   1. What is the type of `evens`? Why does it contain &i32, not i32?
//   2. After v2.into_iter()... can you still use v2? Why not?
//   3. What is the difference between iter() and into_iter()?
//   4. What is the difference between &v and v.iter()?
//
// Run: cargo run --bin predict

fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let doubled: Vec<i32> = v.iter().map(|x| x * 2).collect();
    println!("{:?}", doubled);

    let evens: Vec<&i32> = v.iter().filter(|x| *x % 2 == 0).collect();
    println!("{:?}", evens);

    let sum: i32 = v.iter().sum();
    println!("sum: {}", sum);

    // v is still usable because iter() borrows
    println!("v still here: {:?}", v);

    // into_iter() consumes the Vec
    let v2 = vec![10, 20, 30];
    let doubled2: Vec<i32> = v2.into_iter().map(|x| x * 2).collect();
    println!("{:?}", doubled2);
    // println!("{:?}", v2); // uncomment to see what happens

    // chained iterators
    let result: Vec<i32> = vec![1, 2, 3, 4, 5, 6]
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();
    println!("{:?}", result);
}
