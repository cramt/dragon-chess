use core::board::MoveType;
use core::board_controller::BoardController;
use core::board_controller::PieceColor::White;
use core::pieces::Piece;
use core::vector3::Vector3;
use css_in_rust::Style;
use std::collections::HashMap;
use yew::prelude::*;

pub struct Board {
    link: ComponentLink<Self>,
    board_controller: BoardController,
    upper_white: String,
    middle_white: String,
    lower_white: String,
    upper_black: String,
    middle_black: String,
    lower_black: String,
    chessboard: String,
    possible_move: String,
    possible_capture: String,
    possible_remote_capture: String,
    empty_style: String,
    white_piece: String,
    black_piece: String,
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
            upper_white: "upper-white".to_string(),
            upper_black: "upper-black".to_string(),
            middle_black: "middle-black".to_string(),
            lower_black: "lower-black".to_string(),
            chessboard: "chessboard".to_string(),
            possible_move: "possible-move".to_string(),
            possible_capture: "possible-capture".to_string(),
            possible_remote_capture: "possible-remote-capture".to_string(),
            empty_style: String::new(),
            white_piece: "white-piece".to_string(),
            middle_white: "middle-white".to_string(),
            lower_white: "lower-white".to_string(),
            black_piece: "black-piece".to_string(),
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
                <style>{include_str!("board.css")}</style>
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
        style: Vec<&String>,
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
