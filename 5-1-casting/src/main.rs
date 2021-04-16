use std::convert::{TryFrom, TryInto};

fn main() {
    let int: i32 = -1;
    println!("{:?}", int);
    println!("{:?}", int as u8);
    println!("{:?}", u8::try_from(int).ok());
    println!("{:?}", u8::try_from(-1i32).ok());
    println!("{:?}", u8::try_from(0i32).ok());
    println!("{:?}", u8::try_from(1i32).ok());
    println!("{:?}", u8::try_from(1i32).ok().unwrap());
    println!("{:?}", u8::try_from(-1i32).ok().unwrap_or_default());
    println!("{:?}", u8::try_from(255i32).ok().unwrap_or_default());
    // println!("{:?}", u8::try_from(256i32).ok().unwrap_or_else(|i| {
    //     if i < u8::MIN as i32 {
    //         u8::MIN
    //     } else {
    //         u8::MAX
    //     }
    // }));
    println!("{:?}", u8::MAX.saturating_add(1u8));
    println!("{:?}", 255i32.saturating_add(1i32));
    println!("{:?}", saturating_cast(-1i32));
    println!("{:?}", saturating_cast(0i32));
    println!("{:?}", saturating_cast(1i32));
    println!("{:?}", saturating_cast(254i32));
    println!("{:?}", saturating_cast(255i32));
    println!("{:?}", saturating_cast(256i32));
}

fn saturating_cast(x: i32) -> u8 {
    if let Some(i) = u8::try_from(x).ok() {
        i
    } else if x < u8::MIN as i32 {
        u8::MIN
    } else {
        u8::MAX
    }
}
