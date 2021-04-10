extern crate number_place_lib;

use crate::components::number_selector::{NumberSelector};
use crate::components::board::{Board};

use anyhow::Result;
use yew::{format::{Json, Nothing}, prelude::*};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::services::console::ConsoleService;
use yew::utils::document;

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
    entries: Vec<Vec<i32>>,
    hints: Vec<Vec<i32>>,
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

    fn remaining_cell_count(&self, entries: Vec<Vec<i32>>) -> usize {
        entries.iter()
            .fold(0, |sum, row| {
                let zero_count: usize = row.iter()
                    .filter(|&col| *col == 0)
                    .count();
                sum + zero_count
            })
    }

    fn are_entries_solved(&self) -> bool {
        number_place_lib::check_solved(&self.entries)
    }
}

pub struct Game {
    link: ComponentLink<Self>,
    props: Props,
    numbers: Numbers,
    target_cell: NumberCell,
    fetch_task: Option<FetchTask>,
    problem_index: usize,
}

pub enum Msg {
    CellClick(NumberCell),
    NumberClick(i32),
    ResetClick(),
    NewGameResponse(Result<Vec<Vec<i32>>, anyhow::Error>),
}

impl Game {
    fn view_number(&self, cell: NumberCell, x: usize, y: usize) -> Html {
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
                    x=x + cell.col * 40 + 12
                    y=y + cell.row * 40 + 30
                    width="40"
                    height="40"
                    fill=text_color
                    stroke=text_color
                    font-size="25">
                    { cell.num_text.clone() }
                </text>
                <rect
                    onclick=self.link.callback(move |_| Msg::CellClick(cl.clone()))
                    x=x + cell.col * 40
                    y=y + cell.row * 40
                    width="40"
                    height="40"
                    stroke=stroke
                    stroke-width=stroke_width
                    fill-opacity="0"
                    fill="#fff" />
            </>
        }
    }

    fn next_problem_index(&mut self) {
        self.problem_index = match self.problem_index {
            9 => 0,
            _ => self.problem_index + 1
        };
    }
}

impl Component for Game {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let hints = vec![
            vec![0,0,0,0,0,0,0,0,0],
            vec![0,0,0,0,0,0,0,0,0],
            vec![0,0,0,0,0,0,0,0,0],
            vec![0,0,0,0,0,0,0,0,0],
            vec![0,0,0,0,0,0,0,0,0],
            vec![0,0,0,0,0,0,0,0,0],
            vec![0,0,0,0,0,0,0,0,0],
            vec![0,0,0,0,0,0,0,0,0],
            vec![0,0,0,0,0,0,0,0,0],
        ];
        let numbers = Numbers{
            hints: hints.clone(),
            entries: hints.clone(),
        };

        link.send_message(Msg::ResetClick());

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
            fetch_task: None,
            problem_index: 0,
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
            Msg::ResetClick() => {
                let url_path = document().location().unwrap().pathname().unwrap().replace("/index.html", "");
                //ConsoleService::info(format!("ResetClick: {}", url_path).as_str());
                let req = Request::get(format!("{}/problems/{}.json", url_path, self.problem_index))
                    .body(Nothing)
                    .expect("Could not build request.");
                let callback = self.link
                    .callback(|response: Response<Json<Result<Vec<Vec<i32>>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg::NewGameResponse(data)
                    });
                let task = FetchService::fetch(req, callback).expect("failed to start request");
                self.fetch_task = Some(task);
                self.next_problem_index();
            },
            Msg::NewGameResponse(res) => {
                //ConsoleService::info(format!("NewGameResponse").as_str());
                match res {
                    Ok(hints) => {
                        self.numbers.hints = hints.clone();
                        self.numbers.entries = hints.clone();
                    }
                    Err(error) => {
                        ConsoleService::info(format!("New game error: {}", error).as_str());
                    }
                }
                self.fetch_task = None;
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
            .callback(|num: i32| Msg::NumberClick(num));
        let transform = format!("translate({}, {})", self.props.x, self.props.y);

        let hint_zero_count = self.numbers.remaining_cell_count(self.numbers.hints.clone());
        let entry_zero_count = self.numbers.remaining_cell_count(self.numbers.entries.clone());

        let solved_opacity = match self.numbers.are_entries_solved() {
            true => "1.0",
            false => "0.0",
        };

        html! {
            <g transform=transform>
                <text
                    x=110
                    y=20
                    fill="#000"
                    stroke="#000"
                    text-anchor="middle"
                    dominant-baseline="central"
                    font-size="20">{ format!("#{} {}/{}", self.problem_index, entry_zero_count, hint_zero_count) }</text>
                <circle
                    r="10"
                    cx="200"
                    cy="20"
                    fill-opacity="0"
                    stroke-opacity=solved_opacity
                    stroke="#0c0"
                    stroke-width="4" />
                <rect
                    x=0
                    y=0
                    width="220"
                    height="40"
                    stroke="#000"
                    stroke-width="1"
                    fill-opacity="0"
                    fill="#fff" />

                <rect
                    x=230
                    y=0
                    rx=5
                    ry=5
                    width="130"
                    height="40"
                    stroke="#c00"
                    stroke-width="1"
                    fill="#fcc" />
                <text
                    x=295
                    y=20
                    fill="#000"
                    stroke="#000"
                    text-anchor="middle"
                    dominant-baseline="central"
                    font-size="15">{ "NEXT GAME" }</text>
                <rect
                    onclick=self.link.callback(|_| Msg::ResetClick())
                    x=230
                    y=0
                    width="130"
                    height="40"
                    stroke-opacity="0"
                    fill-opacity="0" />

                <Board x=0 y=50 />
                { for self.numbers.number_cells().iter().map(|c| self.view_number(c.clone(), 0, 50)) }
                <NumberSelector x=0 y=420 on_number_select=handle_number_selector.clone() />
            </g>
        }
    }
}
