use decorator::cloth_entity::{BasicClothEntity, ClothEntity};
use decorator::decorator_dyn::{DressDyn, SleeveDyn};

fn main() {
    let sleeve = SleeveDyn::new(Box::new(BasicClothEntity));
    let dress = DressDyn::new(Box::new(sleeve));
    let dress_name = "Jade Dress".to_string();
    dress
        .send_to_plotter(dress_name)
        .expect("Plot error happened.");
}
