use bevy::prelude::*;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

/// We define the two core States of our game.
/// In Bevy, States control what systems run at what time.
#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    /// The game starts in the Lobby (where we can tweak unit data safely)
    #[default]
    Lobby,
    /// The actual RTS simulation
    InGame,
}

/// This is our first core ECS Component representing a Unit's basic data.
/// By deriving `Reflect`, we are telling Rust to expose these variables
/// to the engine at runtime.
/// By deriving `InspectorOptions` (from bevy-inspector-egui), we can even 
/// define the bounds of the sliders in the UI!
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct UnitStats {
    pub max_health: f32,
    pub current_health: f32,
    pub movement_speed: f32,
    pub build_cost: u32,
}

/// A simple tag component so we know which team a unit belongs to.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct Team(pub u8);

/// This component controls our RTS camera. 
/// We attach it to the `Camera2d` entity so that our camera system
/// knows how fast to move or zoom this specific viewpoint.
#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct PanOrbitCamera {
    pub pan_speed: f32,
    pub zoom_speed: f32,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        Self {
            pan_speed: 500.0,
            zoom_speed: 0.1,
        }
    }
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        
        // 1. We initialize our State machine.
        .init_state::<GameState>()
        
        // 2. The Inspector UI requires the base `EguiPlugin` from `bevy_egui`
        // to be initialized first before it can draw its windows.
        .add_plugins(bevy_egui::EguiPlugin::default())
        .add_plugins(WorldInspectorPlugin::new())
        
        // 3. We must register every component we want to appear in the Inspector UI.
        .register_type::<UnitStats>()
        .register_type::<Team>()
        .register_type::<PanOrbitCamera>()
        
        // 4. We run our startup logic once.
        .add_systems(Startup, setup)
        
        // 5. This is our first "Update" system. It runs every single frame.
        // If the game runs at 60 FPS, this function is called 60 times a second.
        .add_systems(Update, camera_movement_system)
        
        .run();
}

fn setup(mut commands: Commands) {
    // We spawn a 2D camera as our basic viewport and attach our custom
    // `PanOrbitCamera` component to it so the movement system can find it.
    commands.spawn((
        Camera2d,
        PanOrbitCamera::default(),
        Name::new("Main Camera"),
    ));

    // Let's spawn a test unit just so we can see it in our new Development UI!
    commands.spawn((
        // The display name in the entity tree
        Name::new("Test Commander Unit"),
        
        // Our custom components!
        UnitStats {
            max_health: 1500.0,
            current_health: 1500.0,
            movement_speed: 4.5,
            build_cost: 1000,
        },
        Team(1),
        
        // A basic transform so it exists somewhere in the world
        Transform::default(),
    ));
}

/// A Bevy "System" is just a standard Rust function.
/// We define what data we want to modify in the parameters.
/// Here, we want `Time` (to know how long a frame took),
/// `ButtonInput` & `EventReader` (to read the keyboard and mouse wheel),
/// and a `Query` (to find all entities that have both a `PanOrbitCamera` AND a `Transform`).
fn camera_movement_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut scroll_events: bevy::ecs::message::MessageReader<MouseWheel>,
    // The `Query` says: "Give me the mutable Transform and read-only PanOrbitCamera of everything that has a PanOrbitCamera."
    // `OrthographicProjection` is automatically added by `Camera2d` and controls the zoom level.
    mut query: Query<(&PanOrbitCamera, &mut Transform, &mut Projection)>,
) {
    // We loop through all cameras in the world. Usually there is only 1.
    for (camera, mut transform, mut projection) in query.iter_mut() {
        // 1. Calculate Panning (WASD or Arrows)
        let mut direction = Vec3::ZERO;
        
        if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }

        // Normalize the vector so moving diagonally isn't faster than moving straight
        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
        }

        // Apply panning. We multiply by `time.delta_secs()` so the camera moves
        // exactly `pan_speed` pixels per second, regardless of whether the game is 
        // running at 30 FPS or 144 FPS. This is called "Frame-rate Independence".
        transform.translation += direction * camera.pan_speed * time.delta_secs();

        // 2. Calculate Zooming (Mouse Wheel)
        let mut scroll = 0.0;
        for ev in scroll_events.read() {
            match ev.unit {
                MouseScrollUnit::Line => scroll += ev.y,
                MouseScrollUnit::Pixel => scroll += ev.y * 0.01,
            }
        }

        if scroll != 0.0 {
            // Apply zoom. (A smaller `scale` value zooms IN, a larger value zooms OUT).
            // `f32::exp` creates a smooth logarithmic zoom (feels much better than linear math).
            if let Projection::Orthographic(ref mut ortho) = *projection {
                ortho.scale *= f32::exp(-scroll * camera.zoom_speed);
                
                // Clamp the zoom so we don't zoom in to the atomic level or out past the universe.
                ortho.scale = ortho.scale.clamp(0.1, 5.0);
            }
        }
    }
}


