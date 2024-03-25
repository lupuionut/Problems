// 1418. Display Table of Food Orders in a Restaurant
// --------------------------------------------------

use std::collections::HashMap;
use std::collections::HashSet;

impl Solution {
    pub fn display_table(orders: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut tables = HashSet::new();
        let mut foods = HashSet::new();
        let mut table_foods = HashMap::new();

        orders.iter().for_each(|order| {
            let table = &order[1];
            let food = &order[2];
            tables.insert(table.clone());
            foods.insert(food.clone());
            table_foods
                .entry((table, food))
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });

        let mut tables = tables.iter().collect::<Vec<_>>();
        tables.sort_by(|a, b| {
            let a = a.parse::<i32>().unwrap();
            let b = b.parse::<i32>().unwrap();
            a.cmp(&b)
        });
        let mut foods = foods.iter().collect::<Vec<_>>();
        foods.sort();

        let mut ans = vec![];
        let mut first_row = vec!["Table".to_string()];
        for &food in foods.iter() {
            first_row.push(food.clone());
        }
        ans.push(first_row);

        for table in tables {
            let mut row = vec![];
            row.push(table.clone());
            for food in &foods {
                if let Some(val) = table_foods.get(&(&table, &food)) {
                    row.push(val.to_string());
                } else {
                    row.push("0".to_string());
                }
            }
            ans.push(row);
        }

        ans
    }
}
