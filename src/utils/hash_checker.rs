use lazy_static::lazy_static;
use std::collections::{HashMap, VecDeque};

lazy_static! {
    static ref DIGIT_MAPPING: HashMap<String, char> = {
        let mut map = HashMap::new();
        map.insert(String::from("one"), '1');
        map.insert(String::from("two"), '2');
        map.insert(String::from("thr"), '\0');
        map.insert(String::from("thre"), '\0');
        map.insert(String::from("three"), '3');
        map.insert(String::from("fou"), '\0');
        map.insert(String::from("four"), '4');
        map.insert(String::from("fiv"), '\0');
        map.insert(String::from("five"), '5');
        map.insert(String::from("six"), '6');
        map.insert(String::from("sev"), '\0');
        map.insert(String::from("seve"), '\0');
        map.insert(String::from("seven"), '7');
        map.insert(String::from("eig"), '\0');
        map.insert(String::from("eigh"), '\0');
        map.insert(String::from("eight"), '8');
        map.insert(String::from("nin"), '\0');
        map.insert(String::from("nine"), '9');
        map
    };
}

pub fn check_hash(que: &mut VecDeque<char>, cal_vec: &mut Vec<char>) {
    let x: String = que.iter().cloned().collect();
    match DIGIT_MAPPING.get(&x) {
        Some(x) => match *x {
            '\0' => (),
            _ => {
                cal_vec.push(*x);
                for _i in 1..(que.len() - 1) {
                    que.pop_front();
                }
            }
        },
        None => {
            que.pop_front();
        }
    }
}
