#[derive(Debug, PartialEq, Clone)]
struct Rgb {
    red: u8,
    green: u8,
    blue: u8,
}

impl Rgb {
    fn from_3u8(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    fn from_3percent(red: f32, green: f32, blue: f32) -> Option<Self> {
        if red < 0.0 || red > 100.0 ||
            green < 0.0 || green > 100.0 ||
            blue < 0.0 || blue > 100.0 {
                None
            } else {
                Some(Self {
                    red: (red * 2.55) as u8,
                    green: (green * 2.55) as u8,
                    blue: (blue * 2.55) as u8,
                })
            }
    }

    fn gray(value: f32) -> Option<Self> {
        if value < 0.0 || value > 100.0 {
            None
        } else {
            Some(Self {
                red: (value * 2.55) as u8,
                green: (value * 2.55) as u8,
                blue: (value * 2.55) as u8,
            })
        }
    }

    fn white() -> Self {
        Self {
            red:255, 
            green:255, 
            blue:255
        }
    }

    fn black() -> Self {
        Self {
            red:0,
            green: 0,
            blue: 0,
        }
    }

    fn invert(&mut self) {
        self.red = 255 - self.red;
        self.green = 255 - self.green;
        self.blue = 255 - self.blue;
    }

    fn intensity(&self) -> f32 {
        (self.red as f32 + self.green as f32 + self.blue as f32) / (3.0 * 255.0)
    }

    fn as_rgb_u8tuple(&self) -> (u8, u8, u8) {
        (self.red, self.green, self.blue)
    }

    fn as_cmy_u8tuple(&self) -> (u8, u8, u8) {
        let mut tmp = self.clone();
        tmp.invert();
        (tmp.red, tmp.green, tmp.blue)
    }
}

struct Macierz {
    mat: Vec<Vec<f64>>,
    cols: usize,
    rows: usize,
}

impl PartialEq for Macierz  {
    fn eq(&self, other: &Self) -> bool {
        self.rows == other.rows && self.cols == other.cols
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Macierz {
    fn new(wysokość: usize, szerokość: usize, wypełniacz: f64) -> Self {
        Self {
            mat: vec![vec![wypełniacz; szerokość]; wysokość],
            cols: szerokość,
            rows: wysokość,
        }
    }

    fn zerowa(wysokość: usize, szerokość: usize) -> Self {
        Self {
            mat: vec![vec![0.0; szerokość]; wysokość],
            cols: szerokość,
            rows: wysokość,
        }
    }

    fn jednostkowa(wysokość: usize) -> Self {
        Self {
            mat: vec![vec![0.0; 1]; wysokość],
            cols: 1,
            rows: wysokość,
        }
    }

    fn element(&self, indeks_wiersz: usize, indeks_kolumn: usize) -> f64 {
        *self.mat.get(indeks_wiersz).unwrap().get(indeks_kolumn).unwrap()
    }

    fn zmien_element(&mut self, indeks_wiersz: usize, indeks_kolumn: usize, nowa_wartość: f64) {
        self.mat[indeks_wiersz][indeks_kolumn] = nowa_wartość;
    }

    fn suma(macierz1: Macierz, macierz2: Macierz) -> Option<Self> {
        if macierz1 != macierz2 {
            None
        } else {
            let rows = macierz1.rows;
            let cols = macierz2.cols;
            let mut res = Macierz::zerowa(rows, cols);

            for i in 0..rows {
                for j in 0..cols {
                    let sum = macierz1.element(i, j) + macierz2.element(i, j);
                    res.zmien_element(i, j, sum);
                }
            }

            Some(res)
        }
    }

    fn wyświetl(&self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                print!("{} ", self.element(i, j));
            }
            println!("");
        }
    }
}

fn main() {
    let szary1 = Rgb::from_3u8(127, 127, 127);
    let szary2 = Rgb::from_3percent(50.0, 50.0, 50.0).unwrap();
    let szary3 = Rgb::gray(50.0).unwrap();
    let fiolet = Rgb::from_3u8(100, 35, 120);
    let bialy1 = Rgb::white();
    let bialy2 = Rgb::from_3u8(255, 255, 255);
    let mut czarny1 = Rgb::black();
    let czarny2 = Rgb::from_3u8(0, 0, 0);
    println!("{} {}", szary1 == szary2, szary1 == szary3);
    println!("{} {}", bialy1 == bialy2, czarny1 == czarny2);
    czarny1.invert();
    println!("{}", bialy1 == czarny1);
    println!("{}", fiolet.intensity() == 1.0 / 3.0);
    println!("{}", fiolet.as_rgb_u8tuple() == (100, 35, 120));
    println!("{}", fiolet.as_cmy_u8tuple() == (155, 220, 135));


    // Macierz
    let mat1 = Macierz::new(3, 3, 12.56);
    let mut mat2 = Macierz::zerowa(10, 7);
    let mat3 = Macierz::jednostkowa(8);

    mat3.wyświetl();

    println!("element of mat2: {}", mat2.element(7, 3));
    mat2.zmien_element(7, 3, 56.72);
    println!("element of mat2: {}", mat2.element(7, 3));

    let mat4 = Macierz::new(3, 3, 7.44);
    let mat_sum = Macierz::suma(mat1, mat4);

    if mat_sum.is_some() {
        mat_sum.unwrap().wyświetl();
    } else {
        println!("mat_sum error");
    }
}
    
