use std::time::SystemTime;

pub fn random() -> f64 {
    return SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as f64 % 1000.0 / 1000.0;
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

pub fn sample<T: Clone>(data: &Vec<T>, count: usize) -> Option<Vec<T>> {
    if data.len() < count {
        return None;
    }
    let mut places = (0..data.len() as i32).collect::<Vec<i32>>();
    let mut vec = Vec::<T>::new();
    for _ in 0..count {
        let rnd = rng(0, places.len() as i32) as usize;
        vec.push(data[places[rnd] as usize].clone());
        places.remove(rnd);
    }
    return Some(vec);
}

pub fn shuffle<T: Clone>(data: &Vec<T>) -> Vec<T> {
    if data.len() == 0 {
        return vec![];
    }
    return sample(data, data.len()).unwrap();
}