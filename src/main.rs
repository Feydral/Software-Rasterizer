use crate::engine::engine::Engine;
use crate::scenes::test_scene::TestScene;

mod math;
mod engine;
mod scenes;
mod rasterizer;
mod types;

fn main() {
    let mut scene = TestScene::new();
    let mut engine = Engine::new();
    engine.run(&mut scene);
}
