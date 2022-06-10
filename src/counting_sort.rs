fn counting_sort(arr: &mut[u32], max_val: usize) {
    let mut occurrence: Vec<usize> = vec![0; max_val + 1];

    for &data in arr.iter() {
        occurrence[data as usize] += 1;
    }

    let mut i = 0;

    for (data, &number) in occurrence.iter().enumerate() {
        for _ in 0..number {
            arr[i] = data as u32;
            i+=1
        }
    }
}
use std::ops::AddAssign;

fn generic_counting_sort<T: Into<u64> + From<u8> + Copy + AddAssign>(
    arr: &mut[T],
    max_val: usize,
) {

    let mut occurences: Vec<usize> = vec![0; max_val+1];
    
    for &data in arr.iter() {
        occurences[data.into() as usize] += 1;
    }

    let mut i = 0;

    for (data, &number) in occurences.iter().enumerate() {
        for _ in 0..number {
            arr[i] = T::from(data as u8);
            i += 1;
        }
    }

}


fn main() {
    let mut ve1:Vec<u64> = vec![6,1, 5, 4, 4, 3, 8];
    generic_counting_sort(&mut ve1, 8);
    println!("{ve1:?}");
}