use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub enum ActionType {
    Register,
    PlaceTroops,
    Attack,
    Occupy,
    Regroup,
    DrawCard,
    UseCard,
    ExchangeCards,
    EndTurn,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ActionResult {

}

#[derive(Debug, Serialize, Deserialize)]
pub struct Action {
    ty: ActionType,
    result: ActionResult,
}