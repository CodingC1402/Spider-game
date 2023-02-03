use bevy::{app::AppExit, prelude::*};
use iyes_loopless::state::NextState;

use crate::{data::menu::*, GameState};
use kayak_ui::{
    prelude::*,
    widgets::{
        ButtonState, KayakAppBundle, KayakWidgetsContextPlugin, NinePatch, NinePatchBundle,
        TextProps, TextWidgetBundle,
    },
};

fn menu_button_render(
    In((widget_context, entity)): In<(KayakWidgetContext, Entity)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    menu_button_query: Query<&MenuButton>,
    state_query: Query<&ButtonState>,
) -> bool {
    let state_entity =
        widget_context.use_state(&mut commands, entity, ButtonState { hovering: false });

    let button_text = menu_button_query.get(entity).unwrap().text.clone();
    let button_image = asset_server.load("main_menu/button.png");
    let button_image_hover = asset_server.load("main_menu/button-hover.png");

    let on_event = OnEvent::new(
        move |In((event_dispatcher_context, _, mut event, _entity)): In<(
            EventDispatcherContext,
            WidgetState,
            Event,
            Entity,
        )>,
              mut query: Query<&mut ButtonState>| {
            if let Ok(mut button) = query.get_mut(state_entity) {
                match event.event_type {
                    EventType::MouseIn(..) => {
                        event.stop_propagation();
                        button.hovering = true;
                    }
                    EventType::MouseOut(..) => {
                        button.hovering = false;
                    }
                    _ => {}
                }
            }
            (event_dispatcher_context, event)
        },
    );

    if let Ok(button_state) = state_query.get(state_entity) {
        let button_image_handle = if button_state.hovering {
            button_image_hover
        } else {
            button_image
        };

        let parent_id = Some(entity);
        rsx! {
            <NinePatchBundle
                nine_patch={NinePatch {
                    handle: button_image_handle,
                    border: Edge::all(10.0),
                }}
                styles={KStyle {
                    width: Units::Stretch(1.0).into(),
                    height: Units::Pixels(40.0).into(),
                    bottom: Units::Pixels(30.0).into(),
                    left: Units::Pixels(50.0).into(),
                    right: Units::Pixels(50.0).into(),
                    ..KStyle::default()
                }}
                on_event={on_event}
            >
                <TextWidgetBundle
                    styles={KStyle {
                        top: Units::Stretch(1.0).into(),
                        bottom: Units::Stretch(1.0).into(),
                        ..Default::default()
                    }}
                    text={TextProps {
                        alignment: Alignment::Middle,
                        content: button_text,
                        size: 28.0,
                        ..Default::default()
                    }}
                />
            </NinePatchBundle>
        };
    }
    true
}

#[derive(Default, Resource)]
pub struct PreloadResource {
    images: Vec<Handle<Image>>,
}

pub fn spawn_main_menu(
    mut commands: Commands,
    mut font_mapping: ResMut<FontMapping>,
    asset_server: Res<AssetServer>,
    mut preload_resource: ResMut<PreloadResource>,
) {
    font_mapping.set_default(asset_server.load("lato-light.kttf"));

    let mut widget_context = KayakRootContext::new();
    widget_context.add_plugin(KayakWidgetsContextPlugin);
    widget_context.add_widget_data::<MenuButton, ButtonState>();
    widget_context.add_widget_system(
        MenuButton::default().get_name(),
        widget_update::<MenuButton, ButtonState>,
        menu_button_render,
    );

    let panel1_image = asset_server.load("main_menu/panel1.png");
    let button_image = asset_server.load("main_menu/button.png");
    let button_image_hover = asset_server.load("main_menu/button-hover.png");

    preload_resource.images.extend(vec![
        panel1_image.clone(),
        button_image.clone(),
        button_image_hover.clone(),
    ]);

    let handle_click_close = OnEvent::new(
        move |In((event_dispatcher_context, _, event, _entity)): In<(
            EventDispatcherContext,
            WidgetState,
            Event,
            Entity,
        )>,
              mut exit: EventWriter<AppExit>| {
            match event.event_type {
                EventType::Click(..) => {
                    exit.send(AppExit);
                }
                _ => {}
            }
            (event_dispatcher_context, event)
        },
    );

    let handle_switch_state = OnEvent::new(
        move |In((event_dispatcher_context, _, event, _entity)): In<(
            EventDispatcherContext,
            WidgetState,
            Event,
            Entity,
        )>,
              mut commands: Commands| {
            match event.event_type {
                EventType::Click(..) => {
                    commands.insert_resource(NextState(GameState::InGame))
                }
                _ => {}
            }
            (event_dispatcher_context, event)
        },
    );

    let parent_id = None;
    rsx! {
        <KayakAppBundle>
            <NinePatchBundle
                nine_patch={NinePatch {
                    handle: panel1_image,
                    border: Edge::all(25.0),
                }}
                styles={KStyle {
                    width: Units::Pixels(350.0).into(),
                    height: Units::Pixels(200.0).into(),
                    left: Units::Stretch(1.0).into(),
                    right: Units::Stretch(1.0).into(),
                    top: Units::Stretch(1.0).into(),
                    bottom: Units::Stretch(1.0).into(),
                    padding: Edge::new(
                        Units::Pixels(50.0),
                        Units::Pixels(20.0),
                        Units::Pixels(20.0),
                        Units::Pixels(20.0),
                    ).into(),

                    ..KStyle::default()
                }}
            >
                <MenuButtonBundle button={MenuButton { text: "Play".into() }} on_event={handle_switch_state} />
                <MenuButtonBundle
                    button={MenuButton { text: "Quit".into() }}
                    on_event={handle_click_close}
                />
            </NinePatchBundle>
        </KayakAppBundle>
    };

    commands.spawn(UICameraBundle::new(widget_context));
}
