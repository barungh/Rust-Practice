// Passing arguments as Reference

fn main()  {
   let mut arr = [ 3, -5, 7, 1, -7, -1, 3, 4, 4, 0];
   
   // it returns [3, -10, 7, 1, -14, -2, 3, 4, 4, 0]
   
   double_negatives(&mut arr);
   print!("{:?}", arr);
}

// fn double_negatives(a: &mut [i32; 10]) {
//     for i in 0..10 {
//         if (*a)[i] < 0 { (*a)[i] *= 2; }
//     }
// }


fn double_negatives(a: &mut [i32; 10]) {
    for i in 0..10 {
        if a[i] < 0 { a[i] *= 2; }
    }
}
