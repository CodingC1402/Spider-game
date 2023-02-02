use super::PlayerAnimState;
use bevy::prelude::*;
use sprite_animation::prelude::{
    match_node::MatchNode,
    play_node::{PlayNode, SpriteAnimation},
    *,
};

pub const FPS: usize = 14;

fn create_idle_anim() -> AnimNode<PlayerAnimState> {
    AnimNode::PlayNode(PlayNode::new(
        0.3,
        true,
        true,
        SpriteAnimation::new(FPS, &[4, 3, 2, 0, 4, 4, 4]),
    ))
}
fn create_walking_anim() -> AnimNode<PlayerAnimState> {
    AnimNode::PlayNode(PlayNode::new(
        1.,
        true,
        false,
        SpriteAnimation::new_range(FPS, 9, 14),
    ))
}
// fn create_ascending_anim() -> SpriteAnimation {

// }
// fn create_descending_anim() -> SpriteAnimation {

// }
// fn create_floating_anim() -> SpriteAnimation {

// }
impl ToString for PlayerAnimState {
    fn to_string(&self) -> String {
        String::from(match self {
            PlayerAnimState::Idle => "PlayerState::Idle",
            PlayerAnimState::Walking => "PlayerState::Walking",
            PlayerAnimState::MidAir => "PlayerState::MidAir",
            PlayerAnimState::Ascending => "PlayerState::Ascending",
            PlayerAnimState::Descending => "PlayerState::Descending",
        })
    }
}
impl AnimState for PlayerAnimState {}

#[derive(Resource, Default)]
pub struct PlayerAnimTree(AnimTree<PlayerAnimState>);
impl AnimTreeWrap<PlayerAnimState> for PlayerAnimTree {
    fn get(&self) -> &AnimTree<PlayerAnimState> {
        &self.0
    }

    fn get_mut(&mut self) -> &mut AnimTree<PlayerAnimState> {
        &mut self.0
    }
}

pub struct PlayerAnimationPlugin;
impl PlayerAnimationPlugin {
    pub fn build_anim_tree() -> PlayerAnimTree {
        let mut start_node: MatchNode<PlayerAnimState> = MatchNode::new();
        let idle_node = create_idle_anim();
        let walking_node = create_walking_anim();

        start_node
            .insert(PlayerAnimState::Idle, idle_node.get_id())
            .insert(PlayerAnimState::Walking, walking_node.get_id());

        let mut tree = PlayerAnimTree(AnimTree::<PlayerAnimState>::new(AnimNode::MatchNode(
            start_node,
        )));
        tree.get_mut()
            .insert_node(idle_node)
            .unwrap()
            .insert_node(walking_node)
            .unwrap();

        tree
    }
}

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Self::build_anim_tree())
            .insert_resource(Self::build_anim_tree())
            .add_plugin(AnimPlugin::<PlayerAnimTree, PlayerAnimState>::default());
    }
}
