use std::f64::INFINITY;

fn convert_to(measurement: &str, value: f64) -> f64 {
    if measurement == "celsius" {
        return (value * 9.0 / 5.0) + 32.0;
    } else if measurement == "fahrenheit" {
        return (value - 32.0) * 5.0 / 9.0;
    } else {
        return INFINITY;
    }
}

fn fibonacci(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut f0 = 0;
    let mut f1 = 1;
    let mut f2 = f0 + f1;

    for _ in 2..(n + 1) {
        f2 = f0 + f1;
        f0 = f1;
        f1 = f2;
    }

    return f2;
}

fn main() {
    println!("{}°F is {}°C", 100, convert_to("celsius", 100.0));
    println!("{}°F is {}°C", convert_to("fahrenheit", 100.0), 100);

    println!("Fibonacci number at position 10 is {}", fibonacci(10));
}
