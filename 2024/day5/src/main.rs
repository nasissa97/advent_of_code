use std::borrow::Borrow;
use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::collections::{HashMap, hash_map::Entry};

fn is_valid(updates: &Vec<u64>, task_prereq: &HashMap<u64, Vec<u64>>) -> bool {
    for (i, task) in updates.iter().enumerate() {
        for order in i+1..updates.len() {
            // let check_task = updates.get(i).unwrap();
            let prereq = updates.get(order).unwrap();
            match task_prereq.contains_key(task) {
                true => {
                    if task_prereq.get(task).unwrap().contains(prereq) {
                        return false
                    }
                }
                false => continue
            }
        }
   } 
    true
}

fn fix_order(updates: &mut Vec<u64>, task_prereq: &HashMap<u64, Vec<u64>>)  {
    println!("Current: {:?}", updates);
    let n = updates.len();
    let mut i = 0;
    while i < n {
        // let check_task = updates.get(i).unwrap();
        let mut swapped = false;
        let task = *updates.get(i).unwrap();
        for j in i+1..n {
            let prereq = updates.get(j).unwrap().clone();
            match task_prereq.contains_key(&task) {
                true => {
                    if task_prereq.get(&task).unwrap().contains(&prereq) {
                        updates.swap(i, j);
                        swapped = true
                    }
                }
                false => continue
            }
        }
        if !swapped {
           i += 1; 
        }
    }
    println!("Updated: {:?}", &updates);
}

fn main() {
    println!("Hello, world!");
    let file = File::open("data/data.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut task_prereq: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut updates: Vec<Vec<u64>> = Vec::new();
    for line in reader.by_ref().lines() {
        let line = line.unwrap();
        if line == "" {
            break;
        }
        let tasks: Vec<u64> = line
            .split('|')
            .map(|t| t.parse::<u64>().unwrap())
            .collect();
        match task_prereq.entry(tasks.get(1).unwrap().clone()) {
            Entry::Vacant(e) => {
                e.insert(vec![tasks.get(0).unwrap().clone()]);
            },
            Entry::Occupied(mut e) => {
                e.get_mut().push(tasks.get(0).unwrap().clone());
            }
        }
    }
    // println!("Prereq: \n{:?}", task_prereq);

    for line in reader.lines() {
        let line: Vec<u64> = line
            .unwrap()
            .split(",")
            .map(|t| t.parse::<u64>().unwrap())
            .collect();

        updates.push(line)
    }

    let mut ans1 = 0;
    let mut ans2 = 0;
    for update in &mut updates {
        if is_valid(&update, &task_prereq) {
            let middle = update.len() / 2;
            let middle_val = update.get(middle).unwrap();
            ans1 += middle_val;
        } else {
            fix_order(update, &task_prereq);
            let middle = update.len() / 2;
            let middle_val = update.get(middle).unwrap();
            ans2 += middle_val;
        }
    }
    println!("Part1 Answer: {}\nPart2 Answer: {}", ans1, ans2);
}
