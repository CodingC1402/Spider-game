use std::ops::Not;

use crate::{data::player::PlayerMovement, GameState, utils::state_helper::StateExtend};

use super::{PlayerAnimState, PlayerEvent};
use bevy::prelude::*;
use sprite_animation::prelude::{
    all_node::AllNode,
    match_node::MatchNode,
    play_node::{PlayNode, SpriteAnimation},
    *,
};

pub const FPS: usize = 12;

fn create_standing_anim() -> AnimNode<PlayerAnimState> {
    AnimNode::PlayNode(PlayNode::new(
        0.3,
        true,
        true,
        SpriteAnimation::new(FPS, &[0]),
    ))
}
fn create_idle_anim() -> AnimNode<PlayerAnimState> {
    AnimNode::PlayNode(PlayNode::new(
        0.3,
        true,
        true,
        SpriteAnimation::new(FPS, &[4, 3, 0, 0, 4, 4, 4]),
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
        2.,
        false,
        true,
        SpriteAnimation::new_range(FPS, 18, 18),
    ))
}
fn create_floating_anim() -> AnimNode<PlayerAnimState> {
    AnimNode::PlayNode(PlayNode::new(
        0.5,
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
            PlayerAnimState::Standing => "PlayerAnimState::Standing",
            PlayerAnimState::None => "PlayerAnimState::None",
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

pub struct PlayerAnimationPlugin {
    run_in: Option<GameState>,
}
impl PlayerAnimationPlugin {
    pub fn new(state: Option<GameState>) -> Self {
        Self {
            run_in: state
        }
    }
}

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(build_anim_tree())
            .add_plugin(AnimPlugin::<PlayerAnimTree, PlayerAnimState, GameState>::new(self.run_in))
            .add_system_run_if(self.run_in, update_animation);
    }
}

fn build_anim_tree() -> PlayerAnimTree {
    let mut start_node: MatchNode<PlayerAnimState> = MatchNode::new();
    let idle_node = create_idle_anim();
    let walking_node = create_walking_anim();
    let jump_anim_node = create_jump_anim();
    let float_node = create_floating_anim();
    let land_anim_node = create_land_anim();
    let stand_node = create_standing_anim();

    let mut jump_node = AllNode::new();
    jump_node.nodes = vec![jump_anim_node.get_id(), float_node.get_id()];
    jump_node.is_loop = false;

    let mut land_node = AllNode::new();
    land_node.nodes = vec![land_anim_node.get_id(), stand_node.get_id()];
    land_node.is_loop = false;

    start_node
        .insert(PlayerAnimState::Idle, idle_node.get_id())
        .insert(PlayerAnimState::Walking, walking_node.get_id())
        .insert(PlayerAnimState::Jumping, jump_node.get_id())
        .insert(PlayerAnimState::MidAir, float_node.get_id())
        .insert(PlayerAnimState::Landing, land_node.get_id())
        .insert(PlayerAnimState::Standing, stand_node.get_id());

    let mut tree = PlayerAnimTree(AnimTree::<PlayerAnimState>::new(AnimNode::MatchNode(
        start_node,
    )));
    tree.get_mut()
        .insert_unwrap(idle_node)
        .insert_unwrap(walking_node)
        .insert_unwrap(AnimNode::AllNode(jump_node))
        .insert_unwrap(float_node)
        .insert_unwrap(AnimNode::AllNode(land_node))
        .insert_unwrap(jump_anim_node)
        .insert_unwrap(land_anim_node)
        .insert_unwrap(stand_node);

    tree
}

const IDLE_TIME: f32 = 1.5;
const HURT_TIME: f32 = 0.5;

fn update_animation(
    mut q: Query<(
        &mut AnimData<PlayerAnimState>,
        &PlayerMovement,
        &mut TextureAtlasSprite,
    )>,
    mut e_reader: EventReader<PlayerEvent>,
) {
    q.iter_mut()
        .for_each(|(mut anim_data, movement_data, mut sprite)| {
            let get_new_state = |data: &AnimData<PlayerAnimState>, new_state: PlayerAnimState| {
                data.get_state()
                    .eq(&PlayerAnimState::Hurt)
                    .then_some(data.get_state_time().ge(&HURT_TIME))
                    .unwrap_or(true)
                    .then_some(new_state)
            };

            e_reader.iter().for_each(|e| {
                let state = anim_data.get_state();
                let anim_state = match e {
                    PlayerEvent::Airborne(_) => PlayerAnimState::MidAir,
                    PlayerEvent::Jumped(_) => PlayerAnimState::Jumping,
                    PlayerEvent::Grounded(_) => PlayerAnimState::Landing,
                    PlayerEvent::Moving(_, _) => PlayerAnimState::Walking,
                    PlayerEvent::Standing(_) => PlayerAnimState::Standing,
                    PlayerEvent::ShotWeb => PlayerAnimState::None,
                    PlayerEvent::Died(_) => PlayerAnimState::None,
                    // PlayerEvent::Hurt(_) => PlayerAnimState::Hurt,
                    // PlayerEvent::Attacks(_) => PlayerAnimState::Hurt,
                };

                let anim_state = get_new_state(anim_data.as_ref(), anim_state)
                    .and_then(|anim_state| match anim_state {
                        PlayerAnimState::None => None,
                        PlayerAnimState::Standing => {
                            state.eq(&PlayerAnimState::Idle).not().then_some(anim_state)
                        }
                        PlayerAnimState::MidAir => state
                            .eq(&PlayerAnimState::Jumping)
                            .not()
                            .then_some(anim_state),
                        PlayerAnimState::Walking => (state.eq(&PlayerAnimState::MidAir)
                            || state.eq(&PlayerAnimState::Jumping)
                            || state.eq(&PlayerAnimState::Hurt))
                        .not()
                        .then_some(anim_state),
                        _ => Some(anim_state),
                    })
                    .unwrap_or(*anim_data.get_state());

                anim_data.set_state(anim_state);
            });

            // Continuous update
            match anim_data.get_state() {
                PlayerAnimState::Standing | PlayerAnimState::Landing => {
                    anim_data
                        .get_state_time()
                        .ge(&IDLE_TIME)
                        .then(|| anim_data.set_state(PlayerAnimState::Idle));
                }
                _ => (),
            };

            (movement_data.axis != 0.0).then(|| {
                match anim_data.get_state() {
                    PlayerAnimState::Hurt | PlayerAnimState::Jumping | PlayerAnimState::MidAir => {}
                    _ => {
                        anim_data.set_state(PlayerAnimState::Walking);
                    }
                };
                sprite.flip_x = movement_data.axis.gt(&0.0);
            });
        });
}
