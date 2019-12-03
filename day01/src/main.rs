/*
fn main() {
    let mut sum: i32 = 0;

    loop {
        let mut line = String::new();
        if std::io::stdin().read_line(&mut line).expect("read line") == 0 {
            break;
        }

        let mass: i32 = line.trim().parse().expect("parse");

        sum += (mass / 3) - 2;
    }

    println!("fuel requirement: {}", sum);
}
*/

fn main() {
    let mut sum: i32 = 0;

    loop {
        let mut line = String::new();
        if std::io::stdin().read_line(&mut line).expect("read line") == 0 {
            break;
        }

        let mut mass: i32 = line.trim().parse().expect("parse");

        loop {
            let fuel = mass / 3 - 2;
            if fuel <= 0 {
                break;
            }
            sum += fuel;
            mass = fuel;
        }
    }

    println!("fuel requirement: {}", sum);
}
