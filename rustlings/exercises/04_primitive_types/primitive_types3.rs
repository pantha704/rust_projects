fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???
    let a = [0; 100];
    
    // Created an array `a` with 100 elements, all initialized to 0
    // This means all 100 values in the array are 0s

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
