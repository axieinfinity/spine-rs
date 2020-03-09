use spine::Atlas;
use spine::SkeletonJson;

mod r#impl;

fn main() -> spine::Result<()> {
    let _atlas = Atlas::from_file("/Users/trung/Downloads/normal.atlas")?;
    let _skeleton_json = SkeletonJson::from_atlas(&_atlas)?;
    Ok(())
}
