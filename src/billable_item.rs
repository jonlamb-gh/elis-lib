use board_dimensions::BoardDimensions;
use lumber_types::LumberType;
use steel_cent::SmallMoney;

#[derive(Clone, Debug)]
pub struct BillableItem {
    lumber_type: LumberType,
    description: String,
    board_dimensions: BoardDimensions,
    quantity: usize,
}

impl BillableItem {
    pub fn new() -> Self {
        Self {
            lumber_type: LumberType::DouglasFir,
            description: String::from("PIECE DESCRIPTION"),
            board_dimensions: BoardDimensions::new(),
            quantity: 1,
        }
    }

    pub fn lumber_type(&self) -> &LumberType {
        &self.lumber_type
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

    pub fn cost(&self) -> SmallMoney {
        let fob_price = self.lumber_type.fob_price();

        (fob_price * self.board_dimensions.board_feet()) * (self.quantity as f64)
    }
}
