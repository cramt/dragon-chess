use core::board::MoveType;
use core::board_controller::BoardController;
use core::board_controller::PieceColor::White;
use core::pieces::vector3::Vector3;
use core::pieces::Piece;
use css_in_rust::Style;
use std::collections::HashMap;
use yew::prelude::*;

pub struct Board {
    link: ComponentLink<Self>,
    board_controller: BoardController,
    upper_white: Style,
    middle_white: Style,
    lower_white: Style,
    upper_black: Style,
    middle_black: Style,
    lower_black: Style,
    chessboard: Style,
    possible_move: Style,
    possible_capture: Style,
    possible_remote_capture: Style,
    empty_style: Style,
    white_piece: Style,
    black_piece: Style,
}

pub enum Msg {
    Click(Vector3),
}

impl Component for Board {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            board_controller: BoardController::default(),
            upper_white: Style::create(
                "upper-white",
                r#"
                float: left;
                width: 32px;
                height: 32px;
                background-color: #d6d6d6;
                font-size: 20px;
                text-align: center;
                display: table-cell;
                vertical-align: middle;
            "#,
            )
            .unwrap(),
            upper_black: Style::create(
                "upper-black",
                r#"
                float: left;
                width: 32px;
                height: 32px;
                background-color: #62b0ff;
                font-size: 20px;
                text-align: center;
                display: table-cell;
                vertical-align: middle;
            "#,
            )
            .unwrap(),
            middle_white: Style::create(
                "middle-white",
                r#"
                float: left;
                width: 32px;
                height: 32px;
                background-color: #d2ae71;
                font-size: 20px;
                text-align: center;
                display: table-cell;
                vertical-align: middle;
            "#,
            )
            .unwrap(),
            middle_black: Style::create(
                "middle-black",
                r#"
                float: left;
                width: 32px;
                height: 32px;
                background-color: #009d00;
                font-size: 20px;
                text-align: center;
                display: table-cell;
                vertical-align: middle;
            "#,
            )
            .unwrap(),
            lower_white: Style::create(
                "lower-white",
                r#"
                float: left;
                width: 32px;
                height: 32px;
                background-color: #8e4700;
                font-size: 20px;
                text-align: center;
                display: table-cell;
                vertical-align: middle;
            "#,
            )
            .unwrap(),
            lower_black: Style::create(
                "lower-black",
                r#"
                float: left;
                width: 32px;
                height: 32px;
                background-color: #db2213;
                font-size: 20px;
                text-align: center;
                display: table-cell;
                vertical-align: middle;
            "#,
            )
            .unwrap(),
            chessboard: Style::create(
                "chessboard",
                r#"
                width: 384px;
                height: 256px;
                margin: 20px;
                border: 25px solid #333;
            "#,
            )
            .unwrap(),
            possible_move: Style::create(
                "possible-move",
                r#"
                background-color: rgba(252, 211, 3, 0.5);
                height: 100%;
                border-radius: 50%;
            "#,
            )
            .unwrap(),
            possible_capture: Style::create(
                "possible-capture",
                r#"
                background-color: rgba(252, 3, 3, 0.5);
                height: 100%;
                border-radius: 50%;
            "#,
            )
            .unwrap(),
            possible_remote_capture: Style::create(
                "possible-remote-capture",
                r#"
                background-color: rgba(252, 3, 201, 0.5);
                height: 100%;
                border-radius: 50%;
            "#,
            )
            .unwrap(),
            empty_style: Style::create(
                "empty",
                r#"
                display: block;
            "#,
            )
            .unwrap(),
            black_piece: Style::create(
                "black-piece",
                r#"
                color: black;
            "#,
            )
            .unwrap(),
            white_piece: Style::create(
                "white-piece",
                r#"
                color: white;
            "#,
            )
            .unwrap(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Click(vector) => self.board_controller.select(vector),
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let (colour, check) = self.board_controller.check_mate();
        let check_string = match check {
            _Free => String::new(),
            _Check => format!(
                "{} has been put in check",
                colour.to_string().to_lowercase()
            ),
            _CheckMate => format!(
                "{} has been put in checkmate, {} wins",
                colour.to_string().to_lowercase(),
                colour.flip().to_string().to_lowercase()
            ),
        };
        html! {
            <div>
                <p>{format!("{}'s turn", colour.to_string().to_lowercase())}</p>
                <p>{check_string}</p>
                {self.render_board()}
            </div>
        }
    }
}

impl Board {
    fn render_board(&self) -> Vec<Html> {
        let info = self
            .board_controller
            .pieces_info()
            .into_iter()
            .map(|x| (*x.get_position(), x))
            .collect::<HashMap<Vector3, &Box<dyn Piece>>>();
        let possible_moves = self.board_controller.possible_moves();
        vec![
            vec![&self.lower_white, &self.lower_black],
            vec![&self.middle_white, &self.middle_black],
            vec![&self.upper_white, &self.upper_black],
        ]
        .into_iter()
        .enumerate()
        .map(|(i, style)| {
            html! {
                <div class=self.chessboard.clone()>
                    {self.render_cell(style, i as i32, &info, &possible_moves)}
                </div>
            }
        })
        .collect()
    }

    fn render_cell(
        &self,
        style: Vec<&Style>,
        z: i32,
        info: &HashMap<Vector3, &Box<dyn Piece>>,
        possible_moves: &HashMap<Vector3, MoveType>,
    ) -> Vec<Html> {
        let mut vec = vec![];
        for y in 0..8 {
            for x in 0..12 {
                let style = style[(((y % 2 == 0) as usize + x) % 2 == 0) as usize].clone();
                let vector = Vector3::new(x as i32, y, z);
                let (str, piece_style) = match info.get(&vector) {
                    Some(piece) => (
                        piece.get_char().to_string(),
                        if self.board_controller.color_of_piece(piece) == White {
                            self.white_piece.clone()
                        } else {
                            self.black_piece.clone()
                        },
                    ),
                    None => (String::new(), self.empty_style.clone()),
                };
                let move_style = match possible_moves.get(&vector) {
                    Some(move_type) => match *move_type {
                        MoveType::RemoteCapture => self.possible_remote_capture.clone(),
                        MoveType::Capture => self.possible_capture.clone(),
                        MoveType::Move => self.possible_move.clone(),
                    },
                    None => self.empty_style.clone(),
                };
                vec.push(
                    html! {<div class=style onclick=self.link.callback(move |_|Msg::Click(vector))>
                        <div class=move_style>
                            <div class=piece_style>{str}</div>
                        </div>
                    </div>},
                )
            }
        }
        vec
    }
}
