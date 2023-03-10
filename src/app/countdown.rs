use gloo::timers::callback::Interval;
use yew::{html, Component, Context, Html, Properties};

pub enum Msg {
    UpdateTime,
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub deadline_secs: i64,
}

pub struct Countdown {
    deadline: i64,
    _standalone: Interval,
}

impl Countdown {
    fn remaining(&self) -> i64 {
        self.deadline - (js_sys::Date::now() as i64 / 1000)
    }
}

impl Component for Countdown {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let deadline = ctx.props().deadline_secs;

        let clock_handle = {
            let link = ctx.link().clone();
            Interval::new(1000, move || link.send_message(Msg::UpdateTime))
        };

        Self {
            deadline,
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
                        <h1>{"Lenka priletela!"}</h1>
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
                <h2>{"Lenka priletí za"}</h2>
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
