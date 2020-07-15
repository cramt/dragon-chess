mod board_controller;
mod board_piece;

extern crate wasm_bindgen_test;
extern crate wasm_bindgen;
extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;
use crate::pieces::vector3::Vector3;
use crate::board_controller::BoardController;
use std::sync::Mutex;

mod grid;
mod tests;
mod player;
mod board;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate maplit;

mod pieces;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use css_in_rust::Style;

struct Board {
    link: ComponentLink<Self>,
    board_controller: BoardController,
    upper_white: Style,
    middle_white: Style,
    lower_white: Style,
    upper_black: Style,
    middle_black: Style,
    lower_black: Style,
    chessboard: Style,
}

enum Msg {}

impl Component for Board {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            board_controller: BoardController::default(),
            upper_white: Style::create("upper-white", r#"
                float: left;
                width: 80px;
                height: 80px;
                background-color: #d6d6d6;
                font-size: 50px;
                text-align: center;
                display: table-cell;
                vertical-align: middle;
            "#).unwrap(),
            upper_black: Style::create("upper-black", r#"
                float: left;
                width: 80px;
                height: 80px;
                background-color: #62b0ff;
                font-size: 50px;
                text-align: center;
                display: table-cell;
                vertical-align: middle;
            "#).unwrap(),
            middle_white: Style::create("middle-white", r#"
                float: left;
                width: 80px;
                height: 80px;
                background-color: #d2ae71;
                font-size: 50px;
                text-align: center;
                display: table-cell;
                vertical-align: middle;
            "#).unwrap(),
            middle_black: Style::create("middle-black", r#"
                float: left;
                width: 80px;
                height: 80px;
                background-color: #009d00;
                font-size: 50px;
                text-align: center;
                display: table-cell;
                vertical-align: middle;
            "#).unwrap(),
            lower_white: Style::create("lower-white", r#"
                float: left;
                width: 80px;
                height: 80px;
                background-color: #8e4700;
                font-size: 50px;
                text-align: center;
                display: table-cell;
                vertical-align: middle;
            "#).unwrap(),
            lower_black: Style::create("lower-black", r#"
                float: left;
                width: 80px;
                height: 80px;
                background-color: #db2213;
                font-size: 50px;
                text-align: center;
                display: table-cell;
                vertical-align: middle;
            "#).unwrap(),
            chessboard: Style::create("chessboard", r#"
                width: 960px;
                height: 640px;
                margin: 20px;
                border: 25px solid #333;
            "#).unwrap()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
            {self.render_board()}
            </div>
        }
    }
}

impl Board {
    fn render_board(&self) -> Vec<Html> {
        vec![
            vec![&self.lower_white, &self.lower_black],
            vec![&self.middle_white, &self.middle_black],
            vec![&self.upper_white, &self.upper_black]
        ].into_iter().map(|style|{
            html!{
                <div class=self.chessboard.clone()>
                    {self.render_cell(style)}
                </div>
            }
        }).collect()
    }

    fn render_cell(&self, style: Vec<&Style>) -> Vec<Html> {
        let mut vec = vec![];
        for i in 0..8 {
            for j in 0..12 {
                let style = style[(((i % 2 == 0) as usize + j) % 2 == 0) as usize].clone();
                vec.push(html!{<div class=style></div>})
            }
        }
        vec
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Board>::new().mount_to_body();
}
