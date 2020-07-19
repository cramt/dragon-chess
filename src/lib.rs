mod front_end;
mod board_controller;
mod board_piece;

extern crate wasm_bindgen_test;
extern crate wasm_bindgen;
extern crate console_error_panic_hook;

use wasm_bindgen::prelude::*;
use crate::pieces::vector3::Vector3;
use crate::board_controller::BoardController;
use std::sync::Mutex;
use yew::{App, Component, ComponentLink, Html};
use crate::front_end::board::Board;
use crate::front_end::raw_html::RawHTML;
use yew::prelude::*;

mod grid;
mod tests;
mod player;
mod board;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate maplit;

mod pieces;

struct Main {}

impl Component for Main {
    type Message = ();
    type Properties = ();

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Main {}
    }

    fn update(&mut self, msg: Self::Message) -> bool {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Board/>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    App::<Main>::new().mount_to_body();
}
