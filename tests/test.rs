use std::vec;

use radix_sort::lsd_sort;

use std::time::Instant;

const KX: u32 = 123456789;
const KY: u32 = 362436069;
const KZ: u32 = 521288629;
const KW: u32 = 88675123;
pub struct Rand {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}
impl Rand {
    pub fn new(seed: u32) -> Rand {
        Rand {
            x: KX ^ seed,
            y: KY ^ seed,
            z: KZ,
            w: KW,
        }
    }
    pub fn rand(&mut self) -> u32 {
        let t = self.x ^ self.x.wrapping_shl(11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w ^= self.w.wrapping_shr(19) ^ t ^ t.wrapping_shr(8);
        return self.w;
    }
    pub fn shuffle<T>(&mut self, a: &mut [T]) {
        if a.len() == 0 {
            return;
        }
        let mut i = a.len() - 1;
        while i > 0 {
            let j = (self.rand() as usize) % (i + 1);
            a.swap(i, j);
            i -= 1;
        }
    }
    pub fn rand_range(&mut self, a: i32, b: i32) -> i32 {
        let m = (b - a + 1) as u32;
        return a + (self.rand() % m) as i32;
    }
    pub fn rand_float(&mut self) -> f64 {
        (self.rand() as f64) / (<u32>::max_value() as f64)
    }
}

#[test]
fn test_lsd_1() {
    let mut arr: Vec<u32> = vec![4, 678, 234, 1, 8904, 45, 909, 13, 45333, 23000];
    lsd_sort(&mut arr);
    assert_eq!(arr, vec![1, 4, 13, 45, 234, 678, 909, 8904, 23000, 45333]);
}
#[test]
fn test_lsd_2() {
    let mut arr = vec![0; 100_000];
    let mut reference = vec![0; 100_000];
    let mut rng = Rand::new(0);
    for idx in 0..100_000 {
        let value = rng.rand_range(1, 10000000) as u64;
        arr[idx] = value;
        reference[idx] = value;
    }
    reference.sort();
    lsd_sort(&mut arr);
    assert_eq!(arr, reference);
}
#[test]
fn test_lsd_3() {
    let mut arr = vec![0; 100_000];
    let mut reference = vec![0; 100_000];
    let mut rng = Rand::new(0);
    for idx in 0..100_000 {
        let value = rng.rand_range(1, 10000000) as u32;
        arr[idx] = value;
        reference[idx] = value;
    }
    reference.sort();
    lsd_sort(&mut arr);
    assert_eq!(arr, reference);
}
#[test]
fn test_lsd_4() {
    let mut arr = vec![0; 100_000];
    let mut reference = vec![0; 100_000];
    let mut rng = Rand::new(0);
    for idx in 0..100_000 {
        let value = rng.rand_range(1, 60000) as u16;
        arr[idx] = value;
        reference[idx] = value;
    }
    reference.sort();
    lsd_sort(&mut arr);
    assert_eq!(arr, reference);
}

#[test]
fn test_lsd_5() {
    let mut arr = vec![0; 10_000_000];
    let mut reference = vec![0; 10_000_000];
    let mut rng = Rand::new(0);
    for idx in 0..10_000_000 {
        let value = rng.rand_range(1, 255) as u8;
        arr[idx] = value;
        reference[idx] = value;
    }
    let start = Instant::now();
    reference.sort();
    println!("Standart sorting: {} ms", start.elapsed().as_millis());
    let start = Instant::now();
    lsd_sort(&mut arr);
    println!("Radix sorting: {:?} ms", start.elapsed().as_millis());
    assert_eq!(arr, reference);
}

#[test]
fn test_lsd_6() {
    let mut arr = vec![0; 10_000_000];
    let mut reference = vec![0; 10_000_000];
    let mut rng = Rand::new(3007);
    for idx in 0..10_000_000 {
        let value = rng.rand_range(1, 400000000) as u32;
        arr[idx] = value;
        reference[idx] = value;
    }
    let start = Instant::now();
    reference.sort();
    println!("Standart sorting: {} ms", start.elapsed().as_millis());
    let start = Instant::now();
    lsd_sort(&mut arr);
    println!("Radix sorting: {:?} ms", start.elapsed().as_millis());
    assert_eq!(arr, reference);
}
