#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Illumination {
    Ambient,
    AmbientDiffuse,
    AmbientDiffuseSpecular,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Material {
    pub name: String,
    pub color_ambient: Color,
    pub color_diffuse: Color,
    pub color_specular: Color,
    pub color_emissive: Color,
    pub specular_exponent: f64,
    pub dissolve: f64,
    pub optical_density: Option<f64>,
    pub illumination: Illumination,
    pub map_ambient: Option<String>,
    pub map_diffuse: Option<String>,
    pub map_specular: Option<String>,
    pub map_emissive: Option<String>,
    pub map_bump: Option<String>,
    pub map_displacement: Option<String>,
    pub map_dissolve: Option<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MtlSet {
    pub materials: Vec<Material>,
}

