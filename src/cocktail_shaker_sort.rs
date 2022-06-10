fn cocktail_shaker_sort<T: Ord>(arr: &mut[T]) {
    let len = arr.len();
    if len == 0 {
        return;
    }

    loop {
        let mut swapped = false;

        for i in 0..(len-1).clamp(0, len) {
            if arr[i] > arr[i+1] {
                arr.swap(i, i+1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }

        for i in (0..(len-1).clamp(0, len)).rev() {
            if arr[i] > arr[i+1] {
                arr.swap(i+1, i);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }

    }

}

fn main() {
    let mut vec = vec![3,4,5,1,8];
    cocktail_shaker_sort(&mut vec);
    println!("{vec:?}");
}