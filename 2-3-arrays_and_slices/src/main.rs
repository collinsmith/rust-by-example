use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("first element of the slice: {}", slice[0]);
    println!("the slice has {} elements", slice.len());
}

fn main() {
    let xs = [1, 2, 3, 4, 5];
    let ys = [0; 500]; // 500 elements init as 0

    println!("array occupies {} bytes", mem::size_of_val(&xs));
    println!("array occupies {} bytes", mem::size_of_val(&ys));

    println!("borrow {:?} as a slice", xs);
    analyze_slice(&xs);

    println!("borrow a section of an array as a slice");
    analyze_slice(&ys[1..4]);

    // println!("{}", xs[5]);
}
