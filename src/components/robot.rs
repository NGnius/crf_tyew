use yew::{html, Component, Context, Html, Properties};
use yew_icons::{Icon, IconId};

use crate::api::ResultItem;

#[derive(Properties, PartialEq)]
pub struct RobotProperties {
    pub robot: ResultItem
}

pub struct RobotComponent;

impl Component for RobotComponent {
    type Message = ();
    type Properties = RobotProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let item = &ctx.props().robot;
        let alt = format!("{} by {}", item.robot.name, item.robot.creatorName);
        html! {
            <div class="bot">
                <div class="bot-image"> //style={format!("background-image: url('{}');", robot.image)}>
                    <img alt={alt.clone()} src={item.robot.image.clone()} width="100%" title={alt}/>
                </div>
                <div class="bot-name">
                    <span alt={item.robot.id.clone()}>{ &item.robot.name }</span>
                </div>
                <div class="bot-creator" key={item.robot.creatorId.clone()}>
                    <span class="bot-creator-icon"><Icon icon_id={IconId::BootstrapBrush} title={"Creator"} height={"1.1rem".to_owned()}/></span>
                    <span class="bot-creator-name" alt={item.robot.creatorId.clone()}>{&item.robot.creatorName}</span>
                </div>
                <div class="bot-cpu">
                    //<div class="bot-cpu-header">{"CPU"}</div>
                    <div class="bot-base-cpu">
                        <span class="bot-cpu-icon"><Icon icon_id={IconId::BootstrapCpu} title={"Base CPU"} height={"1rem".to_owned()}/></span>
                        <span class="bot-cpu-number">{item.robot.baseCpu}</span>
                    </div>
                    <div class="bot-weapon-cpu">
                        <span class="bot-cpu-icon"><Icon icon_id={IconId::BootstrapBandaid} title={"Weapon CPU"} height={"1rem".to_owned()}/></span>
                        <span class="bot-cpu-number">{item.robot.weaponCpu}</span>
                    </div>
                    <div class="bot-cosmetic-cpu">
                        <span class="bot-cpu-icon"><Icon icon_id={IconId::BootstrapBox2Heart} title={"Cosmetic CPU"} height={"1rem".to_owned()}/></span>
                        <span class="bot-cpu-number">{item.robot.cosmeticCpu}</span>
                    </div>
                </div>
                <div class="bot-cluster-wrapper">
                    <div class="bot-cluster-count">
                        <span class="bot-cluster-icon"><Icon icon_id={IconId::BootstrapBoxes} title={"Clusters"} height={"1rem".to_owned()}/></span>
                        <span class="bot-cluster-number">{item.robot.clusterCount}</span>
                    </div>
                </div>
                /*<div class="bot-price-wrapper">
                    {
                        item.prices.iter().enumerate().map(|(index, price)| {
                            let (icon, name) = match index {
                                0 => (IconId::BootstrapCoin, "Techpoints"),
                                1 => (IconId::LucideCurrency, "Bloxcoins"),
                                _ => (IconId::BootstrapCashCoin, "???"),
                            };
                            html!{
                                <div class="bot-price">
                                    <span class="bot-price-icon"><Icon icon_id={icon} title={name} height={"1rem".to_owned()}/></span>
                                    <span class="bot-price-number">{price.amount}</span>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>*/
            </div>
        }
    }
}
