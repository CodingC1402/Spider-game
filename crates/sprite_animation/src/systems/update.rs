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
        let mut logic_stack = data.logic_stack.clone();
        data.add_state_time(time.delta_seconds());
        match anim_tree.get().update(&data, time.delta_seconds(), &mut logic_stack) {
            Ok(value) => {
                data.logic_stack = logic_stack;
                match value {
                    AnimTreeUpdateResult::Update(updates) => {
                        data.index = updates.keyframe_index;
                        data.time = updates.time;
                        data.current_node = updates.current_node;
            
                        sprite.index = updates.atlas_index.eq(&usize::MAX).then_some(sprite.index).unwrap_or(updates.atlas_index);
                    },
                    // plus 1 to mark that this component have already received the last update and finished.
                    AnimTreeUpdateResult::Finished => data.index += 1,
                    AnimTreeUpdateResult::NoUpdate => (),
                }
            },
            Err(mgs) => panic!("{}", mgs.as_str()),
        }
    })
}
