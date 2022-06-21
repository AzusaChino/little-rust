#![recursion_limit = "1024"]
//
//! A simple demo of wasm application
//

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

/**
 * 模块组件
 */
mod components;

/**
 * 实体类定义
 */
mod model;

/**
 * 错误定义
 */
mod error;

/**
 * 页面信息
 */
mod pages;

/**
 * 路由定义
 */
mod router;

/**
 * 公共服务模块
 */
mod services;

/**
 * 公开错误枚举
 */
pub use error::Error;

#[derive(Debug, Clone, Copy, PartialEq, Routable)]
pub enum Route {
    #[at("*")]
    Fallback,
    #[at("/error")]
    Error,
    #[at("/")]
    Login,
    #[at("/404")]
    NotFound,
}

pub struct App {}

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    /**
     * Always update view while new data arrived
     */
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        }
    }
}

fn switch(routes: &Route) -> Html {
    // match routes {
    //     Route::Login | Route::Fallback => html! {<pages::Login/>},
    //     Route::Error => html! {<pages::Error/>},
    //     Route::NotFound => html! {<h1> {"404 page"}</h1>},
    // }
    html! {
        "ok"
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<App>();
}
