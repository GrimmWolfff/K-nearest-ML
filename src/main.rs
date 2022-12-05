fn main() {
    let data: Vec<(f32, f32, String)> = vec![
        (1.319, 26.61, "Estonia".to_string()),
        (81.0, 852.0, "Turkey".to_string()),
        (2.9, 11.54, "Armenia".to_string()),
        (9.84, 40.75, "Azerbaijan".to_string()),
        (44.48, 112.0, "Ukraine".to_string()),
        (37.9, 526.0, "Poland".to_string())
    ];
    println!("{:?}", knn(3, &(4.0, 15.08, "Georgia".to_string()), &data));
}

fn knn(k: i32, given_point: &(f32, f32, String), data: &Vec<(f32, f32, String)>) -> Vec<(f32, f32, String)> {
    if data.len() == 0 {
        panic!("DO NOT ENTER EMPTY DATA!")
    }
    let mut distances: Vec<f32> = vec![];
    for p in data {
        distances.push(distance(p, given_point));
    }

    let mut k_nearest: Vec<(f32, f32, String)> = vec![];
    let mut i: usize = 0;

    while i < k as usize {
        let min_idx: usize = get_min_distance(&distances);
        k_nearest.push(data[min_idx].to_owned());
        distances[min_idx] = f32::MAX;
        i += 1;
    }

    k_nearest
}

fn distance(point1: &(f32, f32, String), point2: &(f32, f32, String)) -> f32 {
    f32::sqrt(f32::powf((point1.0 - point2.0) as f32, 2.0) + f32::powf((point1.1 - point2.1) as f32, 2.0))
}

fn get_min_distance(data: &Vec<f32>) -> usize {
    let mut min: f32 = f32::MAX;
    let mut min_idx = 0;
    let mut i = 0;
    while i < data.len() {
        if min > data[i] {
            min = data[i];
            min_idx = i;
        }
        i += 1;
    }
    min_idx as usize
}
