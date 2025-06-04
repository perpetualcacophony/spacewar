use bevy::prelude::*;

macro_rules! line_enum {
    ($($variant:ident $title:literal)*) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Component)]
        pub enum Line {
            $($variant),*
        }

        impl Line {
            pub const VARIANTS: &[Self] = &[$(
                Self::$variant
            ),*];

            pub const fn title(self) -> &'static str {
                match self {
                    $(
                        Self::$variant => $title
                    ),*
                }
            }
        }
    };
}

line_enum! {
    Heading "Heading"
    AngVel "Angular velocity"
    Speed "Speed"
    Zoom "Zoom"
    Fps "Frames per second"
}

impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: ", self.title())
    }
}

impl From<&Line> for String {
    fn from(value: &Line) -> Self {
        value.to_string()
    }
}

#[derive(Default, Clone, Copy, Debug, Component)]
#[require(Node)]
pub struct DebugInfo;

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Regular.ttf");

    let root = commands
        .spawn((
            DebugInfo,
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(15.0),
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            TextFont {
                font: font.clone(),
                ..Default::default()
            },
        ))
        .id();

    for line in Line::VARIANTS {
        commands
            .spawn((ChildOf(root), Text::new(line), Node::default()))
            .with_child((
                TextSpan::default(),
                *line,
                TextFont {
                    font: font.clone_weak(),
                    ..Default::default()
                },
            ));
    }
}

pub fn update(
    text: Query<(&Line, &mut TextSpan)>,
    ship: Single<(&crate::Ship, &crate::Transform)>,
    camera: Single<&Projection, With<Camera2d>>,
    diagnostics: Res<bevy::diagnostic::DiagnosticsStore>,
) {
    let (ship, ship_transform) = ship.into_inner();
    let projection = match **camera {
        Projection::Orthographic(ref projection) => projection,
        _ => unimplemented!(),
    };

    for (line, mut text) in text {
        text.0 = match *line {
            Line::Heading => format!(
                "{:.2}°",
                360.0 - {
                    let rot = ship_transform
                        .rotation
                        .angle_to(Rot2::IDENTITY)
                        .to_degrees();
                    if rot.is_sign_positive() {
                        rot
                    } else {
                        rot + 360.0
                    }
                }
            ),
            Line::AngVel => format!("{:.0}°/sec", ship.rotational_velocity),
            Line::Speed => format!("{:.2}", ship.velocity.length()),
            Line::Zoom => format!("{:.2}x", 1.0 / projection.scale),
            Line::Fps => format!(
                "{:.2}",
                diagnostics
                    .get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS)
                    .and_then(|d| d.average())
                    .unwrap_or_default()
            ),
        };
    }
}
