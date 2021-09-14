// In Rust, it is not necessary to write 'return'
// last expression/statement is return by function
// return can be used to return prematurely
// *****************************************
// see README of this branch

fn main() {
    print!("{:?}", divide(140, 9));

    let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    arr = double_negatives(arr); // value of returned array is assigned to arr variable
    print!(" {:?} ", arr);
}

    fn divide(dividend: i32, divisor: i32) -> (i32, i32) {
        (dividend / divisor, dividend % divisor)
    }
// dividend / divisor ->  gives us quotient 
// dividend % divisor -> this gives us remainder

fn double_negatives(mut a: [i32; 10]) -> [i32; 10] {
    for i in 0..10 {
        if a[i] < 0 { a[i] *= 2; }
    }
    a // it returns the modified array
}


