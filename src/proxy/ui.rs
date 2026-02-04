use eure::{FromEure, IntoEure};

#[derive(FromEure, IntoEure)]
#[eure(proxy = "bevy_ui::Val")]
pub enum ValProxy {
    Auto,
    Px(f32),
    Percent(f32),
    Vw(f32),
    Vh(f32),
    VMin(f32),
    VMax(f32),
}

// UiRect型のリモート定義
#[derive(FromEure, IntoEure)]
#[eure(proxy = "bevy_ui::UiRect")]
pub struct UiRectProxy {
    #[eure(via = "ValProxy")]
    pub left: bevy_ui::Val,
    #[eure(via = "ValProxy")]
    pub right: bevy_ui::Val,
    #[eure(via = "ValProxy")]
    pub top: bevy_ui::Val,
    #[eure(via = "ValProxy")]
    pub bottom: bevy_ui::Val,
}

// FlexDirection型のリモート定義
#[derive(FromEure, IntoEure)]
#[eure(proxy = "bevy_ui::FlexDirection")]
pub enum FlexDirectionProxy {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

// JustifyContent型のリモート定義
#[derive(FromEure, IntoEure)]
#[eure(proxy = "bevy_ui::JustifyContent")]
pub enum JustifyContentProxy {
    Default,
    Start,
    End,
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceEvenly,
    SpaceAround,
    Stretch,
}

// AlignItems型のリモート定義
#[derive(FromEure, IntoEure)]
#[eure(proxy = "bevy_ui::AlignItems")]
pub enum AlignItemsProxy {
    Default,
    Start,
    End,
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}

// PositionType型のリモート定義
#[derive(FromEure, IntoEure)]
#[eure(proxy = "bevy_ui::PositionType")]
pub enum PositionTypeProxy {
    Relative,
    Absolute,
}
