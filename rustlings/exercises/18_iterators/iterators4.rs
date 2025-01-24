// Method 1: Using iterator and product() method
fn factorial(num: u64) -> u64 {
    // TODO: Complete this function to return the factorial of `num` which is
    // defined as `1 * 2 * 3 * … * num`.
    // https://en.wikipedia.org/wiki/Factorial
    //
    // Do not use:
    // - early returns (using the `return` keyword explicitly)
    // Try not to use:
    // - imperative style loops (for/while)
    // - additional variables
    // For an extra challenge, don't use:
    // - recursion
    
    // The range 1..=num creates an iterator that includes all numbers from 1 to num (inclusive).
    // The product() method calculates the product of all elements in the iterator.
    (1..=num).product()
}

// Method 2: Using recursion
fn factorial_recursive(num: u64) -> u64 {
    match num {
        0 | 1 => 1,
        _ => num * factorial_recursive(num - 1),
    }
}

// Method 3: Using imperative style loop
fn factorial_loop(num: u64) -> u64 {
    let mut result = 1;
    for i in 1..=num {
        result *= i;
    }
    result
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}
