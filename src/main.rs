fn swap(a: &mut i32, b: &mut i32) {
    let temp = *a;
    *a = *b;
    *b = temp;
}

fn sort_with_if(a: &mut i32, b: &mut i32, c: &mut i32) {
    let arr = &mut [a, b, c];

    for i in 0..3 {
        for j in 0..3 - i - 1 {
            let (left, right) = arr.split_at_mut(j + 1);
            if *left[j] > *right[0] {
                swap(left[j], right[0]);
            }
        }
    }
}

fn rand(seed: &mut u128, min_rand: i32, max_rand: i32) -> i32 {
    let m = 2_u128.pow(16) + 1;
    let a = 75;
    let c = 74;

    *seed = (a * *seed + c) % m;
    *seed as i32 % (max_rand - min_rand + 1) as i32 + min_rand as i32
}

fn swap_arr(arr: &mut [i32], i: usize, j: usize) {
    let temp = arr[i];
    arr[i] = arr[j];
    arr[j] = temp;
}

fn rand_perm(arr: &mut [i32], seed: &mut u128) {
    for i in (1..arr.len()).rev() {
        let j = rand(seed, 0, i as i32);
        swap_arr(arr, i, j as usize);
    }
}

fn main() {
    let mut a = 16;
    let mut b = 32;
    let mut c = 8;
    swap(&mut a, &mut b);
    dbg!(a);
    dbg!(b);

    sort_with_if(&mut a, &mut b, &mut c);
    dbg!(a);
    dbg!(b);
    dbg!(c);

    let min = 10;
    let max = 20;
    println!("Rand number {}", rand(&mut 56584879574, min, max));

    let mut arr = [5, 6, 67, 32, 15];
    swap_arr(&mut arr, 3, 1);
    println!("{:?}", arr);

    let mut arr = [1, 2, 3, 4, 5, 6, 7];
    rand_perm(&mut arr, &mut 463657);
    println!("{:?}", arr);

}
