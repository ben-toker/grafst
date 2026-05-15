#[derive(Clone, Debug, Default)]
pub struct NodeStyle {
    pub fill: Option<String>,
    pub shape: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct EdgeStyle {
    pub color: Option<String>,
    pub width: Option<f64>,
}
