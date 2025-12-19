use super::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [sorted | unsorted]
        for unsorted in 0..slice.len() {
            let smallest_in_rest = slice[unsorted..]
                .iter()
                .enumerate()
                .min_by_key(|&(_, v)| v) 
                .map(|(i, _)| unsorted + i)
                .expect("slice is non-empty");


            // or 
            
            // let mut smallest_in_rest__2 = unsorted;
            // for i in unsorted+1..slice.len() {
            //     if slice[i] < slice[smallest_in_rest__2] {
            //         smallest_in_rest__2 = i;
            //     }
            // }

            // assert_eq!(smallest_in_rest, smallest_in_rest__2);

            if unsorted != smallest_in_rest {
                slice.swap(unsorted, smallest_in_rest);
            }
        }
    }
}

#[test]
fn it_works() { 
    let mut things = vec![4, 2, 3, 1];
    SelectionSort.sort(&mut things);
    assert_eq!(things, &[1, 2, 3, 4])
}
