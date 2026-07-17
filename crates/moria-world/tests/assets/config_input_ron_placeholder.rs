use std::{fs, path::Path};

use moria_world::presentation::{AssetId, AssetLoader, AssetMissingAction, RuntimeAssetProfile};
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
struct InputConfig {
    schema_version: u16,
    bindings: Vec<InputBinding>,
    stick_dead_zone: f32,
    mouse_orbit_sensitivity_degrees_per_unit: f32,
    gamepad_orbit_sensitivity_degrees_per_second: f32,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct InputBinding {
    physical: PhysicalControl,
    action: SemanticAction,
    value: ActionValue,
    modifier: Option<InputModifier>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
enum PhysicalControl {
    Keyboard(KeyboardControl),
    Mouse(MouseControl),
    Gamepad(GamepadControl),
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
enum KeyboardControl {
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
    Equal,
    Tab,
    F5,
    F9,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
enum MouseControl {
    CapturedMotion,
    Wheel,
    LeftButton,
    RightButton,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
enum GamepadControl {
    LeftStick,
    LeftStickPress,
    South,
    RightStick,
    TriggerDifference,
    RightShoulder,
    LeftShoulder,
    DPadLeft,
    DPadRight,
    DPadDown,
    DPadUp,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
enum SemanticAction {
    Player(PlayerAction),
    Debug(DebugAction),
    Ui(UiAction),
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
enum PlayerAction {
    Move,
    Sprint,
    Jump,
    Orbit,
    Zoom,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
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

#[derive(Debug, Deserialize, PartialEq, Eq)]
enum UiAction {
    AdjustTimeOfDay,
    ToggleTimeSliderFocus,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
enum ActionValue {
    NegativeX,
    PositiveX,
    NegativeY,
    PositiveY,
    Continuous,
    Pressed,
    Negative,
    Positive,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
enum InputModifier {
    Debug,
}

#[test]
fn input_config_placeholder_uses_the_declared_required_runtime_path() {
    let loader = AssetLoader::new();
    let declaration = loader.declaration(AssetId::InputConfig);

    assert_eq!(declaration.id.stable_id(), "moria.config.input");
    assert_eq!(declaration.path, "config/input.ron");
    assert_eq!(
        loader.resolve_runtime_path(declaration.path),
        Ok(declaration)
    );
    assert_eq!(
        loader.validation_fixture(AssetId::InputConfig).key,
        "moria.config.input"
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::InputConfig, RuntimeAssetProfile::Development),
        AssetMissingAction::Fatal
    );
    assert_eq!(
        loader.missing_asset_action(AssetId::InputConfig, RuntimeAssetProfile::Release),
        AssetMissingAction::Fatal
    );
}

#[test]
fn input_config_placeholder_maps_every_documented_physical_control() {
    let asset_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets")
        .join("config/input.ron");
    let input: InputConfig = ron::from_str(
        &fs::read_to_string(asset_path)
            .expect("input placeholder exists at its declared runtime path"),
    )
    .expect("input placeholder uses the centralized physical-to-semantic schema");

    assert_eq!(input.schema_version, 1);
    assert_eq!(input.stick_dead_zone, 0.15);
    assert_eq!(input.mouse_orbit_sensitivity_degrees_per_unit, 0.12);
    assert_eq!(input.gamepad_orbit_sensitivity_degrees_per_second, 150.0);
    assert_eq!(input.bindings, expected_bindings());
}

fn expected_bindings() -> Vec<InputBinding> {
    use ActionValue::{
        Continuous, Negative, NegativeX, NegativeY, Positive, PositiveX, PositiveY, Pressed,
    };
    use DebugAction::{
        Dig, Load, Place, Save, SelectNextMaterial, SelectPreviousMaterial, ToggleBrickBounds,
        ToggleRawVoxels, ToggleStreamingBands,
    };
    use GamepadControl::{
        DPadDown, DPadLeft, DPadRight, DPadUp, LeftShoulder, LeftStick, LeftStickPress,
        RightShoulder, RightStick, South, TriggerDifference,
    };
    use InputModifier::Debug as DebugModifier;
    use KeyboardControl::{
        A, D, Equal, F1, F2, F3, F5, F9, G, LeftBracket, LeftShift, Minus, P, RightBracket, S,
        Space, Tab, W,
    };
    use MouseControl::{CapturedMotion, LeftButton, RightButton, Wheel};
    use PhysicalControl::{Gamepad, Keyboard, Mouse};
    use PlayerAction::{Jump, Move, Orbit, Sprint, Zoom};
    use SemanticAction::{Debug, Player, Ui};
    use UiAction::{AdjustTimeOfDay, ToggleTimeSliderFocus};

    let binding = |physical, action, value| InputBinding {
        physical,
        action,
        value,
        modifier: None,
    };
    let debug_binding = |physical, value| InputBinding {
        physical,
        action: Ui(AdjustTimeOfDay),
        value,
        modifier: Some(DebugModifier),
    };

    vec![
        binding(Keyboard(W), Player(Move), PositiveY),
        binding(Keyboard(A), Player(Move), NegativeX),
        binding(Keyboard(S), Player(Move), NegativeY),
        binding(Keyboard(D), Player(Move), PositiveX),
        binding(Keyboard(LeftShift), Player(Sprint), Pressed),
        binding(Keyboard(Space), Player(Jump), Pressed),
        binding(Mouse(CapturedMotion), Player(Orbit), Continuous),
        binding(Mouse(Wheel), Player(Zoom), Continuous),
        binding(Keyboard(G), Debug(Dig), Pressed),
        binding(Mouse(LeftButton), Debug(Dig), Pressed),
        binding(Keyboard(P), Debug(Place), Pressed),
        binding(Mouse(RightButton), Debug(Place), Pressed),
        binding(
            Keyboard(LeftBracket),
            Debug(SelectPreviousMaterial),
            Pressed,
        ),
        binding(Keyboard(RightBracket), Debug(SelectNextMaterial), Pressed),
        binding(Keyboard(F1), Debug(ToggleBrickBounds), Pressed),
        binding(Keyboard(F2), Debug(ToggleRawVoxels), Pressed),
        binding(Keyboard(F3), Debug(ToggleStreamingBands), Pressed),
        binding(Keyboard(Minus), Ui(AdjustTimeOfDay), Negative),
        binding(Keyboard(Equal), Ui(AdjustTimeOfDay), Positive),
        binding(Keyboard(Tab), Ui(ToggleTimeSliderFocus), Pressed),
        binding(Keyboard(F5), Debug(Save), Pressed),
        binding(Keyboard(F9), Debug(Load), Pressed),
        binding(Gamepad(LeftStick), Player(Move), Continuous),
        binding(Gamepad(LeftStickPress), Player(Sprint), Pressed),
        binding(Gamepad(South), Player(Jump), Pressed),
        binding(Gamepad(RightStick), Player(Orbit), Continuous),
        binding(Gamepad(TriggerDifference), Player(Zoom), Continuous),
        binding(Gamepad(RightShoulder), Debug(Dig), Pressed),
        binding(Gamepad(LeftShoulder), Debug(Place), Pressed),
        binding(Gamepad(DPadLeft), Debug(SelectPreviousMaterial), Pressed),
        binding(Gamepad(DPadRight), Debug(SelectNextMaterial), Pressed),
        debug_binding(Gamepad(DPadDown), Negative),
        debug_binding(Gamepad(DPadUp), Positive),
    ]
}
