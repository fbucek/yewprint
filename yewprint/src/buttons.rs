use crate::{ConditionalClass, Icon, IconName, Intent};
use yew::prelude::*;

pub struct Button {
    props: Props,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub fill: ConditionalClass,
    #[prop_or_default]
    pub minimal: ConditionalClass,
    #[prop_or_default]
    pub icon: Option<IconName>,
    #[prop_or_default]
    pub intent: Option<Intent>,
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    pub children: html::Children,
}

impl Component for Button {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Button { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
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
        html! {
            <button
                class=(
                    "bp3-button",
                    self.props.fill.map_some("bp3-fill"),
                    self.props.minimal.map_some("bp3-minimal"),
                    self.props.intent,
                )
                onclick={self.props.onclick.clone()}
            >
                {
                    if let Some(icon) = self.props.icon {
                        html! {
                            <Icon icon=icon />
                        }
                    } else {
                        html!()
                    }
                }
                <span class="bp3-button-text">
                    {self.props.children.clone()}
                </span>
            </button>
        }
    }
}
