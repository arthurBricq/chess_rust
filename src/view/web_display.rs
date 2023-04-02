use log::info;
use yew::prelude::*;
use super::super::view::ChessViewModelModel::*;
use crate::model::game::{ChessGame, Type, pos_to_index};

/// Component 

impl Component for ChessViewModelModel {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            game: ChessGame::new(),
            selected_pos: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Msg) -> bool {
        self.message_received(&msg)
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Self {
            ref game,
            ..
        } = *self;

        html! {
            <div>
                <div class="row">
                    // <button {onclick}>{game.get_char_at(0, 0)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(56))} class={self.get_class_name(0, 7)}>{self.get_char_at(0, 7)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(57))} class={self.get_class_name(1, 7)}>{self.get_char_at(1, 7)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(58))} class={self.get_class_name(2, 7)}>{self.get_char_at(2, 7)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(59))} class={self.get_class_name(3, 7)}>{self.get_char_at(3, 7)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(60))} class={self.get_class_name(4, 7)}>{self.get_char_at(4, 7)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(61))} class={self.get_class_name(5, 7)}>{self.get_char_at(5, 7)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(62))} class={self.get_class_name(6, 7)}>{self.get_char_at(6, 7)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(63))} class={self.get_class_name(7, 7)}>{self.get_char_at(7, 7)}</button>
                </div>
                <div class="row">
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(48))} class={self.get_class_name(0, 6)}>{self.get_char_at(0, 6)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(49))} class={self.get_class_name(1, 6)}>{self.get_char_at(1, 6)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(50))} class={self.get_class_name(2, 6)}>{self.get_char_at(2, 6)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(51))} class={self.get_class_name(3, 6)}>{self.get_char_at(3, 6)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(52))} class={self.get_class_name(4, 6)}>{self.get_char_at(4, 6)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(53))} class={self.get_class_name(5, 6)}>{self.get_char_at(5, 6)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(54))} class={self.get_class_name(6, 6)}>{self.get_char_at(6, 6)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(55))} class={self.get_class_name(7, 6)}>{self.get_char_at(7, 6)}</button>
                </div>
                <div class="row">
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(40))} class={self.get_class_name(0, 5)}>{self.get_char_at(0, 5)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(41))} class={self.get_class_name(1, 5)}>{self.get_char_at(1, 5)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(42))} class={self.get_class_name(2, 5)}>{self.get_char_at(2, 5)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(43))} class={self.get_class_name(3, 5)}>{self.get_char_at(3, 5)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(44))} class={self.get_class_name(4, 5)}>{self.get_char_at(4, 5)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(45))} class={self.get_class_name(5, 5)}>{self.get_char_at(5, 5)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(46))} class={self.get_class_name(6, 5)}>{self.get_char_at(6, 5)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(47))} class={self.get_class_name(7, 5)}>{self.get_char_at(7, 5)}</button>
                </div>
                <div class="row">
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(32))} class={{self.get_class_name(0, 4)}}>{self.get_char_at(0, 4)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(33))} class={self.get_class_name(1, 4)}>{self.get_char_at(1, 4)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(34))} class={self.get_class_name(2, 4)}>{self.get_char_at(2, 4)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(35))} class={self.get_class_name(3, 4)}>{self.get_char_at(3, 4)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(36))} class={self.get_class_name(4, 4)}>{self.get_char_at(4, 4)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(37))} class={self.get_class_name(5, 4)}>{self.get_char_at(5, 4)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(38))} class={self.get_class_name(6, 4)}>{self.get_char_at(6, 4)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(39))} class={self.get_class_name(7, 4)}>{self.get_char_at(7, 4)}</button>
                </div>
                <div class="row">
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(24))} class={self.get_class_name(0, 3)}>{self.get_char_at(0, 3)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(25))} class={self.get_class_name(1, 3)}>{self.get_char_at(1, 3)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(26))} class={self.get_class_name(2, 3)}>{self.get_char_at(2, 3)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(27))} class={self.get_class_name(3, 3)}>{self.get_char_at(3, 3)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(28))} class={self.get_class_name(4, 3)}>{self.get_char_at(4, 3)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(29))} class={self.get_class_name(5, 3)}>{self.get_char_at(5, 3)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(30))} class={self.get_class_name(6, 3)}>{self.get_char_at(6, 3)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(31))} class={self.get_class_name(7, 3)}>{self.get_char_at(7, 3)}</button>
                </div>
                <div class="row">
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(16))} class={self.get_class_name(0, 2)}>{self.get_char_at(0, 2)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(17))} class={self.get_class_name(1, 2)}>{self.get_char_at(1, 2)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(18))} class={self.get_class_name(2, 2)}>{self.get_char_at(2, 2)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(19))} class={self.get_class_name(3, 2)}>{self.get_char_at(3, 2)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(20))} class={self.get_class_name(4, 2)}>{self.get_char_at(4, 2)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(21))} class={self.get_class_name(5, 2)}>{self.get_char_at(5, 2)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(22))} class={self.get_class_name(6, 2)}>{self.get_char_at(6, 2)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(23))} class={self.get_class_name(7, 2)}>{self.get_char_at(7, 2)}</button>
                </div>
                <div class="row">
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(8))} class={self.get_class_name(0, 1)}>{self.get_char_at(0, 1)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(9))} class={self.get_class_name(1, 1)}>{self.get_char_at(1, 1)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(10))} class={self.get_class_name(2, 1)}>{self.get_char_at(2, 1)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(11))} class={self.get_class_name(3, 1)}>{self.get_char_at(3, 1)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(12))} class={self.get_class_name(4, 1)}>{self.get_char_at(4, 1)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(13))} class={self.get_class_name(5, 1)}>{self.get_char_at(5, 1)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(14))} class={self.get_class_name(6, 1)}>{self.get_char_at(6, 1)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(15))} class={self.get_class_name(7, 1)}>{self.get_char_at(7, 1)}</button>
                </div>
                <div class="row">
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(0))} class={self.get_class_name(0, 0)}>{self.get_char_at(0, 0)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(1))} class={self.get_class_name(1, 0)}>{self.get_char_at(1, 0)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(2))} class={self.get_class_name(2, 0)}>{self.get_char_at(2, 0)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(3))} class={self.get_class_name(3, 0)}>{self.get_char_at(3, 0)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(4))} class={self.get_class_name(4, 0)}>{self.get_char_at(4, 0)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(5))} class={self.get_class_name(5, 0)}>{self.get_char_at(5, 0)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(6))} class={self.get_class_name(6, 0)}>{self.get_char_at(6, 0)}</button>
                    <button onclick={ctx.link().callback(|_| Msg::SquareTapped(7))} class={self.get_class_name(7, 0)}>{self.get_char_at(7, 0)}</button>
                </div>
            </div>
        }
    
    }

}

pub fn display_board() {
    // ChessViewModelModel::run(Settings::default()); 

    // setup the logger
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Some info");

    // Render the webpage
    yew::Renderer::<ChessViewModelModel>::new().render();
}