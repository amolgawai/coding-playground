fn main() {
    println!("Hello, world!");
}

fn gcd(num1: u8, num2: u8) -> u8 {
    assert!(num1 != 0 && num2 !=0);
    let mut m = num1;
    let mut n = num2;
    //Euler's algo
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;

    }
    n

}

mod gcd_tests {
    use super::*;
    #[test]
    fn gcd_test() {
        assert_eq!(gcd(15, 14), 1);
    }

}
