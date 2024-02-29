mod entity;

use entity::Player;

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

