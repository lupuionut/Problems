// 2115. Find All Possible Recipes from Given Supplies
// ---------------------------------------------------

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn find_all_recipes(
        recipes: Vec<String>,
        ingredients: Vec<Vec<String>>,
        supplies: Vec<String>,
    ) -> Vec<String> {
        let mut res = supplies.iter().collect::<HashSet<&String>>();
        let mut status: HashMap<&String, (usize, Option<bool>)> = HashMap::new();
        recipes.iter().enumerate().for_each(|(k, r)| {
            status.insert(r, (k, None));
        });
        let mut ans = vec![];

        fn can_prepare<'a>(
            i: usize,
            path: &mut HashSet<&'a String>,
            recipes: &'a Vec<String>,
            ingredients: &Vec<Vec<String>>,
            res: &mut HashSet<&'a String>,
            status: &mut HashMap<&'a String, (usize, Option<bool>)>,
        ) -> bool {
            let necessary = &ingredients[i];
            path.insert(&recipes[i]);
            for j in 0..necessary.len() {
                if !res.contains(&necessary[j]) {
                    if let Some(&(key, stat)) = status.get(&necessary[j]) {
                        match stat {
                            Some(false) => {
                                return false;
                            }
                            Some(true) => {}
                            None => {
                                if path.contains(&recipes[key]) {
                                    return false;
                                }
                                if !can_prepare(key, path, recipes, ingredients, res, status) {
                                    status
                                        .entry(&recipes[i])
                                        .and_modify(|v| *v = (i, Some(false)));
                                    return false;
                                }
                            }
                        }
                    } else {
                        return false;
                    }
                }
            }
            status
                .entry(&recipes[i])
                .and_modify(|v| *v = (i, Some(true)));
            res.insert(&recipes[i]);
            true
        }

        for i in 0..recipes.len() {
            let mut path = HashSet::new();
            if can_prepare(i, &mut path, &recipes, &ingredients, &mut res, &mut status) {
                ans.push(recipes[i].clone());
            }
        }

        ans
    }
}
