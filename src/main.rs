#![recursion_limit = "256"]

mod world;

use yew::prelude::*;
use yew::services::RenderService;
use yew::services::render::RenderTask;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};
use wasm_bindgen::prelude::*;
use crate::world::{WORLD_WIDTH, WORLD_HEIGHT, World, Cell};

#[derive(Debug)]
pub enum Msg {
    Tick,
    Randomize,
    Start,
    Pause
}

#[derive(Debug)]
pub struct Model {
    link: ComponentLink<Self>,
    _task: RenderTask,
    canvas: NodeRef,
    ctx: Option<CanvasRenderingContext2d>,
    world: World,
    playing: bool
}


impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let task = RenderService::request_animation_frame(link.callback(|_| Msg::Tick));
        Self {
            link,
            canvas: NodeRef::default(),
            _task: task,
            ctx: None,
            world: World::new(),
            playing: false
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Tick => self.render(),
            Msg::Randomize => self.world.randomize(),
            Msg::Start => self.playing = true,
            Msg::Pause => self.playing = false
        };
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="container">
                <div class="center">
                    <h2>{ "Game of Life, but with Canvas" }</h2>
                </div>
                <div class="canvas-container">
                    <canvas id="life" width="300" height="300" ref=self.canvas.clone() />
                </div>
                <div class="controls">
                    <button onclick=self.link.callback(|_| Msg::Randomize)>{" Randomize "}</button>
                    <button onclick=self.link.callback(|_| Msg::Start)>{" Start "}</button>
                    <button onclick=self.link.callback(|_| Msg::Pause)>{" Pause "}</button>
                </div>
            </div>
        }
    }
}

impl Model {
    pub fn render(&mut self) {
        let canvas = self.canvas.cast::<HtmlCanvasElement>().unwrap();
        let ctx = match &self.ctx {
            Some(ctx) => ctx,
            None => {
                let ctx = CanvasRenderingContext2d::from(
                    JsValue::from(canvas.get_context("2d").unwrap().unwrap())
                );
                self.ctx = Some(ctx);
                self.ctx.as_ref().unwrap()
            }
        };

        if self.playing {
            self.world.step();
        }
        
        ctx.set_fill_style(&JsValue::from_str("#ffffff"));
        ctx.fill_rect(0.0, 0.0, 300.0, 300.0);

        ctx.set_fill_style(&JsValue::from_str("#000000"));
        const TILE_WIDTH: f64 = (1.0 / WORLD_WIDTH as f64) * 300.0;
        const TILE_HEIGHT: f64 = (1.0 / WORLD_HEIGHT as f64) * 300.0;

        for y in 0..WORLD_HEIGHT {
            for x in 0..WORLD_WIDTH {
                match self.world.cells[y][x] {
                    Cell::Dead => {},
                    Cell::Alive => {
                        ctx.fill_rect(x as f64 * TILE_WIDTH,
                            y as f64 * TILE_HEIGHT,
                            TILE_WIDTH,
                            TILE_HEIGHT);
                    }
                }
            }
        }

        self._task = RenderService::request_animation_frame(self.link.callback(|_| Msg::Tick));
    }
}

fn main() {
    yew::start_app::<Model>();
}
