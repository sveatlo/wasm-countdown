use gloo::timers::callback::Interval;
use yew::{html, Component, Context, Html, Properties};

pub enum Msg {
    UpdateTime,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub deadline_timestamp: i64,
    pub title: &'static str,
    pub message_after_deadline: &'static str,
}

pub struct Countdown {
    props: Props,
    _standalone: Interval,
}

impl Countdown {
    fn remaining(&self) -> i64 {
        self.props.deadline_timestamp - (js_sys::Date::now() as i64 / 1000)
    }
}

impl Component for Countdown {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let clock_handle = {
            let link = ctx.link().clone();
            Interval::new(1000, move || link.send_message(Msg::UpdateTime))
        };

        Self {
            props: ctx.props().clone(),
            _standalone: clock_handle,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateTime => true,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let remaining_secs = self.remaining();

        if remaining_secs <= 0 {
            return html! {
                <>
                    <div class="confetti">
                        <div class="confetti-piece"></div>
                        <div class="confetti-piece"></div>
                        <div class="confetti-piece"></div>
                        <div class="confetti-piece"></div>
                        <div class="confetti-piece"></div>
                        <div class="confetti-piece"></div>
                        <div class="confetti-piece"></div>
                        <div class="confetti-piece"></div>
                        <div class="confetti-piece"></div>
                        <div class="confetti-piece"></div>
                        <div class="confetti-piece"></div>
                        <div class="confetti-piece"></div>
                        <div class="confetti-piece"></div>
                        <div class="confetti-piece"></div>
                        <div class="confetti-piece"></div>
                        <div class="confetti-piece"></div>
                        <div class="confetti-piece"></div>
                        <div class="confetti-piece"></div>
                        <div class="confetti-piece"></div>
                        <div class="confetti-piece"></div>
                    </div>
                    <div class="countdown">
                        <h1>{self.props.message_after_deadline}</h1>
                    </div>
                </>
            };
        }

        let days = remaining_secs / 86400;
        let hours = (remaining_secs % 86400) / 3600;
        let minutes = (remaining_secs % 3600) / 60;
        let seconds = remaining_secs % 60;

        html! {
            <div class="countdown">
                <h2>{self.props.title}</h2>
                <h1>
                    { format!("{days:02}") }<small>{"d "}</small>
                    { format!("{hours:02}") }<small>{"h "}</small>
                    { format!("{minutes:02}") }<small>{"m "}</small>
                    { format!("{seconds:02}") }<small>{"s"}</small>
                </h1>
            </div>
        }
    }
}
