// Zadanie 1.1
fn show_eng_alp() {
    let arr:Vec<_> = ('a'..='z').collect();
    println!("Vector with iter {:?}", arr); 

    let mut arr:Vec<char> = Vec::new();

    for ch in 'a'..='z' {
        arr.push(ch);
    }

    println!("Vector with for {:?}", arr);
}

// Zadanie 1.2
fn pow() {
    let arr: Vec<_> = (1..=10).map(|x| x * x).collect();
    println!("Vector with iter {:?}", arr); 

    let mut arr: Vec<i32> = Vec::new();
    for i in 1..=10 {
        arr.push(i * i);
    }
    println!("Vector with for {:?}", arr);
}

// Zadanie 1.3
fn pow_2() {
    let arr: Vec<_> = (0..10).map(|x| 2_i32.pow(x)).collect();
    println!("Vector with iter {:?}", arr);

    let mut arr: Vec<i32> = Vec::new();
    for i in 0..10 {
        arr.push(2_i32.pow(i));
    }
    println!("Vector with for {:?}", arr);
}

// Zadanie 1.4
fn inverses() {
    let arr: Vec<_> = (1..=20).map(|x| 1.0 / x as f32).collect();
    println!("Vector with iter {:?}", arr);

    let mut arr: Vec<f32> = Vec::new();
    for i in 1..=20 {
        arr.push(1.0 / i as f32);
    }

    println!("Vector with for {:?}", arr);
}

// Zadanie 1.5
fn filter() {
    let arr: Vec<_> = (1..=100).filter(|x| x % 3 == 0 && x % 4 != 0).collect();
    println!("Vector with iter {:?}", arr);

    let mut arr: Vec<i32> = Vec::new();
    for i in 1..=100 {
        if i % 3 == 0 && i % 4 != 0 {
            arr.push(i);
        }
    }
    println!("Vector with for {:?}", arr);
}

// Zadanie 2.1
fn short_string(arr: &Vec<String>) -> Vec<&String> {
    arr.iter().filter(|st| st.len() < 4).collect()
}

// Zadanie 2.2 
fn without_a (arr: &Vec<String>) -> Vec<&String> {
    arr.iter()
        .filter(|st| !st.chars().any(|c| c.to_ascii_lowercase() == 'a'))
        .collect()
}

// Zadanie 2.3
fn with_num (arr: &Vec<String>) -> Vec<&String> {
    arr.iter()
        .filter(|st| st.chars().any(|c| c.is_digit(10)))
        .collect()
}

// Zadanie 2.4
fn rev_string (arr: &Vec<String>) -> Vec<String> {
    arr.iter()
        .map(|st| st.chars().rev().collect())
        .collect() 
}

// Zadanie 2.5
fn double_char (arr: &Vec<String>) -> Vec<&String> {
    arr.iter()
        .filter(|st| {
            st.chars()
                .collect::<Vec<_>>()
                .windows(2)
                .any(|pair| pair[0] == pair[1])
        })  
        .collect()
}

fn indeksy(tablica: &Vec<String>, element: &str) -> Vec<usize> {
    tablica.iter()
        .enumerate()
        .filter(|(_i, e)| {
            if e.len() != element.len() { return false; }

            for j in 0..e.len() {
                if e.chars().nth(j) != element.chars().nth(j) { return false; }
            }

            return true;
        })
        .map(|(i, _e)| i)
        .collect()
}

fn better_indeksy(tablica: &Vec<String>, element: &str) -> Vec<usize> {
    tablica.iter()
        .enumerate()
        .filter_map(|
            (i, el)| 
            if el.len() == element.len() && el.chars().zip(element.chars()).all(|(c1, c2)| c1 == c2) {
                Some(i)
            } else {
                None
            })
        .collect()
}

fn main() {
    show_eng_alp();
    pow();
    pow_2();
    inverses();
    filter();

    let arr = vec![
        String::from("Rust"),
        String::from("is"),
        String::from("awe5some"),
        String::from("and"),
        String::from("fast"),
        String::from("language 45"),
        String::from("inne"),
        String::from("is"),
    ];

    println!("short_string: {:?}", short_string(&arr));
    println!("without a: {:?}", without_a(&arr));
    println!("with numbers: {:?}", with_num(&arr));
    println!("reverse string: {:?}", rev_string(&arr));
    println!("double char: {:?}", double_char(&arr));

    println!("iters: {:?}", indeksy(&arr, &"is"));
    println!("better_indeksy: {:?}", better_indeksy(&arr, &"is"));
}