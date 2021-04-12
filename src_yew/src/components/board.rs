use yew::prelude::*;
//use yew::services::console::ConsoleService;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub x: u32,
    pub y: u32,
}

pub struct Board {
    props: Props,
}

pub enum Msg {
}

impl Board {
    fn view_line(&self, index: u32) -> Html {
        let stroke_width = match index % 3 {
            0 => 3,
            _ => 1,
        };
        let point = index * 40;

        html! {
            <>
                <line
                    stroke="#000"
                    stroke-width=stroke_width
                    x1=point
                    y1="0"
                    x2=point
                    y2="360" />
                <line
                    stroke="#000"
                    stroke-width=stroke_width
                    x1="0"
                    y1=point
                    x2="360"
                    y2=point />
            </>
        }
    }
}

impl Component for Board {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            props,
        }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
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
        let transform = format!("translate({}, {})", self.props.x, self.props.y);

        html! {
            <g transform=transform>
                <rect
                    x="0"
                    y="0"
                    width="360"
                    height="360"
                    stroke="#000"
                    stroke-width="3"
                    fill-opacity="0"
                    fill="#fff" />
                { for (1..10).map(|e: u32| self.view_line(e)) }
            </g>
        }
    }
}
