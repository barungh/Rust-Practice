fn main() {
    
struct SomeData {
    i: i32,
    f: f32,
    c: char,
    five_bytes: [u8; 5],
}

let data = SomeData {
    i: 10_000_000,
    f: 134.23,
    c: 'T',
    five_bytes: [9, 0, 3, 43, 130],
};
print!("{} {} {} {}", data.five_bytes[3], data.i,
       data.f, data.c);
}



