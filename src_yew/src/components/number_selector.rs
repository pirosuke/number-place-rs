use yew::prelude::*;

#[derive(Properties, Clone)]
pub struct Props {
    pub x: u32,
    pub y: u32,
    pub on_number_select: Callback<u32>,
}

pub struct NumberSelector {
    link: ComponentLink<Self>,
    props: Props,
    target_number: u32,
}

pub enum Msg {
    NumberClick(u32),
}

impl NumberSelector {
    fn view_number_selector(&self, index: &u32) -> Html {
        let point = index * 40 - 40;
        let num = index.clone() as u32;
        let fill = match self.target_number == *index {
            true => "#ccf",
            false => "#fff",
        };
        html! {
            <>
                <rect
                    x=point
                    y="0"
                    width="40"
                    height="40"
                    stroke="#000"
                    stroke-width="1"
                    fill=fill />
                <text
                    x=point + 12
                    y="30"
                    width="40"
                    height="40"
                    fill="#000"
                    stroke="#000"
                    font-size="25">
                    { index }
                </text>
                <rect
                    onclick=self.link.callback(move |_| Msg::NumberClick(num))
                    x=point
                    y="0"
                    width="40"
                    height="40"
                    stroke-opacity="0"
                    stroke-width="1"
                    fill-opacity="0" />
            </>
        }
    }
}

impl Component for NumberSelector {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            target_number: 99,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::NumberClick(num) =>  {
                //ConsoleService::info(format!("Number Click: {}", &num).as_str());
                self.target_number = num;
                self.props.on_number_select.emit(num);
            },
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let transform = format!("translate({}, {})", self.props.x, self.props.y);

        html! {
            <g transform=transform>
                { for vec![1,2,3,4,5,6,7,8,9].iter().map(|e| self.view_number_selector(e)) }
            </g>
        }
    }
}
