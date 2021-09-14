### References

this function is good  
but copies the array twice  
which has computational cost  

```rust
fn main() {
    print!("{:?}", divide(140, 9));

    let mut arr = [5, -4, 9, 0, -7, -1, 3, 5, 3, 1];
    arr = double_negatives(arr); // value of returned array is assigned to arr variable
    print!(" {:?} ", arr);
}

```

instead arguments can be passed as arguments  


```rust
// passing arguments as reference

fn main()  {
   let mut arr = [ 3, -5, 7, 1, -7, -1, 3, 4, 4, 0];
   
   // it returns [3, -10, 7, 1, -14, -2, 3, 4, 4, 0]
   
   double_negatives(&mut arr);
   print!("{:?}", arr);
}

fn double_negatives(a: &mut [i32; 10]) {
    for i in 0..10 {
        if (*a)[i] < 0 { (*a)[i] *= 2; }
    }
}
```

**&** is used for referencing variable  
**' * '**, aestrick is used to refer object on the reference

or, it can be written like this

```rust

fn main()  {
   let mut arr = [ 3, -5, 7, 1, -7, -1, 3, 4, 4, 0];
   double_negatives(&mut arr);
   print!("{:?}", arr);
}

fn double_negatives(a: &mut [i32; 10]) {
    for i in 0..10 {
        if a[i] < 0 { a[i] *= 2; }
    }
}
```
