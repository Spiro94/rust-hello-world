//Enums

fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    enum IpAddr {
        V4(String),
        V6(String),
    }

    let home = IpAddr::V4(String::from("127.0.0.1"));
    let loopback = IpAddr::V6(String::from("::1"));

    fn divide(numerator: f64, denominator: f64) -> Option<f64> {
        if denominator == 0.0 {
            None
        } else {
            Some(numerator / denominator)
        }
    }

    fn divideR(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0.0 {
            Err("Division by 0 is not possible".to_string())
        } else {
            Ok(numerator / denominator)
        }
    }

    let result = divide(15.0, 0.0);
    println!("Result: {:?}", result);
    let resultR = divideR(16.0, 0.0);
    println!("ResultR: {:?}", resultR);
}
