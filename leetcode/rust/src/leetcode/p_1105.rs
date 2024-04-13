// 1105. Filling Bookcase Shelves
// ------------------------------

use std::collections::HashMap;

impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        fn place(
            books: &Vec<Vec<i32>>,
            i: usize,
            curr_height: i32,
            curr_width: i32,
            shelf_width: i32,
            cache: &mut HashMap<(usize, i32, i32), i32>,
        ) -> i32 {
            if i == books.len() {
                return 0;
            }
            if let Some(&val) = cache.get(&(i, curr_width, curr_height)) {
                return val;
            }

            let mut best = i32::MAX;
            let book = &books[i];
            let new_width = curr_width + book[0];
            if curr_width == 0 {
                best =
                    best.min(book[1] + place(books, i + 1, book[1], new_width, shelf_width, cache));
            } else {
                if new_width <= shelf_width {
                    if book[1] > curr_height {
                        best = best.min(
                            (book[1] - curr_height)
                                + place(books, i + 1, book[1], new_width, shelf_width, cache),
                        );
                    } else {
                        best = best.min(place(
                            books,
                            i + 1,
                            curr_height,
                            new_width,
                            shelf_width,
                            cache,
                        ));
                    }
                    best = best.min(place(books, i, 0, 0, shelf_width, cache));
                } else {
                    best = best.min(place(books, i, 0, 0, shelf_width, cache));
                }
            }

            cache.insert((i, curr_width, curr_height), best);
            best
        }

        let mut cache = HashMap::new();
        place(&books, 0, 0, 0, shelf_width, &mut cache)
    }
}
