fn main() {
    let faren_temp = 87.0;
    let celsius: f64 = convert(faren_temp);
    println!("{faren_temp}F converted into Celsius is {celsius}")
}

fn convert(temp: f64) -> f64{
    let converted_temp = (temp - 32.0) / 1.8;
    return converted_temp;
}