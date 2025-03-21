
// Zadanie 1
fn is_leap_year(year: i32) -> bool {
    if year % 4 == 0 || year % 400 == 0 && year % 100 != 0 {
        return true;
    } else {
        return false
    }
}

// Zadanie 2
fn days_amount(month: i32, year: i32) -> i32 {
    if month < 1 || month > 12 {
        return -1;
    }

    let days: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    if month == 2 && is_leap_year(year) {
        return 29;
    }

    return days[(month - 1) as usize];
}

// Zadanie 3
fn convert_to_f(c: f32) -> f32 {
    return 32.0 + c * 9.0 / 5.0;
}

// Zadanie 4
fn convert_to_c(f: f32) -> f32 {
    return (f - 32.0) * 5.0 / 9.0;
}

// zadanie 5
fn minus_time() {
    let g1 = 7;
    let m1 = 16;
    let s1 = 50;

    let g2 = 6;
    let m2 = 59;
    let s2 = 02;

    let conv1 = g1 * 3600 + m1 * 60 + s1;
    let conv2 = g2 * 3600 + m2 * 60 + s2;
    let diff = conv1 - conv2;
    println!("{}:{}:{}", diff / 3600, (diff % 3600) / 60, diff % 60);
}

//Zadanie 6
fn factorial(n: u32) -> u32 {
    let mut result: u32 = 1;
    let mut i = 1;
    while i != n {
        result += i * result;
        i += 1;
    }

    return result;
}

fn factorial_loop(n: u32) -> u32 {
    let mut result: u32 = 1;
    let mut i = 1;
    loop {
        if i == n {
            break result;
        }

        result += i * result;
        i += 1;
    }
}

fn factorial_for(n: u32) -> u32 {
    let mut result: u32 = 1;
    for i in 1..n {
        result += i * result;
    }

    result
}

//Zadanie 7
fn show_digits(number: i32) {
    let mut n = number;
    while n > 0 {
        print!("{} ", n % 10);
        n /= 10;
    }
    println!();
}

//Zadanie 8
fn sum_of_digits(number: i32) -> i32 {
    let mut result = 0;
    let mut n = number;

    while n > 0 {
        result += n % 10;
        n  /= 10;
    }

    return result;
}

//Zadanie 9
fn pythagorean_triples(number: i32) {
    let mut a = 1;
    while a <= number {
        let mut b = a + 1;
        while b <= number {
            let mut c = b + 1;
            while c * c < a * a + b * b {
                c += 1;
            }

            if c * c == a * a + b * b && c <= number {
                println!("({}, {}, {})", a, b, c);
            }

            b += 1;
        }

        a += 1;
    }
}

fn pythagorean_triples_loop(number: i32) {
    let mut a = 1;
    loop {
        if a > number {
            break;
        }

        let mut b = a + 1;
        loop {
            if b > number {
                break;
            }

            let mut c = b + 1;
            loop {
                if c > number {
                    break;
                }

                let c_square = a * a + b * b;

                if c * c == c_square {
                    println!("({}, {}, {})", a, b, c);
                }

                c += 1;
            }

            b += 1;
        }

        a += 1;
    }
}

fn main() {
    println!("{}", convert_to_f(30.0));
    println!("{}", convert_to_c(86.0));
    println!("{}", convert_to_f(1.0));
    println!("{}", convert_to_f(30.0));
    minus_time();
    println!("{}", factorial(5));
    println!("{}", factorial_loop(5));
    println!("{}", factorial_for(5));
    println!("{}", is_leap_year(2025));
    println!("{}", days_amount(5, 2025));
    show_digits(56789);
    println!("{}", sum_of_digits(16));
    pythagorean_triples(20);
    println!("-----------------");
    pythagorean_triples_loop(20);
}
    
