### References

Passing arguments as Reference

```rust
fn main()  {
    let a = 15;
    let ref_a = &a;
    print!("{} {} {}", a, *ref_a, ref_a);
    ref_mut();
}
```

> mutability of References  

```rust
fn ref_mut() {
    let mut a: i32 = 10;
    let mut b: i32 = 20;
    let mut p: &mut i32 = &mut a; // line 3

    print!("{} ", *p);
    *p += 1; // line 5
    print!("{} ", *p);
    p = &mut b;
    print!("{} ", *p);
    *p += 1; // line 9
    print!("{} ", *p);
}
```

> at line 5 and line 9  
> objects referred to by p   
> are increamented   
> that is they are readable   
> and writable  
> and for this   
> if *p is mutable  
> therefore in line 3  
> type of p is not just i32   
> instead, &mut i32  


