// drive4.rs
//
// Execute `rustlings hint drive1` or use the `hint` watch subcommand for a
// hint.



// This execrise shares build.rs with the previous exercise.
// You need to add some code to build.rs to make both this exercise and
// the previous one work.


fn main() {
    // This code will only be compiled if the "pass" feature is enabled
    #[cfg(feature = "pass")]
    {
        println!("The 'pass' feature is enabled!");
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        #[cfg(feature = "pass")]
        return;

        panic!("no cfg set");
    }
}
