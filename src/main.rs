// Started a new branch in rust practice repo
// started workin on rust struct data type
// it seems like interface of typescript


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

print!("\n");
another_func();
print!("\n");
other_func();
}

// mutabale struct
fn another_func ()  {
    struct SomeData {
        integer: i32,
        fractional: f32,
    }
    let mut data = SomeData {
        integer: 10,
        fractional: 183.19,
    };
    data.fractional = 8.2;
    print!("{}, {}", data.fractional, data.integer);
}

// tuple struct

fn other_func ()  {
   struct SomeData (
       i32,
       f32,
       char,
       [u8; 5],
       ); 
   let data = SomeData (
       10_000_000,
       183.19,
       'Q',
       [9, 0, 250, 60, 200],
       );
   print!("{}, {}, {}, {}", 
          data.2, data.0, data.1, data.3[2]);
}


