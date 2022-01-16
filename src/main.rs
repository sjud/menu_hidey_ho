extern crate gloo_events;
extern crate web_sys;
use wasm_bindgen::JsCast;

use yew::prelude::*;
use gloo_events::{EventListener, EventListenerOptions};
use web_sys::HtmlElement;

fn main() {
        yew::start_app::<Model>();
}

    enum Msg {
        HideMenu,
    }

    struct Model {
        menu:NodeRef,
        show_menu_listener:Option<EventListener>,
        menu_is_hidden:bool,
    }

    impl Component for Model {
        type Message = Msg;
        type Properties = ();

        fn create(_ctx: &Context<Self>) -> Self {
            Self {
                menu:NodeRef::default(),
                show_menu_listener:None,
                menu_is_hidden:false,
            }
        }

        fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
            match msg {
                Msg::HideMenu => {
                        self.menu.cast::<HtmlElement>()
                            .unwrap()
                            .set_attribute("hidden", "true");
                }
            }
            true
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            html! {
            <div>
               <div id="menu" ref={self.menu.clone()}>{"Menu"}</div>
               <div>{"Not menu"}</div>
            </div>
        }
        }
        fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
            if !first_render {
                return;
            }

            let window = gloo_utils::window();
            let ontogglemenu = ctx.link().callback(|_: Event| Msg::HideMenu);
            let menu = self.menu.clone().cast::<HtmlElement>().unwrap();
            let options = EventListenerOptions::run_in_capture_phase();
            let listener = EventListener::new_with_options(&window, "click", options,move |e| {
                if let Some(target) = e.target_dyn_into::<HtmlElement>() {
                    if target == menu {
                        return;
                    }
                }
                ontogglemenu.emit(e.clone())
            });
            self.show_menu_listener = Some(listener);
        }
    }