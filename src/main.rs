fn f(x: f64) -> f64 {
    x * x - 3.0 * x + 2.0
} 

fn fp(x: f64) -> f64 {
    2.0 * x - 3.0
}

fn met_newt(mut x0: f64, eps: f64, mut n: u128) -> f64 {
    loop {
        let x = x0 - (f(x0) / fp(x0));

        if ((x - x0).abs() < eps) || (n == 0) {
            return x;
        }

        x0 = x;
        n -= 1;
    }
}

fn met_newt_while(mut x0: f64, eps: f64, mut n: u128) -> f64 {
    let mut x = x0 - (f(x0) / fp(x0));
    while (n > 0) || ((x - x0).abs() > eps) {
        x0 = x;
        x = x0 - (f(x0) / fp(x0));
        n -= 1;
    }

    x 
}

fn met_newt_recur(x0: f64, eps: f64, n: u128) -> f64 {
    if n == 0 {
        return x0;
    }

    let x = x0 - (f(x0) / fp(x0));
    if (x - x0).abs() < eps {
        return x;
    }

    return met_newt_recur(x, eps, n - 1);
}

fn met_newt_for(mut x0: f64, eps: f64, n: u128) -> f64 {
    let mut x = x0 - (f(x0) / fp(x0));
    for _ in 0..n {
        if (x - x0).abs() < eps {
            return x;
        }

        x0 = x;
        x = x0 - (f(x0) / fp(x0));
    }

    x
}

#[allow(unused)]
fn show_ascii() {
    for i in 33..=126 {
        print!("{}: {}\t", i, i as u8 as char);
    }
}

fn collatz_problem(mut n: u64) -> i32 {
    let mut count = 0;

    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        count += 1;
    }

    count
}

fn is_armstrong_number(n: u32) -> bool {
    let mut sum = 0;
    let mut digits = 0;
    let mut temp = n;

    while temp != 0 {
        digits += 1;
        temp /= 10;
    }

    temp = n;

    while temp != 0 {
        let rem = temp % 10;
        sum += rem.pow(digits);
        temp /= 10;
    }

    n == sum
}

fn is_perfect_number(n: u32) -> bool {
    let mut sum = 0;
    for i in 1..n {
        if n % i == 0 {
            sum += i;
        }
    }

    n == sum
}

fn prime_factors(mut n: u64) {
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            print!("{} ", i);
            n /= i;
        } else {
            i += 1;
        }
    }

    if n > 1 {
        print!("{} ", n);
    }

    println!();
}

fn pow_mod(x: u128, n: u128, p: u128) -> u128 {
    let mut result = 1;
    let mut x = x % p;
    let mut n = n;

    while n > 0 {
        if n % 2 == 1 {
            result = (result * x) % p;
        }

        n /= 2;
        x = (x * x) % p;
    }

    result
}
fn main() {
    println!("Result: {:.2}", met_newt(5.0, 1e-6, 1000));
    println!("Result: {:.2}", met_newt_while(5.0, 1e-6, 1000));
    println!("Result: {:.2}", met_newt_recur(5.0, 1e-6, 1000));
    println!("Result: {:.2}", met_newt_for(5.0, 1e-6, 1000));
    //show_ascii();
    println!("Collatz problem: {}", collatz_problem(12));
    println!("Is Armstrong number: {}", is_armstrong_number(153));
    println!("Is Armstrong number: {}", is_armstrong_number(1253));
    println!("Is Perfect number: {}", is_perfect_number(28));
    println!("Is Perfect number: {}", is_perfect_number(12));
    prime_factors(56);
    println!("Pow mod: {}", pow_mod(2, 5, 13));
}


