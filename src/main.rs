struct RandGen {
    seed: i64,
}

impl RandGen {
    fn new(seed: i64) -> Self {
        Self { seed }
    }

    fn gen_range(&self, from: i64, to: i64) -> i64 {
        let a = 75;
        let c = 0;
        let m = 2_i64.pow(16) + 1;
        let seed = (a * self.seed + c) % m;
        seed % (to - from + 1) + from
    }
}

struct Urna {
    znaki: Vec<char>,
    rand_gen: RandGen,
}

impl Urna {
    fn new(rand_gen: RandGen) -> Self {
        let znaki: Vec<char> = Vec::new();
        Self { znaki, rand_gen }
    }

    fn losuj_bez_us(&self) -> Option<char> {
        if self.znaki.is_empty() {
            return None;
        }

        let n = self.rand_gen.gen_range(0, self.znaki.len() as i64 - 1) as usize;

        let znak = self.znaki.iter().nth(n);

        if let Some(z) = znak {
            Some(*z)
        } else {
            None
        }
    }

    fn losuj_z_us(&mut self) -> Option<char> {
        if self.znaki.is_empty() {
            None
        } else {
            let n = self.rand_gen.gen_range(0, self.znaki.len() as i64 - 1) as usize;

            Some(self.znaki.swap_remove(n))
        }
    }

    fn doloz(&mut self, znak: char) {
        self.znaki.push(znak);
    }

    fn rozmiar(&self) -> usize {
        self.znaki.len()
    }
}

#[derive(PartialEq, Clone)]
enum Jednostka {
    Sztuki,
    Litry,
    Kilogram,
}

#[derive(PartialEq, Clone)]
enum WarunkiPrzechowywania {
    Zamrazarka,
    Chlodziarka,
    Normalne,
}

#[derive(PartialEq, Clone)]
struct Towar {
    nazwa: String,
    jednostka: Jednostka,
    waga: f64,
    warunki: WarunkiPrzechowywania,
}

impl Towar {
    fn new(nazwa: String, jednostka: Jednostka, waga: f64, warunki: WarunkiPrzechowywania) -> Self {
        if waga <= 0.0 {
            panic!("Waga musi być dodatnia!");
        }

        match jednostka {
            Jednostka::Kilogram => Self {
                nazwa,
                jednostka,
                waga: 1.0,
                warunki,
            },
            _ => Self {
                nazwa,
                jednostka,
                waga,
                warunki,
            },
        }
    }
}

enum Count {
    LiczbaCalkowita(i32),
    LiczbaWymierna(f64),
}

impl Count {
    fn increase_count(&mut self, ilosc: f64) {
        match self {
            Count::LiczbaCalkowita(n) => *n += ilosc as i32,
            Count::LiczbaWymierna(n) => *n += ilosc,
        }
    }

    fn as_f64(&self) -> f64 {
        match self {
            Count::LiczbaCalkowita(n) => *n as f64,
            Count::LiczbaWymierna(n) => *n,
        }
    }
}

struct Zamowienie {
    towary: Vec<(Towar, Count)>,
}

impl Zamowienie {
    fn new() -> Self {
        Self { towary: Vec::new() }
    }

    fn weight(&self) -> f64 {
        self.towary
            .iter()
            .fold(0.0, |acc, (towar, count)| acc + towar.waga * count.as_f64())
    }

    fn weight_with_store_type(&self, warunki: WarunkiPrzechowywania) -> f64 {
        self.towary
            .iter()
            .filter(|(towar, _)| towar.warunki == warunki)
            .fold(0.0, |acc, (towar, count)| acc + towar.waga * count.as_f64())
    }

    fn add_item(&mut self, towar: Towar, ilosc: f64) {
        if ilosc <= 0.0 {
            panic!("Ilość musi być dodatnia")
        }

        if let Jednostka::Sztuki = towar.jednostka {
            if ilosc.fract() != 0.0 {
                panic!("Ilość dla jednostki 'Sztuki' musi być liczbą całkowitą");
            }
        }

        let item = self
            .towary
            .iter_mut()
            .find(|(towar_in_order, _)| *towar_in_order == towar);

        if item.is_none() {
            match towar.jednostka {
                Jednostka::Sztuki => self
                    .towary
                    .push((towar, Count::LiczbaCalkowita(ilosc as i32))),
                _ => self.towary.push((towar, Count::LiczbaWymierna(ilosc))),
            }
        } else {
            item.unwrap().1.increase_count(ilosc);
        }
    }
}

//fn main() {
// let mut generator1 = RandGen::new(123);
// let a = generator1.gen_range(3, 15);
// let b = generator1.gen_range(3, 15);
// let c = generator1.gen_range(3, 15);

// let mut generator2 = RandGen::new(123);
// let a2 = generator2.gen_range(3, 15);
// let b2 = generator2.gen_range(3, 15);
// let c2 = generator2.gen_range(3, 15);

// println!("{}", a == a2);
// println!("{}", b == b2);
// println!("{}", c == c2);

// println!("{}", a >= 3);
// println!("{}", b >= 3);
// println!("{}", c >= 3);

// println!("{}", a <= 15);
// println!("{}", b <= 15);
// println!("{}", c <= 15);

// let mut urna = Urna::new(RandGen::new(123));

// let a: Option<char> = urna.losuj_z_us();
// println!("{:?}", a.is_none());
// let a: Option<char> = urna.losuj_bez_us();
// println!("{:?}", a.is_none());

// urna.doloz('a');
// urna.doloz('b');
// urna.doloz('c');
// urna.doloz('d');

// println!("{:?}", urna.rozmiar() == 4);
// let y: Option<char> = urna.losuj_z_us();
// println!("{:?}", y.is_some());
// println!("{:?}", urna.rozmiar() == 3);
// let x: Option<char> = urna.losuj_bez_us();
// println!("{:?}", x.is_some());
// println!("{:?}", urna.rozmiar() == 3);
// println!("{:?}", x != y);
// urna.losuj_z_us();
// println!("{:?}", urna.rozmiar() == 2);
// urna.losuj_z_us();
// println!("{:?}", urna.rozmiar() == 1);
// urna.losuj_z_us();
// println!("{:?}", urna.rozmiar() == 0);
// let z: Option<char> = urna.losuj_z_us();
// println!("{:?}", z.is_none());
// println!("{:?}", urna.rozmiar() == 0);

//     let towar1 = Towar::new(
//         "Mleko".to_string(),
//         Jednostka::Litry,
//         2.0,
//         WarunkiPrzechowywania::Chlodziarka,
//     );

//     let towar2 = Towar::new(
//         "Cukier".to_string(),
//         Jednostka::Kilogram,
//         1.0,
//         WarunkiPrzechowywania::Normalne,
//     );

//     let zamowienie = Zamowienie {
//         towary: vec![
//             (towar1, Count::LiczbaWymierna(5.0)),
//             (towar2, Count::LiczbaWymierna(7.0)),
//         ],
//     };

//     let waga_chlodziarka = zamowienie.weight_with_store_type(WarunkiPrzechowywania::Chlodziarka);
//     let waga_normalne = zamowienie.weight_with_store_type(WarunkiPrzechowywania::Normalne);

//     println!("Waga dla chłodziarki: {}", waga_chlodziarka); // 6.0
//     println!("Waga dla normalnych warunków: {}", waga_normalne); // 5.0
//     println!("Waga dla wszystkich: {}", zamowienie.weight());
// }

fn main() {
    use Jednostka::*;
    use WarunkiPrzechowywania::*;

    let mut zamowienie = Zamowienie::new();

    let towar1 = Towar::new("Mleko".to_string(), Litry, 1.0, Chlodziarka);
    let towar2 = Towar::new("Lód".to_string(), Kilogram, 5.0, Zamrazarka);
    let towar3 = Towar::new("Jabłko".to_string(), Sztuki, 0.2, Normalne);

    // Додавання товарів
    zamowienie.add_item(towar1.clone(), 2.5); // 2.5 * 1.0
    zamowienie.add_item(towar2.clone(), 3.0); // 3.0 * 1.0 (waga wymuszana)
    zamowienie.add_item(towar3.clone(), 5.0); // 5 szt * 0.2

    // Перевірка сумарної ваги
    println!("Całkowita waga: {}", zamowienie.weight()); // Очікується: 2.5 + 3.0*1 + 5*0.2 = 2.5 + 3 + 1 = 6.5

    // Перевірка по категорії зберігання
    println!(
        "Waga (Zamrażarka): {}",
        zamowienie.weight_with_store_type(Zamrazarka)
    ); // 3.0
    println!(
        "Waga (Chłodziarka): {}",
        zamowienie.weight_with_store_type(Chlodziarka)
    ); // 2.5
    println!(
        "Waga (Normalne): {}",
        zamowienie.weight_with_store_type(Normalne)
    ); // 1.0

    // Перевірка збільшення кількості існуючого товару
    zamowienie.add_item(towar1.clone(), 0.5); // додати ще 0.5 л молока
    println!("Waga po dodaniu mleka: {}", zamowienie.weight()); // +0.5*1.0 = 7.0

    // Некоректне додавання
    //zamowienie.add_item(towar3.clone(), -3.0); // панікує
    // zamowienie.add_item(towar3.clone(), 1.5); // панікує, бо sztuki -> неціле
}
