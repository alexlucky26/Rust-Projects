use patterns2::plotter_strategy::{Model, Piece, Plotter};

fn main() {
    let polylines = vec![(0, 0), (0, 3), (3, 3), (3, 0)];
    let square_piece = Piece {
        name: "Square".to_string(),
        perimeter_polyline: polylines,
    };

    let piece_plotter = Plotter::new(square_piece);
    let result = piece_plotter.execute_plot();
    println!("Plotting piece result: {:?}", result);

    let polylines_model = vec![(0, 0), (0, 4), (4, 4), (4, 0)];
    let model_piece = Piece {
        name: "Square Model".to_string(),
        perimeter_polyline: polylines_model,
    };

    let model = Model {
        name: "Dress".to_string(),
        pieces: vec![model_piece],
    };

    let model_plotter = Plotter::new(model);
    let result_model = model_plotter.execute_plot();
    println!("Plotting model result: {:?}", result_model);
}
