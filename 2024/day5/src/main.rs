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
    println!("Prereq: \n{:?}", task_prereq);

    for line in reader.lines() {
        let line: Vec<u64> = line
            .unwrap()
            .split(",")
            .map(|t| t.parse::<u64>().unwrap())
            .collect();

        updates.push(line)
    }

    let mut ans = 0;
    for update in updates {
        if is_valid(&update, &task_prereq) {
            let middle = update.len() / 2;
            let middle_val = update.get(middle).unwrap();
            ans += middle_val;

        }
    }
    println!("Answer: {}", ans);
}
