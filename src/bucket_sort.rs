/// Sort a slice using bucket sort algorithm.
///
/// Time complexity is `O(n + k)` on average, where `n` is the number of elements,
/// `k` is the number of buckets used in process.
///
/// Space complexity is `O(n + k)`, as it sorts not in-place.
fn insertion_sort<T>(arr: &mut[T])
    where T: PartialOrd + Copy
{
    for i in 1..arr.len() {
        let curr = arr[i];
        let mut j = i-1;
        while arr[j] > curr {
            arr.swap(j+1, j);
            if j == 0 {
                break;
            }
            j-=1;
        }
    }
}

fn bucket_sort(arr: &[usize]) -> Vec<usize> {
    if arr.is_empty() {
        return vec![];
    }

    let max = *arr.iter().max().unwrap();
    let len = arr.len();
    let mut buckets = vec![vec![]; len+1];

    for x in arr {
        let cal = len * *x/max;
        buckets[cal].push(*x);
    }

    for bucket in buckets.iter_mut() {
        insertion_sort(bucket);
    }

    let mut result = vec![];
    for bucket in buckets {
        for x in bucket {
            result.push(x);
        }
    }
    result
}


fn main() {

    let mut vec = vec![9,4,6,2,5,3,17];
    vec = bucket_sort(&mut vec);
    println!("{vec:#?}");
}

