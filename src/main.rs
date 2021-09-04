fn main() {
    // let outcome = Result::Success(43.78);
    // print_result(outcome);
    // let outcome = Result::Failure(102, 'Y');
    // print_result(outcome);
    // let outcome = Result::Uncertainty;
    // print_result(outcome); // This line will print nothing

    print!("\n ---- New Function ----- ");
    hare_krishna();
    
}

enum Result {
   Success(f64),
   Failure(u16, char),
   Uncertainty,
}

fn print_result(outcome: Result) {
    match outcome {
       Result::Success(_) => println!("OK"), // _ underscore sign , lets skip variable
       Result::Failure(error_code, module) =>
           println!("Error no. {} in module {}", 
                    error_code, module),
       Result::Uncertainty => {},
    }
}

fn hare_krishna()  {
    // println!("Hare Krisna");
    println!("Jai Shri Krishna")
}
