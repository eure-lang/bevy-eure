use super::math::*;
use bevy_math::{Quat, Vec3};
use eure::{FromEure, IntoEure};

#[derive(FromEure, IntoEure)]
#[eure(proxy = "bevy_transform::components::Transform")]
pub struct TransformProxy {
    #[eure(via = "Vec3Proxy")]
    pub translation: Vec3,
    #[eure(via = "QuatProxy")]
    pub rotation: Quat,
    #[eure(via = "Vec3Proxy")]
    pub scale: Vec3,
}
