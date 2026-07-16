use std::{fs, path::Path};

use moria_world::presentation::{
    AssetId, AssetLoadPolicy, AssetLoader, AssetMissingAction, RuntimeAssetProfile,
};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct InputConfig {
    stick_dead_zone: f32,
    mouse_sensitivity_degrees_per_unit: f32,
    gamepad_orbit_degrees_per_second: f32,
    bindings: Vec<InputBinding>,
}

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct InputBinding {
    physical: PhysicalControl,
    semantic: SemanticAction,
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
enum PhysicalControl {
    Keyboard(Key),
    MouseButton(MouseButton),
    MouseMotion,
    MouseWheel,
    GamepadButton(GamepadButton),
    GamepadAxis(GamepadAxis),
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
enum Key {
    W,
    A,
    S,
    D,
    LeftShift,
    Space,
    G,
    P,
    LeftBracket,
    RightBracket,
    F1,
    F2,
    F3,
    Minus,
    Equals,
    Tab,
    F5,
    F9,
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
enum MouseButton {
    Left,
    Right,
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
enum GamepadButton {
    South,
    LeftStick,
    RightShoulder,
    LeftShoulder,
    DPadLeft,
    DPadRight,
    DPadDown,
    DPadUp,
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
enum GamepadAxis {
    LeftStickX,
    LeftStickY,
    RightStickX,
    RightStickY,
    LeftTrigger,
    RightTrigger,
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
enum SemanticAction {
    Player(PlayerAction),
    Debug(DebugAction),
    Ui(UiAction),
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
enum PlayerAction {
    MoveForward,
    MoveLeft,
    MoveBackward,
    MoveRight,
    Sprint,
    Jump,
    OrbitX,
    OrbitY,
    ZoomIn,
    ZoomOut,
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
enum DebugAction {
    Dig,
    Place,
    SelectPreviousMaterial,
    SelectNextMaterial,
    ToggleBrickBounds,
    ToggleRawVoxels,
    ToggleStreamingBands,
    Save,
    Load,
}

#[derive(Debug, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
enum UiAction {
    AdjustTimeOfDayDown,
    AdjustTimeOfDayUp,
    ToggleTimeSliderFocus,
}

#[test]
fn input_config_placeholder_uses_the_declared_runtime_path_and_required_policy() {
    let loader = AssetLoader::new();
    let declaration = loader.declaration(AssetId::InputConfig);

    assert_eq!(declaration.id.stable_id(), "moria.config.input");
    assert_eq!(declaration.path, "config/input.ron");
    assert_eq!(declaration.load_policy, AssetLoadPolicy::Required);
    assert_eq!(
        loader.resolve_runtime_path(declaration.path),
        Ok(declaration)
    );
    assert_eq!(
        loader.validation_fixture(AssetId::InputConfig).key,
        declaration.id.stable_id()
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::InputConfig, RuntimeAssetProfile::Development),
        AssetMissingAction::Fatal
    );
}

#[test]
fn input_config_placeholder_maps_every_documented_physical_control_to_a_semantic_action() {
    let config: InputConfig = ron::de::from_str(
        &fs::read_to_string(asset_path()).expect("input placeholder must exist at its final path"),
    )
    .expect("input placeholder must use the documented RON schema");

    assert_eq!(config.stick_dead_zone, 0.15);
    assert_eq!(config.mouse_sensitivity_degrees_per_unit, 0.12);
    assert_eq!(config.gamepad_orbit_degrees_per_second, 150.0);

    let bindings: Vec<_> = config
        .bindings
        .iter()
        .map(|binding| (&binding.physical, &binding.semantic))
        .collect();
    assert_eq!(
        bindings.len(),
        36,
        "each documented physical control has one mapping"
    );
    assert!(bindings.contains(&(
        &PhysicalControl::Keyboard(Key::W),
        &SemanticAction::Player(PlayerAction::MoveForward)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::Keyboard(Key::A),
        &SemanticAction::Player(PlayerAction::MoveLeft)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::Keyboard(Key::S),
        &SemanticAction::Player(PlayerAction::MoveBackward)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::Keyboard(Key::D),
        &SemanticAction::Player(PlayerAction::MoveRight)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::GamepadAxis(GamepadAxis::LeftStickX),
        &SemanticAction::Player(PlayerAction::MoveRight)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::GamepadAxis(GamepadAxis::LeftStickY),
        &SemanticAction::Player(PlayerAction::MoveForward)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::Keyboard(Key::LeftShift),
        &SemanticAction::Player(PlayerAction::Sprint)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::GamepadButton(GamepadButton::LeftStick),
        &SemanticAction::Player(PlayerAction::Sprint)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::Keyboard(Key::Space),
        &SemanticAction::Player(PlayerAction::Jump)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::GamepadButton(GamepadButton::South),
        &SemanticAction::Player(PlayerAction::Jump)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::MouseMotion,
        &SemanticAction::Player(PlayerAction::OrbitX)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::GamepadAxis(GamepadAxis::RightStickX),
        &SemanticAction::Player(PlayerAction::OrbitX)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::GamepadAxis(GamepadAxis::RightStickY),
        &SemanticAction::Player(PlayerAction::OrbitY)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::MouseWheel,
        &SemanticAction::Player(PlayerAction::ZoomIn)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::GamepadAxis(GamepadAxis::RightTrigger),
        &SemanticAction::Player(PlayerAction::ZoomIn)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::GamepadAxis(GamepadAxis::LeftTrigger),
        &SemanticAction::Player(PlayerAction::ZoomOut)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::Keyboard(Key::G),
        &SemanticAction::Debug(DebugAction::Dig)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::MouseButton(MouseButton::Left),
        &SemanticAction::Debug(DebugAction::Dig)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::GamepadButton(GamepadButton::RightShoulder),
        &SemanticAction::Debug(DebugAction::Dig)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::Keyboard(Key::P),
        &SemanticAction::Debug(DebugAction::Place)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::MouseButton(MouseButton::Right),
        &SemanticAction::Debug(DebugAction::Place)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::GamepadButton(GamepadButton::LeftShoulder),
        &SemanticAction::Debug(DebugAction::Place)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::Keyboard(Key::LeftBracket),
        &SemanticAction::Debug(DebugAction::SelectPreviousMaterial)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::GamepadButton(GamepadButton::DPadLeft),
        &SemanticAction::Debug(DebugAction::SelectPreviousMaterial)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::Keyboard(Key::RightBracket),
        &SemanticAction::Debug(DebugAction::SelectNextMaterial)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::GamepadButton(GamepadButton::DPadRight),
        &SemanticAction::Debug(DebugAction::SelectNextMaterial)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::Keyboard(Key::F1),
        &SemanticAction::Debug(DebugAction::ToggleBrickBounds)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::Keyboard(Key::F2),
        &SemanticAction::Debug(DebugAction::ToggleRawVoxels)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::Keyboard(Key::F3),
        &SemanticAction::Debug(DebugAction::ToggleStreamingBands)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::Keyboard(Key::Minus),
        &SemanticAction::Ui(UiAction::AdjustTimeOfDayDown)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::GamepadButton(GamepadButton::DPadDown),
        &SemanticAction::Ui(UiAction::AdjustTimeOfDayDown)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::Keyboard(Key::Equals),
        &SemanticAction::Ui(UiAction::AdjustTimeOfDayUp)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::GamepadButton(GamepadButton::DPadUp),
        &SemanticAction::Ui(UiAction::AdjustTimeOfDayUp)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::Keyboard(Key::Tab),
        &SemanticAction::Ui(UiAction::ToggleTimeSliderFocus)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::Keyboard(Key::F5),
        &SemanticAction::Debug(DebugAction::Save)
    )));
    assert!(bindings.contains(&(
        &PhysicalControl::Keyboard(Key::F9),
        &SemanticAction::Debug(DebugAction::Load)
    )));
}

fn asset_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets")
        .join("config/input.ron")
}
