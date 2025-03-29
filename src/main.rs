fn zamien_syst8_na_syst2(z: &str) -> Option<String> {
    if z.len() == 0 {
        None
    } else {
        let num = u32::from_str_radix(z, 8).ok()?;
        
        Some(format!("{:b}", num))
    }
}

fn wartosc_syst2(z: &str) -> Option<u8> {
    if z.len() == 0 || z.len() > 8 {
        return None;
    }

    let mut result: u8 = 0;
    for (i, char) in z.chars().enumerate() {
        let digit = char.to_digit(10)? as u8;
        result += digit * (2u8.pow((z.len() - i - 1) as u32)) as u8;
    }

    Some(result)
}

fn wartosc_syst8(z: &str) -> Option<u8> {
    let binary = zamien_syst8_na_syst2(z);

    if binary.is_none() {
        return None;
    }

    let binary = binary.unwrap();
    wartosc_syst2(&binary)
}

fn wartosc_syst8_with_question_mark(z: &str) -> Option<u8> {
    let binary = zamien_syst8_na_syst2(z)?;
    wartosc_syst2(&binary)
}

fn wartosc_cyfry(c: char) -> Result<u8, String> {
    c.to_digit(10)
        .ok_or(format!("'{c}' is not a digit"))
        .map(|n| n as u8)
}

fn dodaj_pisemnie(a: &str, b: &str) -> Result<String, String> {
    let a_vec = a.chars().rev().collect::<Vec<char>>();
    let b_vec = b.chars().rev().collect::<Vec<char>>();

    if a_vec.is_empty() || b_vec.is_empty() {
        return Err("invalid data: some string is empty".to_string());
    }

    let mut carry: u8 = 0;

    let max_size = a_vec.len().max(b_vec.len());

    let mut result = String::new();

    for i in 0..max_size {
        let num1 = a_vec.get(i).unwrap_or(&'0');
        let num2 = b_vec.get(i).unwrap_or(&'0');

        let digit_1 = wartosc_cyfry(*num1)?;
        let digit_2 = wartosc_cyfry(*num2)?;

        let sum = digit_1 + digit_2 + carry;
        carry = sum / 10;
        result.push(char::from_digit(sum as u32 % 10, 10).unwrap());
    }

    if carry > 0 {
        result.push(char::from_digit(carry as u32, 10).unwrap());
    }

    Ok(result.chars().rev().collect::<String>())
}

fn wartosc_cyfry_rzymskiej(c: char) -> Result<u16, String> {
    match c {
        'I' => return Ok(1),
        'V' => return Ok(5),
        'X' => return Ok(10),
        'L' => return Ok(50),
        'C' => return Ok(100),
        'D' => return Ok(500),
        'M' => return Ok(1000),
        _ => return Err(format!("{c} is not a roman number"))
    }
}

fn rzymskie(napis: &str) -> Result<u128, String> {
    if napis.is_empty() {
        return Err("String is empty".to_string());
    }

    let mut res: u128 = 0;
    let mut i = 0;
    let mut prev_value = 0;
    let mut repeat_count = 0;
    let mut last_char = None;

    while i < napis.len() {
        let current_char = napis.chars().nth(i).unwrap();
        let s1 = wartosc_cyfry_rzymskiej(current_char)?;

        if last_char == Some(current_char) && matches!(s1, 1 | 10 | 100 | 1000) {
            repeat_count += 1;
            if repeat_count > 3 {
                return Err(format!("Wrong count of repeating: {} more than 3 times", current_char));
            }
        } else {
            repeat_count = 1;
            last_char = Some(current_char);
        }

        if i + 1 < napis.len() {
            let s2 = wartosc_cyfry_rzymskiej(napis.chars().nth(i + 1).unwrap())?;

            if s1 >= s2 {
                if s1 == s2 && matches!(s1, 5 | 50 | 500) {
                    return Err(format!("Invalid repeating: {} cannot be repeat", current_char));
                }

                res += s1 as u128;
                prev_value = s1;
                i += 1;
            } else {
                if matches!(s1, 5 | 10 | 500) {
                    return Err(format!("Invalid order: {} cannot be previous greater value", current_char));
                }

                let valid_subtraction = matches!(
                    (s1, s2),
                    (1, 5) | (1, 10) | (10, 50) | (10, 100) | (100, 500) | (100, 1000)
                );

                if !valid_subtraction {
                    return Err(format!("Invalid substraction: {}{}", current_char, napis.chars().nth(i + 1).unwrap()));
                }

                let substraction = s2 - s1;
                if substraction > prev_value && prev_value != 0 {
                    return Err(format!("Invalid substraction: value after substraction greater than previous"));
                }

                res += substraction as u128;
                prev_value = substraction;
                i += 2;
            }
        } else {
            res += s1 as u128;
            prev_value = s1;
            i += 1;
        }
    }

    Ok(res)
}

fn main() {
    println!("zamien_syst8_na_syst2 {:?}", zamien_syst8_na_syst2("72"));
    println!("zamien_syst8_na_syst2 {:?}", zamien_syst8_na_syst2(""));
    println!("zamien_syst8_na_syst2 {:?}", zamien_syst8_na_syst2("987"));

    println!("wartosc_syst2 {:?}", wartosc_syst2("10000000"));

    println!("wartosc_syst8 {:?}", wartosc_syst8("72"));
    println!("wartosc_syst8 {:?}", wartosc_syst8_with_question_mark("72"));
    println!("wartosc_cyfry {:?}", wartosc_cyfry('7'));

    println!("dodaj_pisemnie {:?}", dodaj_pisemnie("25a", "998"));

    println!("rzymskie {:?}", rzymskie("MCCX"));
}