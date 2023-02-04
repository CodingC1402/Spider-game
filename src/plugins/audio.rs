use crate::GameState;
use bevy::prelude::{App, AssetServer, Handle, Plugin, Res, ResMut, Resource};
use bevy_kira_audio::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, IntoConditionalSystem};
use std::time::Duration;

use super::tilemap;

const MAIN_MENU_ST_PATH: &str = "soundtrack/main_menu.wav";
const LEVELS_ST_PATH: &str = "soundtrack/ingame.wav";
const CREDITS_ST_PATH: &str = "soundtrack/credits.wav";

#[derive(Resource, Default)]
struct SoundtrackHandles {
    main_menu: Handle<AudioSource>,
    in_game: Handle<AudioSource>,
    credits: Handle<AudioSource>,
}

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(bevy_kira_audio::prelude::AudioPlugin)
            .insert_resource(SoundtrackHandles::default())
            .add_startup_system(setup)
            .add_enter_system(GameState::InMenu, play_main_menu_music)
            .add_exit_system(GameState::InMenu, fade_out)
            .add_enter_system(GameState::InGame, play_ingame_music)
            .add_exit_system(GameState::InGame, fade_out)
            .add_system(fade_out_and_play_credits_music.run_if(tilemap::entered_credits))
            .add_system(fade_out.run_if(tilemap::exited_credits));
    }
}

fn setup(asset_server: Res<AssetServer>, mut soundtracks: ResMut<SoundtrackHandles>) {
    *soundtracks = SoundtrackHandles {
        main_menu: asset_server.load(MAIN_MENU_ST_PATH),
        in_game: asset_server.load(LEVELS_ST_PATH),
        credits: asset_server.load(CREDITS_ST_PATH),
    };
}

fn play_main_menu_music(audio: Res<Audio>, soundtracks: Res<SoundtrackHandles>) {
    play_soundtrack(&*audio, &soundtracks.main_menu);
}

fn play_ingame_music(audio: Res<Audio>, soundtracks: Res<SoundtrackHandles>) {
    play_soundtrack(&*audio, &soundtracks.in_game);
}

fn fade_out_and_play_credits_music(audio: Res<Audio>, soundtracks: Res<SoundtrackHandles>) {
    audio.stop().fade_out(AudioTween::default());
    play_soundtrack(&*audio, &soundtracks.credits);
}

fn play_soundtrack(audio: &Audio, track: &Handle<AudioSource>) {
    audio
        .play(track.clone())
        .with_volume(0.3)
        .looped()
        .linear_fade_in(Duration::from_secs_f32(0.5));
}

fn fade_out(audio: Res<Audio>) {
    audio.stop().fade_out(AudioTween::default());
}
