use super::Sorter;

pub struct QuickSort;

fn quicksort<T: Ord>(slice: &mut [T]) {
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0, 1);
            }
            return;
        }
        _ => {}
    }

    let pivot_idx = {
        let (pivot, rest) = slice.split_first_mut().expect("slice is non-empty");

        let mut left = 0;
        let mut right = rest.len();
        while left < right {
            if &rest[left] <= pivot {
                left += 1;
            } else {
                right -= 1;
                if &rest[right] <= pivot {
                    rest.swap(left, right);
                    left += 1;
                }
            }
        }
        left
    };

    slice.swap(0, pivot_idx);

    let (left, right) = slice.split_at_mut(pivot_idx);
    quicksort(left);
    quicksort(&mut right[1..]);
}

impl Sorter for QuickSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [unsorted | pivot | sorted]
        // [7 | 3 | 1 | 9 | 4 | 2 | 8 | 5 | 6 | 10]  // inicial
        // pivot=7: [3 | 1 | 4 | 2 | 5 | 6 | 7 | 9 | 8 | 10]  // após particionar (<=7 | 7 | >7)
        //   esquerda [3,1,4,2,5,6]: pivot=3 -> [1 | 2 | 3 | 4 | 5 | 6]
        //     [1,2]: pivot=1 -> [1 | 2] -> [1 | 2]
        //     [4,5,6]: pivot=4 -> [4 | 5 | 6] -> [4 | 5 | 6]
        //   direita [9,8,10]: pivot=9 -> [8 | 9 | 10]
        // [1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10]  // final ordenado
        quicksort(slice);
    }
}

#[test]
fn it_works() { 
    let mut things = vec![4, 2, 3, 1];
    QuickSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4])
}
