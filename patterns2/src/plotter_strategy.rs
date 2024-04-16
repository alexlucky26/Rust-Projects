use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlotError {
    #[error("Unexpected plot response.")]
    UnexpectedError,
    #[error("The plot settings are wrong.")]
    WrongPlotSettings,
}
pub trait Plot {
    fn plot_element(&self) -> Result<bool, PlotError>;
}

pub struct Piece {
    pub name: String,
    pub perimeter_polyline: Vec<(i32, i32)>,
}
pub struct Model {
    pub name: String,
    pub pieces: Vec<Piece>,
}

impl Plot for Piece {
    fn plot_element(&self) -> Result<bool, PlotError> {
        println!("Plotting piece: {}", &self.name);
        for coord in &self.perimeter_polyline {
            println!("Plot point at x: {}, y: {}", coord.0, coord.1);
        }
        Ok(true)
    }
}

impl Plot for Model {
    fn plot_element(&self) -> Result<bool, PlotError> {
        println!("Plotting model: {}", &self.name);
        for piece in &self.pieces {
            piece.plot_element()?;
        }
        Ok(true)
    }
}

pub struct Plotter<PlotElement: Plot> {
    pub src: PlotElement,
}

impl<PlotElement: Plot> Plotter<PlotElement> {
    pub fn new(elem: PlotElement) -> Self {
        Self { src: elem }
    }
    pub fn execute_plot(&self) -> Result<bool, PlotError> {
        self.src.plot_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plotter_strategy::{Model, Piece, Plotter};

    #[test]
    fn test_piece_plotting() {
        let piece = Piece {
            name: "Triangle".to_string(),
            perimeter_polyline: vec![(0, 0), (1, 0), (0, 1)],
        };
        let plotter = Plotter::new(piece);
        assert_eq!(plotter.execute_plot().unwrap(), true);
    }

    #[test]
    fn test_model_plotting() {
        let piece1 = Piece {
            name: "Square".to_string(),
            perimeter_polyline: vec![(0, 0), (1, 0), (1, 1), (0, 1)],
        };
        let piece2 = Piece {
            name: "Rectangle".to_string(),
            perimeter_polyline: vec![(0, 0), (2, 0), (2, 1), (0, 1)],
        };
        let model = Model {
            name: "Two Shapes".to_string(),
            pieces: vec![piece1, piece2],
        };
        let plotter = Plotter::new(model);
        assert_eq!(plotter.execute_plot().unwrap(), true);
    }
}
