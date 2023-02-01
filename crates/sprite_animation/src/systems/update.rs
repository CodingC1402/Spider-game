use crate::prelude::*;
use bevy::prelude::*;

pub fn anim_tree_update<R, T>(
    anim_tree: Res<R>,
    time: Res<Time>,
    mut q_anim: Query<(&mut AnimData<T>, &mut TextureAtlasSprite)>
)
where
    R: AnimTreeWrap<T>,
    T: AnimState,
{
    q_anim.iter_mut().for_each(|(mut data, mut sprite)| {
        match anim_tree.get().update(time.delta_seconds(), data.time, data.index, &data.state) {
            Ok(updates) => {
                data.index = updates.keyframe_index;
                data.time = updates.time;
    
                sprite.index = updates.atlas_index.eq(&usize::MAX).then_some(sprite.index).unwrap_or(updates.atlas_index);
            },
            Err(mgs) => panic!("{}", mgs.as_str()),
        }
    })
}
