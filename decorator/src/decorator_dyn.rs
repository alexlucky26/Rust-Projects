use super::cloth_entity::ClothEntity;
use super::plot_error::PlotError;

pub struct SleeveDyn {
    entity: Box<dyn ClothEntity>,
}

impl SleeveDyn {
    pub fn new(entity: Box<dyn ClothEntity>) -> Self {
        Self { entity }
    }
}
impl ClothEntity for SleeveDyn {
    fn send_to_plotter(&self, message: String) -> Result<bool, PlotError> {
        self.entity
            .send_to_plotter(format!("{} Sleeve was plotted.", message))?;
        Ok(true)
    }
}

pub struct DressDyn {
    entity: Box<dyn ClothEntity>,
}

impl DressDyn {
    pub fn new(entity: Box<dyn ClothEntity>) -> Self {
        Self { entity }
    }
}
impl ClothEntity for DressDyn {
    fn send_to_plotter(&self, message: String) -> Result<bool, PlotError> {
        match self
            .entity
            .send_to_plotter(format!("Dress {} is plotting...", message))
        {
            Ok(_) => {
                println!("Entire Plot for Dress {} is finished.", message);
            }
            Err(e) => {
                println!("Error happened: {}", e);
            }
        }
        Ok(true)
    }
}
