use bevy::prelude::*;
/*
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
} */

#[derive(Debug, Clone, Component)]
pub enum Line2 {
    Heading(f32),
    AngVel(f32),
    Speed(f32),
    Zoom(f32),
    Fps(f32),
}

impl Line2 {
    const HEADING: Self = Self::Heading(0.0);
    const ANG_VEL: Self = Self::AngVel(0.0);
    const SPEED: Self = Self::Speed(0.0);
    const ZOOM: Self = Self::Zoom(1.0);
    const FPS: Self = Self::Fps(0.0);

    pub const fn field(&self) -> &'static str {
        match self {
            Self::Heading(_) => "Heading",
            Self::AngVel(_) => "Angular velocity",
            Self::Speed(_) => "Speed",
            Self::Zoom(_) => "Zoom",
            Self::Fps(_) => "Frames per second",
        }
    }

    pub fn fmt_value(&self) -> String {
        match self {
            Self::Heading(v) => format!("{v:.2}°"),
            Self::AngVel(v) => format!("{v:.0}°/sec"),
            Self::Speed(v) => format!("{v:.2}"),
            Self::Zoom(v) => format!("{v:.2}"),
            Self::Fps(v) => format!("{v:.2}"),
        }
    }
}

impl std::fmt::Display for Line2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.field(), self.fmt_value())
    }
}

/* impl std::fmt::Display for Line {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: ", self.title())
    }
}

impl From<&Line> for String {
    fn from(value: &Line) -> Self {
        value.to_string()
    }
}
 */
#[derive(Default, Clone, Copy, Debug, Component)]
#[require(Node)]
pub struct DebugInfo;

pub fn spawn(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/FiraMono-Regular.ttf");

    commands
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
        .with_child(Line2::HEADING)
        .with_child(Line2::ANG_VEL)
        .with_child(Line2::SPEED)
        .with_child(Line2::ZOOM)
        .with_child(Line2::FPS);
    /*
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
    } */
}

pub fn post_spawn(
    mut commands: Commands,
    lines: Query<(Entity, &Line2), With<Line2>>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraMono-Regular.ttf");

    for (entity, line) in lines {
        commands
            .entity(entity)
            .insert((Text::new(format!("{}: ", line.field())), Node::default()))
            .with_child((
                TextSpan::default(),
                TextFont {
                    font: font.clone(),
                    ..Default::default()
                },
            ));
    }
}

/* pub fn update(
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
} */

pub fn update2(
    lines: Query<(&mut Line2, &Children)>,
    mut spans: Query<&mut TextSpan>,
    ship: Single<(&crate::Ship, &crate::Transform)>,
    camera: Single<&Projection, With<Camera2d>>,
    diagnostics: Res<bevy::diagnostic::DiagnosticsStore>,
) {
    let (ship, ship_transform) = ship.into_inner();
    let projection = match **camera {
        Projection::Orthographic(ref projection) => projection,
        _ => unimplemented!(),
    };

    for (mut line, children) in lines {
        let Some(child_span_id) = children.iter().next() else {
            continue;
        };
        if let Ok(mut span) = spans.get_mut(child_span_id) {
            span.0 = line.fmt_value();
        }

        match *line {
            Line2::Heading(ref mut heading) => {
                *heading = 360.0 - {
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
            }
            Line2::AngVel(ref mut angular_velocity) => *angular_velocity = ship.rotational_velocity,
            Line2::Speed(ref mut speed) => *speed = ship.velocity.length(),
            Line2::Zoom(ref mut zoom) => *zoom = 1.0 / projection.scale,
            Line2::Fps(ref mut fps) => {
                *fps = diagnostics
                    .get(&bevy::diagnostic::FrameTimeDiagnosticsPlugin::FPS)
                    .and_then(|d| d.average())
                    .unwrap_or_default() as f32
            }
        }
    }
}
