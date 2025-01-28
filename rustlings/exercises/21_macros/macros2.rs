macro_rules! my_macro {
    // TODO: Fix the compiler error by moving the whole definition of this macro.
    () => {
        println!("Check out my macro!");
    };
}
fn main() {
    my_macro!();
}

