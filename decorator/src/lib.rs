pub mod cloth_entity;
pub mod decorator;
pub mod decorator_dyn;
pub mod plot_error;

#[cfg(test)]
mod tests {
    use super::cloth_entity::ClothEntity;
    use super::decorator_dyn::{DressDyn, SleeveDyn};
    use super::plot_error::PlotError;

    struct MockClothEntity;
    impl ClothEntity for MockClothEntity {
        fn send_to_plotter(&self, message: String) -> Result<bool, PlotError> {
            Ok(message.contains("Mock"))
        }
    }

    #[test]
    fn test_sleeve_dyn_send_to_plotter_success() {
        let mock_cloth_entity = Box::new(MockClothEntity {});
        let sleeve = SleeveDyn::new(mock_cloth_entity);
        let result = sleeve.send_to_plotter("Mock Sleeve was plotted.".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }

    #[test]
    fn test_dress_dyn_send_to_plotter_success() {
        let mock_cloth_entity = Box::new(MockClothEntity {});
        let dress = DressDyn::new(mock_cloth_entity);
        let result = dress.send_to_plotter("Mock Dress is plotting...".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), true);
    }
}
