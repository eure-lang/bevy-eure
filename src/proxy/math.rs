use bevy_math::Quat;
use eure::{FromEure, IntoEure};

#[derive(FromEure, IntoEure)]
#[eure(proxy = "bevy_math::Vec3")]
pub struct Vec3Proxy {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(FromEure, IntoEure)]
#[eure(proxy = "bevy_math::Vec2")]
pub struct Vec2Proxy {
    pub x: f32,
    pub y: f32,
}

#[derive(FromEure, IntoEure)]
#[eure(proxy = "bevy_math::UVec3")]
pub struct UVec3Proxy {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

#[derive(FromEure, IntoEure)]
#[eure(proxy = "bevy_math::UVec2")]
pub struct UVec2Proxy {
    pub x: u32,
    pub y: u32,
}

#[derive(FromEure, IntoEure)]
#[eure(opaque = "bevy_math::Quat")]
pub struct QuatProxy(pub [f32; 4]);

impl From<Quat> for QuatProxy {
    fn from(value: Quat) -> Self {
        Self(value.to_array())
    }
}

impl From<QuatProxy> for Quat {
    fn from(value: QuatProxy) -> Self {
        Quat::from_array(value.0)
    }
}
