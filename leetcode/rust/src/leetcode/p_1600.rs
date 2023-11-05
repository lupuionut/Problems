// 1600. Throne Inheritance
// ------------------------

use std::collections::HashMap;

struct ThroneInheritance {
    king: String,
    relations: HashMap<String, Vec<String>>,
    alive: HashMap<String, bool>,
    order: Vec<String>,
}

impl ThroneInheritance {
    fn new(kingName: String) -> Self {
        let mut relations: HashMap<String, Vec<String>> = HashMap::new();
        let mut alive: HashMap<String, bool> = HashMap::new();
        let mut order: Vec<String> = vec![];
        alive.insert(kingName.clone(), true);

        Self {
            king: kingName,
            relations,
            alive,
            order,
        }
    }

    fn birth(&mut self, parent_name: String, child_name: String) {
        self.relations
            .entry(parent_name)
            .and_modify(|childs| childs.push(child_name.clone()))
            .or_insert(vec![child_name.clone()]);
        self.alive.insert(child_name.clone(), true);
    }

    fn death(&mut self, name: String) {
        self.alive.insert(name, false);
    }

    fn get_inheritance_order(&mut self) -> Vec<String> {
        self.order = vec![];

        if *self.alive.get(&self.king).unwrap() == true {
            self.order.push(self.king.clone());
        }

        fn dfs(
            parent: String,
            relations: &HashMap<String, Vec<String>>,
            alive: &HashMap<String, bool>,
            order: &mut Vec<String>,
        ) {
            if let Some(kids) = relations.get(&parent) {
                for kid in kids.iter() {
                    let status = alive.get(kid).unwrap();
                    if *status == true {
                        order.push(kid.to_string());
                    }
                    dfs(kid.clone(), relations, alive, order);
                }
            }
        }
        dfs(
            self.king.clone(),
            &self.relations,
            &self.alive,
            &mut self.order,
        );
        self.order.clone()
    }
}
