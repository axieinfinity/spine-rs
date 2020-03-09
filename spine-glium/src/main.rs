use spine::Atlas;
use spine::SkeletonJson;

mod r#impl;

fn main() -> spine::Result<()> {
    let atlas = Atlas::from_file("/Users/trung/Downloads/normal.atlas")?;

    let mut skeleton_json = SkeletonJson::from_atlas(&atlas)?;
    skeleton_json.set_scale(0.5);

    Ok(())
}
