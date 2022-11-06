pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn mult(val1: usize, val2: usize, val3: usize) -> usize {
    val1 * val2 * val3
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works_add() {
        let result = add(2, 2);
        assert_eq!(result, 4);

        //let result_mult = mult(2, 3, 4);
        //assert_eq!(result_mult, 25);
    }

    #[test]
    fn it_works_mult() {
        let result_mult = mult(2, 3, 4);
        assert_eq!(result_mult, 24);
    }
}

// DONE create a function in rust that multiples 3 different integers 
// DONE write a second test function which is intended to pass the test
// DONE run the test fuction from your terminal and ensure it passes

