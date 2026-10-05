// TOPIC 10 — Vec and Iteration
// EXERCISE TYPE: Repair
// =============================================
// Fix the iterator and Vec errors.
//
// Run: cargo run --bin repair

fn main() {
    // ERROR 1: v is consumed by into_iter(), then used again
    let v = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = v.into_iter().map(|x| x * 2).collect();
    println!("doubled: {:?}", doubled);
    println!("original: {:?}", v);

    // ERROR 2: type mismatch — filter produces &i32 but collect expects i32
    let v2 = vec![1, 2, 3, 4, 5, 6];
    let evens: Vec<i32> = v2.iter().filter(|&&x| x % 2 == 0).collect();
    println!("evens: {:?}", evens);

    // ERROR 3: iter_mut() borrows v3 mutably, but v3 is also read in the same scope
    let mut v3 = vec![1, 2, 3];
    let first = &v3[0];
    for x in v3.iter_mut() {
        *x *= 2;
    }
    println!("first (stale): {}", first);
    println!("v3: {:?}", v3);

    // ERROR 4: .sum() without a type annotation — Rust can't infer the type
    let v4 = vec![1, 2, 3, 4];
    let total = v4.iter().sum();
    println!("sum: {}", total);
}
