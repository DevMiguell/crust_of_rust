use super::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord,
    {
        // [sorted | unsorted]
        // [7 | 3 | 1 | 9 | 4 | 2 | 8 | 5 | 6 | 10]  // inicial
        // [1 | 3 | 7 | 9 | 4 | 2 | 8 | 5 | 6 | 10]  // após 1ª iteração (menor=1)
        // [1 | 2 | 7 | 9 | 4 | 3 | 8 | 5 | 6 | 10]  // após 2ª iteração (menor=2)
        // [1 | 2 | 3 | 9 | 4 | 7 | 8 | 5 | 6 | 10]  // após 3ª iteração (menor=3)
        // [1 | 2 | 3 | 4 | 9 | 7 | 8 | 5 | 6 | 10]  // após 4ª iteração (menor=4)
        // [1 | 2 | 3 | 4 | 5 | 7 | 8 | 9 | 6 | 10]  // após 5ª iteração (menor=5)
        // [1 | 2 | 3 | 4 | 5 | 6 | 8 | 9 | 7 | 10]  // após 6ª iteração (menor=6)
        // [1 | 2 | 3 | 4 | 5 | 6 | 7 | 9 | 8 | 10]  // após 7ª iteração (menor=7)
        // [1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10]  // após 8ª iteração (menor=8)
        // [1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10]  // final ordenado
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
