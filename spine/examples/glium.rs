use std::{
    path::Path,
    time::{Duration, Instant},
};

use glium::{glutin, Surface};
use spine::{
    animation::{AnimationState, AnimationStateData},
    atlas::Atlas,
    glium::GliumRenderer,
    skeleton::{Skeleton, SkeletonData, SkeletonJson},
    Renderer,
};

enum Action {
    Continue,
    Stop,
}

fn main() {
    let asset_dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/axie");

    let window_size = glutin::dpi::LogicalSize::new(640.0, 480.0);
    let window_builder = glutin::window::WindowBuilder::new().with_inner_size(window_size);

    let context_builder = glutin::ContextBuilder::new();

    let event_loop = glutin::event_loop::EventLoop::new();

    let display = glium::Display::new(window_builder, context_builder, &event_loop).unwrap();

    let renderer = GliumRenderer::new(display).unwrap();

    let atlas = Atlas::from_file(asset_dir.join("axie.atlas").to_str().unwrap()).unwrap();

    let mut skeleton_json = SkeletonJson::from_atlas(&atlas);
    skeleton_json.set_scale(1.0);

    let skeleton_data =
        SkeletonData::from_json_file(asset_dir.join("axie.json").to_str().unwrap(), skeleton_json)
            .unwrap();

    let animation_state_data = AnimationStateData::new(&skeleton_data);

    let mut skeleton = Skeleton::new(&skeleton_data);

    skeleton.set_y(-240.0);

    let mut animation_state = AnimationState::new(&animation_state_data);

    animation_state
        .set_animation_by_name(0, "action/idle", true)
        .unwrap();

    let mut last_time = Instant::now();
    let mut next_frame_time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        animation_state.update(last_time.elapsed().as_secs_f32());
        last_time = Instant::now();

        animation_state.apply(&mut skeleton);
        skeleton.update_world_transform();

        let mut frame = renderer.display().draw();
        frame.clear_color(0.0, 0.0, 0.0, 0.0);
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
