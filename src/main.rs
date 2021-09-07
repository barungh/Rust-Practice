fn main() {
    for n in -2..5 {
        println!("{} is {}.", n, match n {
            0 => "zero",
            1 => "one",
            _ if n < 0 => "negative", // All other cases 'if' lower than 0
            _ => "positive", // and all other cases, here all positive numbers
        });
    }
}

// fn main() {
//     let outcome = Result::Success(43.78);
//     print_result(outcome);
//     let outcome = Result::Failure(102, 'Y');
//     print_result(outcome);
//     let outcome = Result::Uncertainty;
//     print_result(outcome); // This line will print nothing
// }
// 
// enum Result {
//    Success(f64),
//    Failure(u16, char),
//    Uncertainty,
// }
// 
// fn print_result(outcome: Result) {
//     match outcome {
//        Result::Success(_) => println!("OK"), // _ underscore sign , lets skip variable
//        Result::Failure(error_code, module) =>
//            println!("Error no. {} in module {}", 
//                     error_code, module),
//        Result::Uncertainty => {},
//     }
// }
