use std::ops::{Add, Sub};

use bevy::utils::Uuid;
use sprite_animation_derive::ToUuid;

use crate::derive::*;

use super::{Node, NodeResult};

#[derive(Default)]
pub struct SpriteKeyframe {
    sprite_index: usize,
    delay: Option<f32>, // If none then it will use the default for sprite animation
}
impl SpriteKeyframe {
    pub fn new(index: usize) -> Self {
        SpriteKeyframe {
            sprite_index: index,
            delay: None,
        }
    }
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
    pub fn new(fps: usize, indexes: &[usize]) -> Self {
        let mut keyframes = Vec::<SpriteKeyframe>::new();
        indexes
            .iter()
            .for_each(|i| keyframes.push(SpriteKeyframe::new(*i)));

        Self {
            keyframes,
            delay: 1.0 / fps as f32,
        }
    }
    pub fn new_range(fps: usize, start: usize, end: usize) -> Self {
        let mut keyframes = Vec::<SpriteKeyframe>::new();
        (start..=end).for_each(|i| keyframes.push(SpriteKeyframe::new(i)));

        Self {
            keyframes,
            delay: 1.0 / fps as f32,
        }
    }
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

#[derive(ToUuid, Default)]
pub struct PlayNode {
    pub id: Uuid,
    pub speed: f32,
    pub anim: SpriteAnimation,
    pub is_loop: bool,
    /// If true reset index to 0 when this animation is play
    pub reset: bool,
}

impl PlayNode {
    pub fn new(speed: f32, is_loop: bool, reset: bool, anim: SpriteAnimation) -> PlayNode {
        let mut instance = PlayNode {
            speed,
            is_loop,
            anim,
            reset,
            ..Default::default()
        };
        instance.id = instance.new_uuid();

        instance
    }
}

impl<T> Node<T> for PlayNode
where
    T: AnimState,
{
    fn execute(
        &self,
        data: &crate::prelude::AnimData<T>,
        delta_time: f32,
        _: &mut Vec<(Uuid, usize)>,
    ) -> NodeResult {
        let time = data.time;
        let index = (self.reset && !data.current_node.eq(&self.id))
            .then_some(0)
            .unwrap_or(data.index % self.anim.len());
        let new_index = index.add(1);

        let construct_result = |x: AnimKeyframe| {
            Some(NodeResult::Sprite(
                x.delay + time,
                new_index,
                x.sprite_index,
                self.id,
            ))
        };

        let check_for_loop = || {
            self.is_loop
                .then_some(
                    self.anim
                        .get_keyframe(0)
                        .and_then(|x| {
                            Some(NodeResult::Sprite(
                                x.delay + time,
                                0,
                                x.sprite_index,
                                self.id,
                            ))
                        })
                        .unwrap(),
                )
                .unwrap_or_else(|| {
                    self.anim
                        .len()
                        .eq(&new_index)
                        .then_some(NodeResult::Finished)
                        .unwrap_or(NodeResult::NoUpdate)
                })
        };

        let return_next_frame_values = || {
            self.anim
                .get_keyframe(new_index)
                .and_then(construct_result)
                .unwrap_or_else(check_for_loop)
        };

        let return_old_frame = || {
            let new_time = time.sub(delta_time * self.speed);
            Some(NodeResult::Sprite(new_time, index, usize::MAX, self.id))
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
            .then_some(NodeResult::NoUpdate)
            .unwrap_or_else(play_animation)
    }
}
