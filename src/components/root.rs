use gloo_console as console;
use yew::{html, Component, Context, Html};

use crate::api::{ResultItem, SearchResults};
use super::{RobotComponent, SearchComponent};

pub struct RootComponent {
    results: Vec<ResultItem>,
}

impl Component for RootComponent {
    type Message = SearchResults;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            results: vec![],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        self.results = msg.results;
        console::log!("Got search results");
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <div class="search-wrapper">
                    <SearchComponent on_results={ctx.link().callback(|x| x)}/>
                </div>
                <div class="bot-wrapper">{
                    if self.results.is_empty() {
                        html!{
                            <div class="bot-empty">
                                {"Search for robots from the CRF2"}
                            </div>
                        }
                    } else {
                        self.results.iter().map(|bot| {
                            html!{ <RobotComponent robot={bot.clone()} key={bot.robot.id.clone()}/> }
                        }).collect::<Html>()
                    }
                }</div>
                <div class="footer">
                    {"Unofficial CRF2 site by "}<a href="https://github.com/NGnius">{"NGnius"}</a>
                    {" | "}
                    {"API by "}<a href="https://www.freejamgames.com/">{"FreeJam"}</a>{" through "}<a href="https://github.com/NGnius/libfj">{"libfj"}</a>
                </div>
            </div>
        }
    }
}
