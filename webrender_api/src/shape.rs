use {ColorU, IdNamespace};
use {LayoutPoint, LayoutRect, LayoutVector2D};

#[repr(C)]
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PathKey(pub IdNamespace, pub u32);

#[repr(C)]
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub struct Shape {
    kind: ShapeKind,
    fill: ColorU,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
pub enum ShapeKind {
    Circle(LayoutPoint, f32),
    Ellipse(LayoutPoint, LayoutVector2D),
    Line(LayoutPoint, LayoutPoint),
    Rect(LayoutRect, LayoutVector2D),
    Path(PathKey),
    // Polyline(PathKey),
    // Polygon(PathKey),
}
