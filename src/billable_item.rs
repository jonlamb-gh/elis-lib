use board_dimensions::BoardDimensions;
use lumber::{FobCostReader, LumberType, Props};

use steel_cent::Money;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BillableItem {
    lumber_type: LumberType,
    lumber_props: Props,
    description: String,
    board_dimensions: BoardDimensions,
    quantity: usize,
}

impl BillableItem {
    pub fn new() -> Self {
        Self {
            lumber_type: LumberType::new(),
            lumber_props: Props::default(),
            description: String::from("PIECE DESCRIPTION"),
            board_dimensions: BoardDimensions::new(),
            quantity: 1,
        }
    }

    pub fn lumber_type(&self) -> &LumberType {
        &self.lumber_type
    }

    pub fn lumber_props(&self) -> &Props {
        &self.lumber_props
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn board_dimensions(&self) -> &BoardDimensions {
        &self.board_dimensions
    }

    pub fn quantity(&self) -> usize {
        self.quantity
    }

    pub fn cost<T: FobCostReader>(&self, fob_reader: &T) -> Money {
        let fob_cost = fob_reader.fob_cost(&self.lumber_type);

        (fob_cost * self.board_dimensions.board_feet()) * (self.quantity as f64)
    }
}
