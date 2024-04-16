use super::cloth_entity::{BasicClothEntity, ClothEntity};
use super::plot_error::PlotError;

pub struct Sleeve {
    entity: BasicClothEntity,
}

impl Sleeve {
    pub fn new(entity: BasicClothEntity) -> Self {
        Self { entity }
    }
}
impl ClothEntity for Sleeve {
    fn send_to_plotter(&self, message: String) -> Result<bool, PlotError> {
        self.entity
            .send_to_plotter(format!("{} Sleeve was plotted.", message))?;
        Ok(true)
    }
}

pub struct Dress {
    entity: Sleeve,
}

impl Dress {
    pub fn new(entity: Sleeve) -> Self {
        Self { entity }
    }
}
impl ClothEntity for Dress {
    fn send_to_plotter(&self, message: String) -> Result<bool, PlotError> {
        self.entity
            .send_to_plotter(format!("Dress {} is plotting...", message))?;
        println!("Entire Plot for Dress {} is finished.", message);
        Ok(true)
    }
}
