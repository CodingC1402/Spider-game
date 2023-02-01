use std::ops::{Add, Sub};

use bevy::utils::Uuid;
use sprite_animation_derive::ToUuid;

use crate::derive::*;

#[derive(Default)]
pub struct SpriteKeyframe {
    sprite_index: usize,
    delay: Option<f32>, // If none then it will use the default for sprite animation
}
pub struct AnimKeyframe {
    sprite_index: usize,
    delay: f32,
}

#[derive(Default)]
pub struct SpriteAnimation {
    keyframes: Vec<SpriteKeyframe>,
    delay: f32,
}
impl SpriteAnimation {
    pub fn len(&self) -> usize {
        self.keyframes.len()
    }
    pub fn get_keyframe(&self, index: usize) -> Option<AnimKeyframe> {
        self.keyframes.get(index).and_then(|x| {
            Some(AnimKeyframe {
                sprite_index: x.sprite_index,
                delay: x.delay.unwrap_or(self.delay),
            })
        })
    }
}

pub enum PlayNodeResult {
    NextNode(Uuid),
    Sprite(SpriteResult),
}

pub struct SpriteResult {
    pub delay: f32,
    pub keyframe_index: usize,
    pub atlas_index: usize,
}

#[derive(ToUuid, Default)]
pub struct PlayNode {
    pub id: Uuid,
    pub next: Uuid,
    pub speed: f32,
    pub anim: SpriteAnimation,
    pub is_loop: bool,
}

impl PlayNode {
    pub fn new(speed: f32, is_loop: bool, anim: SpriteAnimation) -> PlayNode {
        let mut instance = PlayNode {
            speed,
            is_loop,
            anim,
            ..Default::default()
        };
        instance.id = instance.to_uuid();

        instance
    }

    pub fn new_with_next(fps: f32, is_loop: bool, anim: SpriteAnimation, next: Uuid) -> PlayNode {
        let mut instance = Self::new(fps, is_loop, anim);
        instance.next = next;

        instance
    }

    pub fn execute(&self, time: f32, delta_time: f32, index: usize) -> PlayNodeResult {
        let new_index = index.add(1);

        let construct_result = |x: AnimKeyframe| {
            Some(PlayNodeResult::Sprite(SpriteResult {
                delay: x.delay + time,
                keyframe_index: new_index,
                atlas_index: x.sprite_index,
            }))
        };

        let check_for_loop = || {
            self.is_loop
                .then_some(PlayNodeResult::Sprite(
                    self.anim
                        .get_keyframe(0)
                        .and_then(|x| {
                            Some(SpriteResult {
                                delay: x.delay + time,
                                keyframe_index: 0,
                                atlas_index: x.sprite_index,
                            })
                        })
                        .unwrap(),
                ))
                .unwrap_or(PlayNodeResult::NextNode(self.next))
        };

        let return_next_frame_values = || {
            self.anim
                .get_keyframe(new_index)
                .and_then(construct_result)
                .unwrap_or_else(check_for_loop)
        };

        let return_old_frame = || {
            let new_time = time.sub(delta_time);
            Some(PlayNodeResult::Sprite(SpriteResult { delay: new_time, keyframe_index: index, atlas_index: usize::MAX }))
        };

        let play_animation = || {
            time.le(&&0.0)
                .then(return_next_frame_values)
                .or_else(return_old_frame)
                .unwrap()
        };

        self.anim
            .keyframes
            .is_empty()
            .then_some(PlayNodeResult::NextNode(self.next))
            .unwrap_or_else(play_animation)
    }
}
