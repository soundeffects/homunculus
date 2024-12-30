use bevy::{prelude::*, window::CursorGrabMode};
use leafwing_input_manager::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<GeneralInput>::default())
            .init_resource::<ActionState<GeneralInput>>()
            .insert_resource(GeneralInput::personalized_input_map())
            .add_systems(Startup, hide_cursor_on_setup)
            .add_systems(Update, toggle_cursor_visibility);
    }
}

#[derive(Actionlike, Reflect, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum GeneralInput {
    #[actionlike(DualAxis)]
    Move,
    #[actionlike(DualAxis)]
    PanCamera,
    #[actionlike(Axis)]
    Zoom,
    Escape,
}

impl GeneralInput {
    pub fn default_input_map() -> InputMap<Self> {
        InputMap::default()
            .with_dual_axis(Self::Move, GamepadStick::LEFT)
            .with_dual_axis(Self::PanCamera, GamepadStick::RIGHT)
            .with_axis(Self::Zoom, VirtualAxis::dpad_y())
            .with(Self::Escape, GamepadButton::Select)
            .with_dual_axis(Self::Move, VirtualDPad::wasd())
            .with_dual_axis(Self::PanCamera, MouseMove::default())
            .with_axis(Self::Zoom, MouseScrollAxis::Y)
            .with(Self::Escape, KeyCode::Escape)
    }

    pub fn personalized_input_map() -> InputMap<Self> {
        let mut map = Self::default_input_map();
        map.clear_action(&Self::Move);
        map.with_dual_axis(
            Self::Move,
            VirtualDPad::new(KeyCode::KeyH, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyT),
        )
    }
}

fn hide_cursor_on_setup(mut windows: Query<&mut Window>) {
    let mut window = windows.single_mut();
    window.cursor_options.grab_mode = CursorGrabMode::Confined;
    window.cursor_options.visible = false;
}

fn toggle_cursor_visibility(
    action_state: Res<ActionState<GeneralInput>>,
    mut windows: Query<&mut Window>,
) {
    if action_state.just_pressed(&GeneralInput::Escape) {
        let mut window = windows.single_mut();
        if window.cursor_options.grab_mode == CursorGrabMode::Confined {
            window.cursor_options.grab_mode = CursorGrabMode::None;
            window.cursor_options.visible = true;
        } else {
            window.cursor_options.grab_mode = CursorGrabMode::Confined;
            window.cursor_options.visible = false;
        }
    }
}
