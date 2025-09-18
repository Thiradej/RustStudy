struct Fibonacci {
    current: u64,
    next: u64,
}

impl Fibonacci {
    fn new() -> Fibonacci {
        Fibonacci {
            current: 0,
            next: 1,
        }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<Self::Item> {
        let temp = self.current;
        self.current = self.next;
        self.next += temp;
        Some(temp)
    }
}


fn main() {
    let fib = Fibonacci::new();
    for (i, val) in fib.take(10).enumerate(){
        println!("Fibonacci {} = {}", i+1, val)
    }
}


// type WeatherData = (f64, u32, bool);


// fn warmest_period(data: &[WeatherData]) -> &[WeatherData] {
//     data.windows(3)
//         .max_by(|a, b| {
//             let avg_a: f64 = a.iter().map(|x| x.0).sum::<f64>() / 3.0;
//             let avg_b: f64 = b.iter().map(|x| x.0).sum::<f64>() / 3.0;
//             avg_a.partial_cmp(&avg_b).unwrap()
//             })
//         .unwrap_or(&[])
// }

// fn coldest_day(data: &[WeatherData]) -> usize {
//     data.iter()
//         .enumerate()
//         .min_by(|(_, a), (_, b)| a.0.partial_cmp(&b.0).unwrap())
//         .map(|(idx, _)| idx)
//         .unwrap_or(0)

// }
// fn predict_rain(day_data: &WeatherData) -> bool {
//     let (temp, humidity, _) =  *day_data ;
//     (0.005 * humidity as f64 + 0.02 * temp - 1.0) > 0.5
// }

// fn count_rainy_days(data: &[WeatherData]) -> usize {
//     data.iter().filter(|&x| x.2).count()
// }

// fn main() {
//     let weather_data: Vec<WeatherData> = vec![
//         (25.0, 65, false), (26.2, 70, false), (24.8, 62,false), (23.5, 78, true), (22.1, 82, true), (20.7,85, true), (21.3,80, true), (22.8, 73, false), (24.0, 68, false), (25.5, 60, false),
//         (27.1, 55, false), (28.3, 52, false), (27.9,58,false), (26.6,64,false), (25.2, 70, true), (23.8, 75, true), (22.4, 80, true), (21.0, 83, true), (20.5, 86, true), (21.8, 82, true),
//         (23.2, 77, false), (24.5, 70, false), (25.8, 63, false), (26.4,58,false), (27.0, 53, false), (26.7, 56, false), (25.3, 62,false), (24.9, 68, true),(23.1, 74, true), (21.7, 79, true)
//     ];
//     println!("\n--- lab 2 output ---");
//     println!("Warmest 3-day period: {:?}", warmest_period(&weather_data));
//     println!("Coldest day: {}", coldest_day(&weather_data));
//     println!("Number of rainy days: {}", count_rainy_days(&weather_data));
//     println!("Will it rain  on the first day? {}", predict_rain(&weather_data[0]));
// }