// Read ../../README.md before starting.

#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // Create a Rectangle with the given width and height.
    fn new(width: f64, height: f64) -> Rectangle {
        todo!()
    }

    // width × height.
    fn area(&self) -> f64 {
        todo!()
    }

    // 2 × (width + height).
    fn perimeter(&self) -> f64 {
        todo!()
    }

    // True when width == height.
    fn is_square(&self) -> bool {
        todo!()
    }

    // Return a new Rectangle with both dimensions multiplied by factor.
    fn scale(&self, factor: f64) -> Rectangle {
        todo!()
    }
}

fn main() {
    let r = Rectangle::new(4.0, 6.0);
    println!("{:?}", r);            // Rectangle { width: 4.0, height: 6.0 }
    println!("{}", r.area());       // 24
    println!("{}", r.perimeter());  // 20
    println!("{}", r.is_square());  // false
    println!("{:?}", r.scale(2.0)); // Rectangle { width: 8.0, height: 12.0 }

    let sq = Rectangle::new(5.0, 5.0);
    println!("{}", sq.is_square()); // true
}

// Supplied checks; leave these unchanged.
#[cfg(test)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/checks/recall.rs"));
