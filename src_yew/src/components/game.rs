use yew::prelude::*;
//use yew::services::console::ConsoleService;

use crate::components::number_selector::{NumberSelector};
use crate::components::board::{Board};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub x: u32,
    pub y: u32,
}

#[derive(Clone)]
pub struct NumberCell {
    num_text: String,
    row: usize,
    col: usize,
    is_hint: bool,
}

struct Numbers {
    entries: Vec<Vec<u32>>,
    hints: Vec<Vec<u32>>,
}

impl Numbers {
    fn number_cells(&self) -> Vec<NumberCell> {
        let mut cells: Vec<NumberCell> = Vec::new();
        for (row_index, row) in self.entries.iter().enumerate() {
            for (col_index, col) in row.iter().enumerate() {
                cells.push(NumberCell{
                    num_text: match col {
                        0 => String::from(""),
                        _ => format!("{}", col),
                    },
                    is_hint: self.hints[row_index][col_index] != 0,
                    row: row_index,
                    col: col_index,
                });
            }
        }
    
        cells
    }
}

pub struct Game {
    link: ComponentLink<Self>,
    props: Props,
    numbers: Numbers,
    target_cell: NumberCell,
}

pub enum Msg {
    CellClick(NumberCell),
    NumberClick(u32),
}

impl Game {
    fn view_number(&self, cell: NumberCell) -> Html {
        let cl = cell.clone();

        let stroke_width = match self.target_cell.row == cell.row && self.target_cell.col == cell.col {
            true => 5,
            false => 1,
        };
        let stroke = match self.target_cell.row == cell.row && self.target_cell.col == cell.col {
            true => "#F66",
            false => "#000",
        };

        let text_color = match cell.is_hint {
            true => "#00c",
            false => "#000",
        };

        html! {
            <>
                <text
                    x=cell.col * 40 + 12
                    y=cell.row * 40 + 30
                    width="40"
                    height="40"
                    fill=text_color
                    stroke=text_color
                    font-size="25">
                    { cell.num_text.clone() }
                </text>
                <rect
                    onclick=self.link.callback(move |_| Msg::CellClick(cl.clone()))
                    x=cell.col * 40
                    y=cell.row * 40
                    width="40"
                    height="40"
                    stroke=stroke
                    stroke-width=stroke_width
                    fill-opacity="0"
                    fill="#fff" />
            </>
        }
    }
}

impl Component for Game {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let hints = vec![
            vec![0,0,8,2,0,0,5,0,0],
            vec![1,0,5,0,6,0,2,3,4],
            vec![3,0,0,0,7,5,9,0,0],
            vec![0,8,6,4,0,0,0,0,0],
            vec![4,1,0,0,0,0,7,5,0],
            vec![5,2,0,0,0,1,8,4,6],
            vec![0,0,0,0,0,0,4,7,2],
            vec![0,5,0,0,0,2,6,8,0],
            vec![0,0,1,0,8,0,3,0,0],
        ];
        let numbers = Numbers{
            hints: hints.clone(),
            entries: hints.clone(),
        };

        Self {
            link,
            props,
            numbers,
            target_cell: NumberCell{
                num_text: String::from(""),
                row: 99,
                col: 99,
                is_hint: false,
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::CellClick(cell) =>  {
                //ConsoleService::info(format!("Cell Click: {}", &cell.num_text).as_str());
                if !cell.is_hint {
                    self.target_cell = cell;
                }
            },
            Msg::NumberClick(num) =>  {
                //ConsoleService::info(format!("Number Click: {}", &num).as_str());

                self.numbers.entries[self.target_cell.row][self.target_cell.col] = num;
            },
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let handle_number_selector = self
            .link
            .callback(|num: u32| Msg::NumberClick(num));
        let transform = format!("translate({}, {})", self.props.x, self.props.y);

        html! {
            <g transform=transform>
                <Board x=0 y=0 />
                { for self.numbers.number_cells().iter().map(|c| self.view_number(c.clone())) }
                <NumberSelector x=0 y=370 on_number_select=handle_number_selector.clone() />
            </g>
        }
    }
}
