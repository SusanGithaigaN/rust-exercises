// Read ../../README.md before starting.

fn main() {
    // ERROR 1
    let v = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = v.into_iter().map(|x| x * 2).collect();
    println!("doubled: {:?}", doubled);
    println!("original: {:?}", v);

    // ERROR 2
    let v2 = vec![1, 2, 3, 4, 5, 6];
    let evens: Vec<i32> = v2.iter().filter(|&&x| x % 2 == 0).collect();
    println!("evens: {:?}", evens);

    // ERROR 3
    let mut v3 = vec![1, 2, 3];
    let first = &v3[0];
    for x in v3.iter_mut() {
        *x *= 2;
    }
    println!("first (stale): {}", first);
    println!("v3: {:?}", v3);

    // ERROR 4
    let v4 = vec![1, 2, 3, 4];
    let total = v4.iter().sum();
    println!("sum: {}", total);
}
