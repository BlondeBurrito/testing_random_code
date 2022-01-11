use bevy::core::FixedTimestep;
use bevy::prelude::*;

const TIME_STEP: f64 = 1.0;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum AppState {
	Go,
	Stop,
}

fn main() {
	App::new()
		.add_plugins(DefaultPlugins)
		.add_state(AppState::Go)
		.add_startup_system(setup)
		.add_system_set(SystemSet::on_update(AppState::Go).with_system(button_system))
		// Scenario 1: run the app and click the button, IDManager::new() continues logging even though state has changed
		.add_system_set(
			SystemSet::on_update(AppState::Go)
				.with_run_criteria(FixedTimestep::step(TIME_STEP))
				.with_system(generate_ids),
		)
		// Scenario 2: uncomment below, comment the above system set, clicking the button stops logging
		// .add_system_set(SystemSet::on_update(AppState::Go).with_system(generate_ids))
		.run();
}

/// Stores the last generated unique ID
#[derive(Component)]
pub struct IDManager {
	current_id: usize,
}
impl Default for IDManager {
	fn default() -> Self {
		IDManager { current_id: 0 }
	}
}
impl IDManager {
	/// Creates a new unique ID
	fn new(&mut self) {
		self.current_id = self.current_id.overflowing_add(1).0;
		info!("Unique ID generated: {}", self.current_id);
	}
}
/// Spawn the IDManager entity, UI camera and button entity
fn setup(mut cmds: Commands, asset_server: Res<AssetServer>) {
	info!("Creating ID Manager");
	cmds.spawn().insert(IDManager::default());
	info!("Creating UI camera");
	cmds.spawn_bundle(UiCameraBundle::default());
	info!("Creating button");
	cmds.spawn_bundle(ButtonBundle {
		style: Style {
			size: Size::new(Val::Px(150.0), Val::Px(65.0)),
			margin: Rect::all(Val::Auto),
			justify_content: JustifyContent::Center,
			align_items: AlignItems::Center,
			..Default::default()
		},
		color: Color::rgb(0.15, 0.15, 0.15).into(),
		..Default::default()
	})
	.with_children(|parent| {
		parent.spawn_bundle(TextBundle {
			text: Text::with_section(
				"Stop state",
				TextStyle {
					font: asset_server.load("fonts/FiraSans-Bold.ttf"),
					font_size: 40.0,
					color: Color::rgb(0.9, 0.9, 0.9),
				},
				Default::default(),
			),
			..Default::default()
		});
	});
}
/// System handling button logic
fn button_system(
	mut interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
	mut state: ResMut<State<AppState>>,
) {
	for interaction in interaction_query.iter_mut() {
		match *interaction {
			Interaction::Clicked => {
				info!("Changing state to Stop");
				state.set(AppState::Stop).unwrap();
			}
			_ => {}
		}
	}
}
/// Each time step create a new ID, the impl prints it out
fn generate_ids(mut query: Query<&mut IDManager>) {
	query.single_mut().new();
}
