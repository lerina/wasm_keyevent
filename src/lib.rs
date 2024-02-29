use wasm_bindgen::prelude::*;
use web_sys::console::log_1;

mod entity;

use entity::Player;

#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    let player = Player::new(10.0, 20.0);
    let (x, y) = player.get_pos();
    log_1(&format!("player pos: {},{}", x, y).into());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_player() {
        //a
        let p = Player::new(10.0, 20.0);
        //a
        assert_eq!((10.0, 20.0), p.get_pos());
    }
}
