use board_dimensions::BoardDimensions;
use database::LumberFobCostProvider;
use lumber::{LumberType, Props};

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
    pub fn new(lumber_type: LumberType) -> Self {
        Self {
            lumber_type,
            lumber_props: Props::default(),
            description: String::new(),
            board_dimensions: BoardDimensions::new(),
            quantity: 1,
        }
    }

    pub fn lumber_type(&self) -> &LumberType {
        &self.lumber_type
    }

    pub fn set_lumber_type(&mut self, lumber_type: LumberType) {
        self.lumber_type = lumber_type;
    }

    pub fn lumber_props(&self) -> &Props {
        &self.lumber_props
    }

    pub fn set_lumber_props(&mut self, props: Props) {
        self.lumber_props = props;
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn set_description(&mut self, description: &str) {
        self.description = String::from(description);
    }

    pub fn board_dimensions(&self) -> &BoardDimensions {
        &self.board_dimensions
    }

    pub fn set_board_dimensions(&mut self, board_dimensions: BoardDimensions) {
        self.board_dimensions = board_dimensions;
    }

    pub fn quantity(&self) -> usize {
        self.quantity
    }

    // TODO - allow 0?
    pub fn set_quantity(&mut self, quantity: usize) {
        self.quantity = quantity;
    }

    pub fn cost<T>(&self, provider: &T) -> Money
    where
        T: LumberFobCostProvider,
    {
        let fob_cost = provider.fob_cost(&self.lumber_type);

        (fob_cost * self.board_dimensions.board_feet()) * (self.quantity as f64)
    }
}
