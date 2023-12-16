mod algorithms;
mod generator;
mod components;

use yew::prelude::*;
use crate::components::cell::{Cell, Array};
use crate::generator::array_generator::generate_array;

pub struct App {
    len: u32,
    min: i32,
    max: i32,
    sorted: bool,
    collection: Array,
    cell_width: i32,
    cell_height: i32,
    swap_time: i32,
}

impl App {
    fn view_cell(&self, idx: usize, cell: &Cell) -> Html {
        html! {
            <div key={idx} class={classes!("cell")}
                style={format!("width: {}; height: {}; background-color: {};", cell.width, cell.height, cell.color)}>
            </div>
        }
    }
}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let (len, min, max): (u32, i32, i32) = (20, 1, 50);
        let generated_vector: Vec<i32> = generate_array(len, min, max);
        let arr: Array = Array::from_vec(generated_vector);
        Self {
            len,
            min,
            max,
            sorted: false,
            collection: arr,
            cell_width: 20,
            cell_height: 100,
            swap_time: 10,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let array_view = self.collection.clone()
            .into_iter()
            .enumerate().map(|(idx, cell)| self.view_cell(idx, &cell));
        html! {
            <div>
                { for array_view }
            </div>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}