// 2353. Design a Food Rating System
// ---------------------------------

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Debug)]
struct FoodRatings {
    foods: HashMap<String, Food>,
    ratings: HashMap<String, BinaryHeap<(i32, Reverse<String>)>>,
}
#[derive(Debug, PartialEq)]
struct Food {
    name: String,
    cuisine: String,
    rating: i32,
}

impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let f_iter = foods.iter();
        let c_iter = cuisines.iter().zip(f_iter);
        let iter = ratings.iter().zip(c_iter);
        let mut r: HashMap<String, BinaryHeap<(i32, Reverse<String>)>> = HashMap::new();
        let mut f: HashMap<String, Food> = HashMap::new();

        iter.for_each(|(&rating, (cuisine, name))| {
            let item = Food {
                name: name.clone(),
                cuisine: cuisine.clone(),
                rating,
            };
            f.insert(name.clone(), item);

            if let Some(entry) = r.get_mut(cuisine) {
                entry.push((rating, Reverse(name.clone())));
            } else {
                let mut heap = BinaryHeap::new();
                heap.push((rating, Reverse(name.clone())));
                r.insert(cuisine.clone(), heap);
            }
        });

        Self {
            foods: f,
            ratings: r,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        if let Some(f) = self.foods.get_mut(&food) {
            f.rating = new_rating;
            if let Some(cuisine) = self.ratings.get_mut(&f.cuisine) {
                cuisine.push((new_rating, Reverse(food)));
            }
        }
    }

    fn highest_rated(&mut self, cuisine: String) -> String {
        loop {
            let mut cuisine = self.ratings.get_mut(&cuisine).unwrap();
            if let Some((value, Reverse(name))) = cuisine.peek() {
                let food = self.foods.get(name).unwrap();
                if food.rating == *value {
                    return food.name.clone();
                } else {
                    cuisine.pop();
                }
            }
        }
    }
}
