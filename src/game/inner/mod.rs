use super::{GameBuilder, GameTime, Timer};
use crate::{
    asset::AssetDatabase,
    ecs::World,
    graphics::{engine::GraphicsEngine, Graphics},
    scene::{Scene, SceneId, SceneManager},
};
use std::{cell::RefCell, collections::HashMap, path::Path, rc::Rc, time::Duration};
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::EventLoop,
};

const FIXED_DELTA: f64 = 1.0 / 60.0;

pub struct Game {
    world: World,
    graphics: GraphicsEngine,
    scenes: HashMap<SceneId, Rc<RefCell<Box<dyn Scene>>>>,
    timer: Rc<RefCell<Timer>>,
}

impl Game {
    pub fn new() -> GameBuilder {
        GameBuilder::new()
    }

    pub(super) async fn build<T: Scene>(events: &EventLoop<()>, builder: GameBuilder) -> Game {
        let graphics = GraphicsEngine::new(events).await;

        let fixed_delta = Duration::from_secs_f64(FIXED_DELTA);
        let timer = Rc::new(RefCell::new(Timer::new(fixed_delta)));
        let scenes = builder.scenes.all().clone();
        let scene_manager = builder.scenes.build::<T>();
        let importers = builder.importers;
        let components = builder.components;
        let mut resources = builder.resources;

        resources.register(GameTime::new(timer.clone()));
        resources.register(Graphics::new(graphics.device().clone()));
        resources.register(AssetDatabase::new());
        resources.register(scene_manager);

        AssetDatabase::load(&Path::new("./assets"), &mut resources, &importers);

        let world = World::new(components, resources);

        Game {
            world,
            graphics,
            scenes,
            timer,
        }
    }

    fn id(&self) -> winit::window::WindowId {
        self.graphics.window().id()
    }

    fn update(&mut self) -> bool {
        let (mut accumulator, fixed_delta) = {
            let mut timer = self.timer.borrow_mut();
            timer.update();

            (timer.delta(), timer.fixed_delta())
        };

        while accumulator >= fixed_delta {
            let current = {
                let mut scene_manager = self.world.resource_mut::<SceneManager>();
                scene_manager.update(&self.world);

                scene_manager.current()
            };

            let scene = self.scenes.get(&current).unwrap();
            scene.borrow_mut().update(&self.world);

            accumulator -= fixed_delta;

            if self.world.resource::<SceneManager>().quitting() {
                return false;
            }
        }

        true
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let mut graphics = self.world.resource_mut::<Graphics>();

        self.graphics.render(&mut graphics)
    }

    fn window(&self) -> &winit::window::Window {
        self.graphics.window()
    }

    fn resize(&mut self, width: u32, height: u32) {
        self.graphics.resize(width, height);
    }
}

pub struct GameRunner;

impl GameRunner {
    pub fn run<T: Scene>(builder: GameBuilder) {
        let runner = async {
            let events = EventLoop::new();
            let mut game = Game::build::<T>(&events, builder).await;

            let _ = events.run(move |event, _, flow| match event {
                Event::WindowEvent { window_id, event } if window_id == game.id() => match event {
                    WindowEvent::Resized(size) => game.resize(size.width, size.height),
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        game.resize(new_inner_size.width, new_inner_size.height)
                    }
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => flow.set_exit(),
                    _ => {}
                },
                Event::MainEventsCleared => {
                    if game.update() {
                        match game.render() {
                            Ok(_) => {}
                            Err(wgpu::SurfaceError::Lost) => {
                                let size = game.window().inner_size();
                                game.resize(size.width, size.height);
                            }
                            Err(wgpu::SurfaceError::OutOfMemory) => flow.set_exit(),
                            Err(e) => eprintln!("{:?}", e),
                        }
                    } else {
                        flow.set_exit();
                    }
                }
                _ => {}
            });
        };

        pollster::block_on(runner);
    }
}
