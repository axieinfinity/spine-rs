#[macro_use]
extern crate lazy_static;

use std::{
    error::Error,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use glium::{glutin, Surface};
use spine::{
    animation::{AnimationState, AnimationStateData},
    backend::glium::GliumRenderer,
    geometry::Bounds,
    render::Renderer,
    skeleton::{Skeleton, SkeletonData, SkeletonJson},
};

lazy_static! {
    static ref ASSET_DIR: PathBuf = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/axie");
}

enum Action {
    Continue,
    Stop,
}

#[inline]
fn path(subpath: &str) -> PathBuf {
    ASSET_DIR.join(subpath)
}

fn main() -> Result<(), Box<dyn Error>> {
    let (event_loop, display) = {
        let window_size = glutin::dpi::LogicalSize::new(640.0, 480.0);
        let window_builder = glutin::window::WindowBuilder::new().with_inner_size(window_size);

        let context_builder = glutin::ContextBuilder::new();

        let event_loop = glutin::event_loop::EventLoop::new();
        let display = glium::Display::new(window_builder, context_builder, &event_loop)?;

        (event_loop, display)
    };

    let mut renderer = GliumRenderer::new(display)?;
    let atlas = renderer.new_atlas(path("axie.atlas"))?;

    let mut skeleton_json = SkeletonJson::new(&atlas);
    skeleton_json.set_scale(1.0);

    let skeleton_data = SkeletonData::from_json_file(path("axie.json"), skeleton_json)?;
    let animation_state_data = AnimationStateData::new(&skeleton_data);

    let mut skeleton = Skeleton::new(&skeleton_data);
    skeleton.set_attachment("shadow", None)?;

    let Bounds { y_min, y_max, .. } = skeleton.get_bounds();
    skeleton.set_y((y_min - y_max) / 2.0);

    let mut animation_state = AnimationState::new(&animation_state_data);
    animation_state.set_animation_by_name(0, "action/idle", true)?;

    let mut last_time = Instant::now();
    let mut next_frame_time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        animation_state.update(last_time.elapsed().as_secs_f32());
        last_time = Instant::now();

        animation_state.apply(&mut skeleton);
        skeleton.update_world_transform();

        let mut frame = renderer.display().draw();
        frame.clear_color(1.0, 1.0, 1.0, 1.0);
        renderer.render(&mut skeleton, &mut frame).unwrap();
        frame.finish().unwrap();

        let action = match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => Action::Stop,
                _ => Action::Continue,
            },

            _ => Action::Continue,
        };

        *control_flow = match action {
            Action::Continue => {
                next_frame_time = Instant::now() + Duration::from_nanos(16666667);
                glutin::event_loop::ControlFlow::WaitUntil(next_frame_time)
            }

            Action::Stop => glutin::event_loop::ControlFlow::Exit,
        };
    });
}
