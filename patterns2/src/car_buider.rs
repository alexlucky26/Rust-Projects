#[allow(dead_code)]
#[derive(Debug)]
pub struct Car {
    wheels: u32,
    color: String,
    electric: bool,
}

pub struct CarBuilder {
    wheels: u32,
    color: String,
    electric: bool,
}

impl CarBuilder {
    pub fn new() -> Self {
        Self {
            wheels: 4,
            color: String::from("black"),
            electric: false,
        }
    }

    pub fn wheels(mut self, wheels: u32) -> Self {
        self.wheels = wheels;
        self
    }

    pub fn color(mut self, color: String) -> Self {
        self.color = color;
        self
    }

    pub fn electric(mut self, electric: bool) -> Self {
        self.electric = electric;
        self
    }

    pub fn build(self) -> Car {
        Car {
            wheels: self.wheels,
            color: self.color,
            electric: self.electric,
        }
    }
}

impl Default for CarBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn car_builder_defaults() {
        let car = CarBuilder::new().build();
        assert_eq!(car.wheels, 4);
        assert_eq!(car.color, "black");
        assert_eq!(car.electric, false);
    }

    #[test]
    fn car_builder_customizations() {
        let car = CarBuilder::new()
            .wheels(3)
            .color(String::from("red"))
            .electric(true)
            .build();

        assert_eq!(car.wheels, 3, "Wheels should be set to 3");
        assert_eq!(car.color, "red", "Color should be set to red");
        assert_eq!(car.electric, true, "Electric should be true");
    }

    #[test]
    fn default_builder_test() {
        let builder = CarBuilder::default();
        assert_eq!(builder.wheels, 4, "Default wheels should be 4");
        assert_eq!(builder.color, "black", "Default color should be black");
        assert_eq!(builder.electric, false, "Default electric should be false");
    }
}
