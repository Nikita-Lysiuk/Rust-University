fn liczba_wystapien(napis: &str, znak: &char) -> i32 {
    let mut count = 0;
    for a in napis.chars() {
        if a.eq(znak) {
            count += 1;
        }
    }

    count
}

fn symbol(r: &char) -> i32 {
    if r.eq(&'I') { 1 }
    else if r.eq(&'V') { 5 }
    else if r.eq(&'X') { 10 }
    else if r.eq(&'L') { 50 }
    else if r.eq(&'C') { 100 }
    else if r.eq(&'D') { 500 }
    else if r.eq(&'M') { 1000 }
    else { -1 }
}

fn rzymskie(napis: &str) -> i32 {
    let mut res = 0;
    let mut i = 0;

    while i < napis.len() {
        let s1 = symbol(&napis.chars().nth(i).unwrap());

        if i + 1 < napis.len() {
            let s2 = symbol(&napis.chars().nth(i + 1).unwrap());

            if s1 >= s2 {
                res = res + s1;
                i += 1;
            } else {
                res = res + s2 - s1;
                i += 2;
            }
        } else {
            res = res + s1;
            i += 1;
        }
    }    

    res
}

fn co_drugi_znak(napis: &str) -> String {
    napis.chars().step_by(2).collect()
}

fn szyfruj(napis: &str, klucz: usize) -> String {
    let mut result: String = String::new();
    let part = napis.len() / klucz;

    for i in 0..part {
        let mut add_str: String = String::new();
        for j in 0..klucz {
            add_str.push(
                napis.chars().nth(i * klucz + j).unwrap()
            );
        }

        result.push_str(
            &add_str.chars().rev().collect::<String>()
        );
    }


    result
}

fn wizytowka(imie: &str, nazwisko: &str) -> String {
    let inicjal = imie.chars().next().unwrap().to_uppercase().to_string();
    let mut nazwisko = nazwisko.to_lowercase();
    if let Some(first) = nazwisko.chars().next() {
        nazwisko.replace_range(0..first.len_utf8(), &first.to_uppercase().to_string());
    }

    format!("{}. {}", inicjal, nazwisko)
}

fn na_rzymskie(liczba: i32) -> String {
    let num = [1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    let sym = ["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];

    let mut number = liczba;
    let mut result = String::new();

    for (i, &value) in num.iter().enumerate() {
        while number >= value {
            number -= value;
            result.push_str(sym[i]);
        }
    }

    result
}

fn dodaj_pisemnie(a: &str, b: &str) -> String {
    let mut result = String::new();
    let mut carry = 0;

    let a_chars: Vec<_> = a.chars().rev().collect();
    let b_chars: Vec<_> = b.chars().rev().collect();

    let max_len = a_chars.len().max(b_chars.len());

    for i in 0..max_len {
        let digit_a = a_chars.get(i).and_then(|c| c.to_digit(10)).unwrap_or(0);
        let digit_b = b_chars.get(i).and_then(|c| c.to_digit(10)).unwrap_or(0);

        let sum = digit_a + digit_b + carry;
        carry = sum / 10;
        result.push(char::from_digit(sum % 10, 10).unwrap());
    }

    if carry > 0 {
        result.push(char::from_digit(carry, 10).unwrap());
    }

    result.chars().rev().collect()
}

fn main() {
    println!("Liczba wystapien a: {}", liczba_wystapien("kfhkgfjgjfadfdsgdsgdfdfffgdaaaa", &'a'));

    println!("Rome number MCMX {}", rzymskie(&"MCMX"));

    println!("{}", co_drugi_znak(&"napis"));

    println!("{}", szyfruj(&"Aladyn", 2));

    println!("{}", wizytowka("nikita", "LYSIUK"));

    println!("{}", na_rzymskie(3549));

    println!("{}", dodaj_pisemnie("5924729874298749827418582", "6782893629472094209740298"))
}
