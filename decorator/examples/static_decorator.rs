use decorator::cloth_entity::{BasicClothEntity, ClothEntity};
use decorator::decorator::{Dress, Sleeve};

fn main() {
    let sleeve = Sleeve::new(BasicClothEntity);
    let dress = Dress::new(sleeve);
    let dress_name = "Lunar Dress".to_string();
    dress
        .send_to_plotter(dress_name)
        .expect("Plot error happened.");
}
