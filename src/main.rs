const IMAGE_WIDTH: i64 = 256;
const IMAGE_HEIGHT: i64 = 256;

fn main() {
    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;

            let red = (255.999 * r) as u64;
            let green = (255.999 * g) as u64;
            let blue = (255.999 * b) as u64;

            println!("{red} {green} {blue}");
        }
    }
}
