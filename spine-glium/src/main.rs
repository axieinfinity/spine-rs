use spine::{AnimationState, AnimationStateData, Atlas, Skeleton, SkeletonData, SkeletonJson};

mod r#impl;

fn main() -> spine::Result<()> {
    let atlas = Atlas::from_file("/Users/trung/Downloads/normal.atlas")?;

    let mut skeleton_json = SkeletonJson::from_atlas(&atlas);
    skeleton_json.set_scale(0.5);

    let skeleton_data =
        SkeletonData::from_json_file("/Users/trung/Downloads/normal.json", &skeleton_json)?;

    let animation_state_data = AnimationStateData::new(&skeleton_data);

    let _skeleton = Skeleton::new(&skeleton_data);
    let _animation_state = AnimationState::new(&animation_state_data);

    Ok(())
}
