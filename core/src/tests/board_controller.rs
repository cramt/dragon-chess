#[cfg(test)]
mod board_controller {
    use crate::board_controller::BoardController;
    use crate::pieces::vector3::Vector3;

    #[test]
    fn generic_capture() {
        println!("a1");
        let mut controller = BoardController::default();
        println!("a2");
        controller.select(Vector3::new(6, 0, 2));
        println!("a3");
        let (white_dragon, _) = controller.piece_info(Vector3::new(6, 0, 2)).unwrap();
        println!("a4");
        assert_eq!("dragon", white_dragon.get_name());
        println!("a5");
        controller.select(Vector3::new(0, 6, 2));
        println!("a6");
        let (white_dragon, _) = controller.piece_info(Vector3::new(0, 6, 2)).unwrap();
        println!("a7");
        assert_eq!("dragon", white_dragon.get_name());
    }
}
