use std::io::{self, Write};

#[derive(Debug, PartialEq, Copy, Clone)]
enum Base {
    Decimal,
    Hexadecimal,
    Binary,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Operation {
    And,
    Or,
    Xor,
}

fn get_number() -> i64 {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        let (base, number_str) = match input.get(..2) {
            Some("0x") => (Base::Hexadecimal, &input[2..]),
            Some("0b") => (Base::Binary, &input[2..]),
            _ => (Base::Decimal, input),
        };

        let radix = match base {
            Base::Decimal => 10,
            Base::Hexadecimal => 16,
            Base::Binary => 2,
        };

        match i64::from_str_radix(number_str, radix) {
            Ok(num) => return num,
            Err(_) => println!("Invalid entry. Please try again."),
        }
    }
}

fn get_operation() -> Operation {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "&" | "and" => return Operation::And,
            "|" | "or" => return Operation::Or,
            "^" | "xor" => return Operation::Xor,
            _ => println!("Invalid operation. Please try again."),
        }
    }
}

fn calculate(a: i64, b: i64, op: Operation) -> i64 {
    match op {
        Operation::And => a & b,
        Operation::Or => a | b,
        Operation::Xor => a ^ b,
    }
}

pub fn main() {
    loop {
        print!("Please enter the first number: ");
        io::stdout().flush().unwrap();
        let a = get_number();

        print!("Please enter the second number: ");
        io::stdout().flush().unwrap();
        let b = get_number();

        print!("Please enter the desired operation: ");
        io::stdout().flush().unwrap();
        let op = get_operation();

        let result = calculate(a, b, op);
        println!("The result of {} {:?} {} is {}", a, op, b, result);

        print!("Do you want to perform another calculation? (y/n): ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        if input.trim().to_lowercase() != "y" {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_decimal() {
        assert_eq!(calculate(12, 32, Operation::And), 0);
        assert_eq!(calculate(12, 32, Operation::Or), 44);
        assert_eq!(calculate(12, 32, Operation::Xor), 44);
    }

    #[test]
    fn test_calculate_hexadecimal() {
        assert_eq!(calculate(0xF, 0xA, Operation::And), 0xA);
        assert_eq!(calculate(0xF, 0xA, Operation::Or), 0xF);
        assert_eq!(calculate(0xF, 0xA, Operation::Xor), 0x5);
    }

    #[test]
    fn test_calculate_binary() {
        assert_eq!(calculate(0b1010, 0b1100, Operation::And), 0b1000);
        assert_eq!(calculate(0b1010, 0b1100, Operation::Or), 0b1110);
        assert_eq!(calculate(0b1010, 0b1100, Operation::Xor), 0b0110);
    }

    #[test]
    fn test_calculate_mixed_bases() {
        assert_eq!(calculate(0xF, 0b1111, Operation::And), 15);
        assert_eq!(calculate(10, 0xA, Operation::Or), 10);
        assert_eq!(calculate(0b1010, 10, Operation::Xor), 0);
    }

    #[test]
    fn test_calculate_edge_cases() {
        assert_eq!(calculate(0, 0xFFFFFFFF, Operation::And), 0);
        assert_eq!(calculate(0, 0xFFFFFFFF, Operation::Or), 0xFFFFFFFF);
        assert_eq!(calculate(0xFFFFFFFF, 0xFFFFFFFF, Operation::Xor), 0);
    }
}