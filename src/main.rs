/// Sort a slice using bucket sort algorithm.
///
/// Time complexity is `O(n + k)` on average, where `n` is the number of elements,
/// `k` is the number of buckets used in process.
///
/// Space complexity is `O(n + k)`, as it sorts not in-place.


fn insertion_sort<T: PartialOrd+Copy>(arr: &mut[T]) {
    for i in 1..arr.len() {
        let curr = arr[i];
        let mut j = i - 1;
        while arr[j] > curr {
            arr.swap(j+1, j);
            if j == 0 {
                break;
            }
            j-=1;
        }
    }

}

fn main() {

    let mut vec = vec![2,10,8,3,6,3,5];
    insertion_sort(&mut vec);
    println!("{vec:?}");
}