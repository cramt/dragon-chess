
use yew::virtual_dom::VNode;
use yew::{Component, ComponentLink, Html, ShouldRender, web_sys, Properties};

pub struct RawHTML {
    props: RawHTMLProps,
}

#[derive(Debug, Clone, Eq, PartialEq, Properties)]
pub struct RawHTMLProps {
    pub inner_html: String,
}

impl Component for RawHTML {
    type Message = ();
    type Properties = RawHTMLProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        RawHTML { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let js_svg = {
            let div = web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .create_element("div")
                .unwrap();
            div.set_inner_html(self.props.inner_html.as_str());
            div
        };
        eprintln!("js_svg: {:?}", js_svg);
        let node = yew::web_sys::Node::from(js_svg);
        let vnode = VNode::VRef(node);
        eprintln!("svg: {:?}", vnode);
        vnode
    }
}
