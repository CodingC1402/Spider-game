use super::PlayerAnimState;
use bevy::prelude::*;
use sprite_animation::prelude::{
    match_node::MatchNode,
    play_node::{PlayNode, SpriteAnimation},
    *, all_node::AllNode,
};

pub const FPS: usize = 12;

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
        true,
        SpriteAnimation::new_range(FPS, 9, 14),
    ))
}
fn create_jump_anim() -> AnimNode<PlayerAnimState> {
    AnimNode::PlayNode(PlayNode::new(
        1.,
        false,
        true,
        SpriteAnimation::new_range(FPS, 18, 20),
    ))
}
fn create_floating_anim() -> AnimNode<PlayerAnimState> {
    AnimNode::PlayNode(PlayNode::new(
        0.3,
        false,
        true,
        SpriteAnimation::new(FPS, &[21, 22, 23, 22]),
    ))
}
fn create_land_anim() -> AnimNode<PlayerAnimState> {
    AnimNode::PlayNode(PlayNode::new(
        1.,
        false,
        true,
        SpriteAnimation::new_range(FPS, 24, 26),
    ))
}
impl ToString for PlayerAnimState {
    fn to_string(&self) -> String {
        String::from(match self {
            PlayerAnimState::Idle => "PlayerState::Idle",
            PlayerAnimState::Walking => "PlayerState::Walking",
            PlayerAnimState::MidAir => "PlayerState::MidAir",
            PlayerAnimState::Landing => "PlayerAnimState::Landing",
            PlayerAnimState::Jumping => "PlayerAnimState::Jumping",
            PlayerAnimState::Hurt => "PlayerAnimState::Hurt",
            
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
        let jump_node = create_jump_anim();
        let float_node = create_floating_anim();
        let land_node = create_land_anim();

        let mut jump_land_all_node = AllNode::new();
        jump_land_all_node.nodes = vec![jump_node.get_id(), float_node.get_id()];
        jump_land_all_node.is_loop = false;

        start_node
            .insert(PlayerAnimState::Idle, idle_node.get_id())
            .insert(PlayerAnimState::Walking, walking_node.get_id())
            .insert(PlayerAnimState::Jumping, jump_land_all_node.get_id());

        let mut tree = PlayerAnimTree(AnimTree::<PlayerAnimState>::new(AnimNode::MatchNode(
            start_node,
        )));
        tree.get_mut()
            .insert_unwrap(idle_node)
            .insert_unwrap(walking_node)
            .insert_unwrap(jump_node)
            .insert_unwrap(float_node)
            .insert_unwrap(land_node)
            .insert_unwrap(AnimNode::AllNode(jump_land_all_node));
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
