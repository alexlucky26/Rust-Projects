use super::plot_error::PlotError;
pub trait ClothEntity {
    fn send_to_plotter(&self, name: String) -> Result<bool, PlotError>;
}

pub struct BasicClothEntity;
impl ClothEntity for BasicClothEntity {
    fn send_to_plotter(&self, message: String) -> Result<bool, PlotError> {
        println!("Entity Plot Result: {}", message);
        Ok(true)
    }
}
