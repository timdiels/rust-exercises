use std::fmt;

fn main() {
    let mut items: Vec<i32> = vec![];
    inplace_sort(&mut items);

    let mut items = vec![3, 1, 8, 1, 0, 5];
    inplace_sort(&mut items);
    assert_eq!(items, [0, 1, 1, 3, 5, 8]);

    let mut items = vec!["d", "b", "f", "b", "a", "c"];
    inplace_sort(&mut items);
    assert_eq!(items, ["a", "b", "b", "c", "d", "f"]);
}

// Exercise: Write a program that sorts a vector of integers/strings “in place” using a
// handwritten selection sort: find the smallest number, move it to the front, then find the
// smallest of the rest, move it to second place etc.
fn inplace_sort<T: Ord + fmt::Debug>(items: &mut Vec<T>) {
    if items.is_empty() {
        return;
    }

    for i in 0..(items.len() - 1) {
        let (index_of_min, _) = items
            .iter()
            // TODO placing skip before enumerate doesn't work, seems to not skip then, why?
            .enumerate()
            .skip(i)
            .min_by(|(_, item1), (_, item2)| (item1).cmp(item2))
            .expect("Items is not empty");

        // Seems mem::swap on the items is impossible as it would require 2 iter_mut => compile
        // error "second mutable borrow"
        items.swap(i, index_of_min);
    }
}
