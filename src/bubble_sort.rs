//Bubble Sort
// Worst case performance O(n^2)
// Best case performance O(n)
// Average case performance O(n^2)

fn bubble_sort<T:Ord>(arr: &mut[T]) {

    let mut sorted = false;
    let mut n = arr.len();
    while !sorted {
        sorted = true;
        for i in 0..n-1 {
            if arr[i] > arr[i+1] {
                arr.swap(i, i+1);
                sorted = false;
            }
        }
        n-=1;
    }
}

fn main() {
    
    let mut vec = vec![2,1,8,3,6,3,5];
    bubble_sort(&mut vec);
    println!("{vec:?}");
}
