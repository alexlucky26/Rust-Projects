use patterns2::car_buider::CarBuilder;

fn main() {
    let car = CarBuilder::new()
        .color("red".to_string())
        .electric(true)
        .build();
    println!("The built car:\n{:#?}", car);
}
