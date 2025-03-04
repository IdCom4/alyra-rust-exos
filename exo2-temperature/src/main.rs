fn main() {
    let temperatures : [f32; 7] = [22.0, 19.5, 21.0, 23.5, 20.0, 18.0, 25.0];
    let average = calculate_average(&temperatures);
    println!("Moyenne température : {}°C", average);
}

fn calculate_average(temperatures: &[f32]) -> f32 {
    let mut sum = 0.0;
    for temperature in temperatures {
        sum += temperature;
    }
    sum / temperatures.len() as f32
}