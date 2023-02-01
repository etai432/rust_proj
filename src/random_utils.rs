use std::time::SystemTime;

pub fn random() -> f64 {
    return SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as f64 % 10000.0 / 10000.0;
}

pub fn rng(min: i32, max:i32) -> i32 {
    return min + (random() * (max - min) as f64) as i32;
}

pub fn choise<T>(arr: &Vec<T>) -> Option<&T> {
    if arr.len() == 0 {
        return None;
    }
    return Some(&arr[rng(0, arr.len() as i32) as usize]);
}

//implement sample function