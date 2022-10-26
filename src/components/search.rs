use gloo_console as console;
use wasm_bindgen::JsCast;
use yew::{html, Component, Context, Html, Properties, Callback, events::Event};
use web_sys::HtmlInputElement;

use crate::api::{SearchResults, SearchRequest, search_query};

pub enum ChangeMessage {
    NoOp,
    ClickSearchButton,
    ClickErrorX,
    SearchError(String),
    SetText(String),
    SetBaseMinimumCpu(isize),
    SetBaseMaximumCpu(isize),
    SetWeaponMinimumCpu(isize),
    SetWeaponMaximumCpu(isize),
    SetCosmeticMinimumCpu(isize),
    SetCosmeticMaximumCpu(isize),
    SetClusterMinimum(isize),
    SetClusterMaximum(isize),
    SetDateMinimum(String),
    SetDateMaximum(String),
    SetPage(isize),
    SetCount(isize),
    SetSortBy(String),
    SetOrderBy(String),
    Clear,
}

#[derive(Properties, PartialEq)]
pub struct SearchProperties {
    pub on_results: Callback<SearchResults>,
}

pub struct SearchComponent {
    request: SearchRequest,
    error: Option<String>,
}

impl Component for SearchComponent {
    type Message = ChangeMessage;
    type Properties = SearchProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            request: SearchRequest::default(),
            error: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            ChangeMessage::NoOp => {
                console::log!("Search NoOp");
            },
            ChangeMessage::ClickSearchButton => {
                let req = self.request.clone();
                let callback_success = ctx.props().on_results.clone();
                let callback_failure = ctx.link().callback(|e| ChangeMessage::SearchError(e));
                wasm_bindgen_futures::spawn_local(async move {
                    let result = search_query(&req).await;
                    match result {
                        Ok(res) => callback_success.emit(res),
                        Err(e) => callback_failure.emit(e),
                    }
                });
                console::log!("Click search button");
            },
            ChangeMessage::ClickErrorX => {
                self.error = None;
                console::log!("Click error X");
            },
            ChangeMessage::SearchError(e) => {
                console::log!("Search error:", &e);
                self.error = Some(e);
            },
            ChangeMessage::SetText(text) => {
                console::log!("Search text:", &text);
                self.request.text = Some(text);
            },
            ChangeMessage::SetBaseMinimumCpu(cpu) => {
                console::log!("Search base min cpu:", cpu);
                self.request.base_minimum_cpu = Some(cpu);
            },
            ChangeMessage::SetBaseMaximumCpu(cpu) => {
                console::log!("Search base max cpu:", cpu);
                self.request.base_maximum_cpu = Some(cpu);
            },
            ChangeMessage::SetWeaponMinimumCpu(cpu) => {
                console::log!("Search weapon min cpu:", cpu);
                self.request.weapon_minimum_cpu = Some(cpu);
            },
            ChangeMessage::SetWeaponMaximumCpu(cpu) => {
                console::log!("Search weapon max cpu:", cpu);
                self.request.weapon_maximum_cpu = Some(cpu);
            },
            ChangeMessage::SetCosmeticMinimumCpu(cpu) => {
                console::log!("Search cosmetic min cpu:", cpu);
                self.request.cosmetic_minimum_cpu = Some(cpu);
            },
            ChangeMessage::SetCosmeticMaximumCpu(cpu) => {
                console::log!("Search cosmetic max cpu:", cpu);
                self.request.cosmetic_maximum_cpu = Some(cpu);
            },
            ChangeMessage::SetClusterMinimum(clusters) => {
                console::log!("Search clusters min:", clusters);
                self.request.cluster_minimum = Some(clusters);
            },
            ChangeMessage::SetClusterMaximum(clusters) => {
                console::log!("Search clusters max:", clusters);
                self.request.cluster_maximum = Some(clusters);
            },
            ChangeMessage::SetDateMinimum(date) => {
                console::log!("Search date minimum:", &date);
                self.request.date_minimum = Some(date);
            },
            ChangeMessage::SetDateMaximum(date) => {
                console::log!("Search date max:", &date);
                self.request.date_maximum = Some(date);
            },
            ChangeMessage::SetPage(page) => {
                console::log!("Search page:", page);
                self.request.page = Some(page);
            },
            ChangeMessage::SetCount(count) => {
                console::log!("Search count:", count);
                self.request.count = Some(count);
            },
            ChangeMessage::SetSortBy(sort) => {
                console::log!("Search sort:", &sort);
                self.request.sort_by = sort;
            },
            ChangeMessage::SetOrderBy(order) => {
                console::log!("Search order:", &order);
                self.request.order_by = order;
            },
            ChangeMessage::Clear => {
                console::log!("Search clear");
                self.request = SearchRequest::default();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="search">
                <div class="search-input-text">
                    <label for="search_text" class="search-label">{"Text"}</label>
                    <input type="text" id="search_text" class="search-input-text-elem" onchange={
                        ctx.link().callback(|e: Event| {
                            let target = e.target().unwrap()
                                .unchecked_into::<HtmlInputElement>();
                            ChangeMessage::SetText(target.value())
                        })
                    }/>
                </div>
                <div class="search-input-number">
                    <label for="base_min_cpu" class="search-label">{"Minimum Base CPU"}</label>
                    <input type="number" id="base_min_cpu" class="search-input-number-elem" min="1" onchange={
                        ctx.link().batch_callback(|e: Event| {
                            let target = e.target().unwrap()
                                .unchecked_into::<HtmlInputElement>();
                            //console::log!("Range", target.value());
                            match target.value().parse() {
                                Ok(val) => Some(ChangeMessage::SetBaseMinimumCpu(val)),
                                Err(e) => {
                                    console::log!("base_min_cpu parse err:", e.to_string());
                                    None
                                }
                            }
                        })
                    }/>
                </div>
                <div class="search-input-number">
                    <label for="base_max_cpu" class="search-label">{"Maximum Base CPU"}</label>
                    <input type="number" id="base_max_cpu" class="search-input-number-elem" min="1" onchange={
                        ctx.link().batch_callback(|e: Event| {
                            let target = e.target().unwrap()
                                .unchecked_into::<HtmlInputElement>();
                            //console::log!("Range", target.value());
                            match target.value().parse() {
                                Ok(val) => Some(ChangeMessage::SetBaseMaximumCpu(val)),
                                Err(e) => {
                                    console::log!("base_max_cpu parse err:", e.to_string());
                                    None
                                }
                            }
                        })
                    }/>
                </div>
                <div class="search-input-number">
                    <label for="cluster_min" class="search-label">{"Minimum Clusters"}</label>
                    <input type="number" id="cluster_min" class="search-input-number-elem" min="1" onchange={
                        ctx.link().batch_callback(|e: Event| {
                            let target = e.target().unwrap()
                                .unchecked_into::<HtmlInputElement>();
                            //console::log!("Range", target.value());
                            match target.value().parse() {
                                Ok(val) => Some(ChangeMessage::SetClusterMinimum(val)),
                                Err(e) => {
                                    console::log!("cluster_min parse err:", e.to_string());
                                    None
                                }
                            }
                        })
                    }/>
                </div>
                <div class="search-input-number">
                    <label for="cluster_max" class="search-label">{"Maximum Clusters"}</label>
                    <input type="number" id="cluster_max" class="search-input-number-elem" min="1" onchange={
                        ctx.link().batch_callback(|e: Event| {
                            let target = e.target().unwrap()
                                .unchecked_into::<HtmlInputElement>();
                            //console::log!("Range", target.value());
                            match target.value().parse() {
                                Ok(val) => Some(ChangeMessage::SetClusterMaximum(val)),
                                Err(e) => {
                                    console::log!("cluster_max parse err:", e.to_string());
                                    None
                                }
                            }
                        })
                    }/>
                </div>
                <div class="search-input-number">
                    <label for="page" class="search-label">{"Page"}</label>
                    <input type="number" id="page" class="search-input-number-elem" min="1" onchange={
                        ctx.link().batch_callback(|e: Event| {
                            let target = e.target().unwrap()
                                .unchecked_into::<HtmlInputElement>();
                            //console::log!("Range", target.value());
                            match target.value().parse() {
                                Ok(val) => Some(ChangeMessage::SetPage(val)),
                                Err(e) => {
                                    console::log!("page parse err:", e.to_string());
                                    None
                                }
                            }
                        })
                    }/>
                </div>
                <div class="search-input-number">
                    <label for="count" class="search-label">{"Results"}</label>
                    <input type="number" id="count" class="search-input-number-elem" min="1" onchange={
                        ctx.link().batch_callback(|e: Event| {
                            let target = e.target().unwrap()
                                .unchecked_into::<HtmlInputElement>();
                            //console::log!("Range", target.value());
                            match target.value().parse() {
                                Ok(val) => Some(ChangeMessage::SetCount(val)),
                                Err(e) => {
                                    console::log!("count parse err:", e.to_string());
                                    None
                                }
                            }
                        })
                    }/>
                </div>
                <div class="search-input-select">
                    <label for="sort_by" class="search-label">{"Sort"}</label>
                    <select id="sort_by" class="search-input-select-elem" onchange={
                        ctx.link().callback(|e: Event| {
                            let target = e.target().unwrap()
                                .unchecked_into::<HtmlInputElement>();
                            //console::log!("Range", target.value());
                            ChangeMessage::SetSortBy(target.value())
                        })
                    }>
                        <option value="default" selected=true>{"Default"}</option>
                        <option value="cpuPower">{"CPU"}</option>
                        <option value="cpuWeapon">{"Weapon CPU"}</option>
                        <option value="cpuCosmetic">{"Cosmetic CPU"}</option>
                        <option value="price">{"Price"}</option>
                        <option value="date">{"Date"}</option>
                        <option value="clusterCount">{"Clusters"}</option>
                        <option value="views">{"Views"}</option>
                    </select>
                </div>
                <div class="search-input-select">
                    <label for="order_by" class="search-label">{"Order"}</label>
                    <select id="order_by" class="search-input-select-elem" onchange={
                        ctx.link().callback(|e: Event| {
                            let target = e.target().unwrap()
                                .unchecked_into::<HtmlInputElement>();
                            //console::log!("Range", target.value());
                            ChangeMessage::SetOrderBy(target.value())
                        })
                    }>
                        <option value="ascending" selected=true>{"Ascending"}</option>
                        <option value="descending">{"Descending"}</option>
                    </select>
                </div>
                <div class="search-button-drawer">
                    <div class="search-input-button" align="center">
                        <button class="search-input-button-elem" onclick={ctx.link().callback(|_| ChangeMessage::ClickSearchButton)}>
                            {"Search"}
                        </button>
                    </div>
                </div>
                /*<div class="search-input-button" align="center">
                    <button class="search-clear-button-elem" onclick={ctx.link().callback(|_| ChangeMessage::ClickSearchButton)}>
                        {"Reset"}
                    </button>
                </div>*/
            </div>
        }
    }
}
