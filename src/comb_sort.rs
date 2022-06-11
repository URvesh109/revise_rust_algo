fn comb_sort<T:Ord>(arr: &mut[T]) {
    let mut gap = arr.len();
    let shrink = 1.3;
    let mut sorted =false;

    while !sorted {
        gap = (gap as f32 / shrink).floor() as usize;

        if gap <= 1 {
            gap = 1;
            sorted = true;
        }

        for i in 0..arr.len() - gap {
            let j = i + gap; 
            if arr[i] > arr[j] {
                arr.swap(i, j);
                sorted = false
            }
        }
    }
}

fn main() {
    let mut vec = vec![6,3,4,8,3,7,9,2];
    comb_sort(&mut vec);
    println!("{vec:?}");
}
