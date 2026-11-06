use bevy::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let modles = asset_server.load(GltfAssetLabel::Scene(0).from_asset("dummy/models.glb"));

    commands.spawn(WorldAssetRoot(modles));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0., 3., 10.).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(0., 1., 0.25),
    ));
}
