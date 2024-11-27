use bevy::{prelude::*, window::CursorGrabMode};
use leafwing_input_manager::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<UserAction>::default())
            .init_resource::<ActionState<UserAction>>()
            .insert_resource(UserAction::personalized_input_map())
            .add_systems(Startup, hide_cursor_on_setup)
            .add_systems(Update, toggle_cursor_visibility);
    }
}

#[derive(Actionlike, Reflect, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum UserAction {
    #[actionlike(DualAxis)]
    Move,
    #[actionlike(DualAxis)]
    PanCamera,
    #[actionlike(Axis)]
    Zoom,
    Escape,
}

impl UserAction {
    pub fn default_input_map() -> InputMap<Self> {
        InputMap::default()
            .with_dual_axis(Self::Move, GamepadStick::LEFT)
            .with_dual_axis(Self::PanCamera, GamepadStick::RIGHT)
            .with_axis(Self::Zoom, GamepadVirtualAxis::DPAD_Y)
            .with(Self::Escape, GamepadButtonType::Select)
            .with_dual_axis(Self::Move, KeyboardVirtualDPad::WASD)
            .with_dual_axis(Self::PanCamera, MouseMove::default())
            .with_axis(Self::Zoom, MouseScrollAxis::Y)
            .with(Self::Escape, KeyCode::Escape)
    }

    pub fn personalized_input_map() -> InputMap<Self> {
        let mut map = Self::default_input_map();
        map.clear_action(&Self::Move);
        map.with_dual_axis(
            Self::Move,
            KeyboardVirtualDPad::new(KeyCode::KeyH, KeyCode::KeyS, KeyCode::KeyT, KeyCode::KeyA),
        )
    }
}

fn hide_cursor_on_setup(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor.grab_mode = CursorGrabMode::Confined;
    window.cursor.visible = false;
}

fn toggle_cursor_visibility(
    action_state: Res<ActionState<UserAction>>,
    mut windows: Query<&mut Window>,
) {
    if action_state.just_pressed(&UserAction::Escape) {
        let mut window = windows.single_mut();
        if window.cursor.grab_mode == CursorGrabMode::Confined {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
        } else {
            window.cursor.grab_mode = CursorGrabMode::Confined;
            window.cursor.visible = false;
        }
    }
}
