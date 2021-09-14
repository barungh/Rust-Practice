### Rust Functions

In rust value of a function is  
value of its body,    
and body of a function is  
a block  
```rust
{
    a = 34/2; // will return 17
    b = 34/3; // will return 11
    // so here b = 11 will be returned by the funciton
}

```
### Few more explanations  

```rust
// In Rust, it is not necessary to write 'return'
// last expression/statement is return by function
// return can be used to return prematurely
// *****************************************

fn main() {
    print!("{:?}", divide(140, 9));

    let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    arr = double_negatives(arr); // value of returned array is assigned to arr variable
    print!(" {:?} ", arr);
}

// FUNCTION 1 ---> it returns the qutient and remainder
fn divide(dividend: i32, divisor: i32) -> (i32, i32) {
        (dividend / divisor, dividend % divisor)
    }
// dividend / divisor ->  gives us quotient 
// dividend % divisor -> this gives us remainder

// FUNCTION 2 ---> it accepts an array as an argument
// and returns a new array, where 
// all negative integers are dobuled
fn double_negatives(mut a: [i32; 10]) -> [i32; 10] {
    for i in 0..10 {
        if a[i] < 0 { a[i] *= 2; }
    }
    a // it returns the modified array
}
```
