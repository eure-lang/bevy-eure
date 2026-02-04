use bevy::prelude::*;
use bevy_eure::EureAssetPlugin;
use eure::FromEure;

#[derive(Asset, FromEure, TypePath, PartialEq)]
struct Script {
    actions: Vec<Action>,
}

#[derive(Asset, FromEure, TypePath, PartialEq)]
#[eure(rename_all = "kebab-case")]
enum Action {
    Push(String),
}

#[derive(Resource)]
struct ScriptHandle(Option<Handle<Script>>);

fn main() {
    std::fs::write(
        "assets/test.script.eure",
        r#"
@ actions[]: Hello, world!
$variant: push
	"#,
    )
    .unwrap();
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EureAssetPlugin::<Script>::new(&["script.eure"]))
        .insert_resource(ScriptHandle(None))
        .add_systems(
            Startup,
            |asset_server: Res<AssetServer>, mut script_handle: ResMut<ScriptHandle>| {
                script_handle.0 = Some(asset_server.load("test.script.eure"));
            },
        )
        .add_systems(
            Update,
            |mut events: MessageReader<AssetEvent<Script>>,
             scripts: Res<Assets<Script>>,
             mut app_events: MessageWriter<AppExit>| {
                for event in events.read() {
                    match event {
                        AssetEvent::Added { id } => {
                            let script = scripts.get(*id).unwrap();
                            for action in &script.actions {
                                match action {
                                    Action::Push(message) => println!("{}", message),
                                }
                            }
                            std::fs::write(
                                "assets/test.script.eure",
                                r#"
@ actions[]: Modified
$variant: push
							"#,
                            )
                            .unwrap();
                        }
                        AssetEvent::Modified { id } => {
                            let script = scripts.get(*id).unwrap();
                            for action in &script.actions {
                                match action {
                                    Action::Push(message) => {
                                        println!("{}", message);
                                        app_events.write(AppExit::Success);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            },
        )
        .run();
}
