mod countdown;

use yew::prelude::*;

use countdown::Countdown;

#[function_component(App)]
pub fn app() -> Html {
    let deadline = js_sys::Date::new(&"2023-03-17T12:20+01:00".into());
    let deadline = deadline.get_time() as i64 / 1000 as i64;

    gloo::console::info!("deadline", deadline);

    html! {
        <main>
            <Countdown title={"Lenka priletí za"} message_after_deadline={"Lenka priletela!"} deadline_timestamp={deadline} />
        </main>
    }
}
