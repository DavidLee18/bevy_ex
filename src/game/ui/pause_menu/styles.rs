use bevy::prelude::*;

pub const PAUSE_MENU_STYLE: Style = Style {
    align_items: AlignItems::Center,
    flex_direction: FlexDirection::Column,
    gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
    justify_content: JustifyContent::Center,
    margin: UiRect::all(Val::Percent(25.0)),
    size: Size::new(Val::Percent(50.0), Val::Percent(50.0)),
    ..Style::DEFAULT
};

pub fn get_text_bundle(asset_server: &Res<AssetServer>, text: &str, font_size: f32) -> TextBundle {
    TextBundle {
        text: Text {
            sections: vec![ TextSection::new(text, TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size,
                color: Color::WHITE,
            }) ],
            alignment: TextAlignment::Center,
            ..Default::default()
        },
        ..Default::default()
    }
}