use wavefront_obj::mtl::{
    MaterialSet,
    Material,
    IlluminationModel,
    Color,
    Parser,
};
use std::slice;


struct Test {
    data: String,
    expected: MaterialSet,
}

struct TestSet { 
    data: Vec<Test>,
}

impl TestSet {
    fn iter(&self) -> TestSetIter {
        TestSetIter {
            inner: self.data.iter(),
        }
    }
}

struct TestSetIter<'a> {
    inner: slice::Iter<'a, Test>,
}

impl<'a> Iterator for TestSetIter<'a> {
    type Item = &'a Test;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

#[rustfmt::skip]
fn test_cases() -> TestSet {
    TestSet {
        data: vec![
            Test {
                data: String::from(r""),
                expected: MaterialSet {
                    materials: vec![]
                }
            },
            Test {
                data: String::from(r"
                    newmtl frost_wind
                    Ka 0.2 0.2 0.2
                    Kd 0.6 0.6 0.6
                    Ks 0.1 0.1 0.1
                    d 1
                    Ns 200
                    illum 2
                    map_d window.png
                "),
                expected: MaterialSet {
                    materials: vec![
                        Material {
                            name: String::from("frost_wind"),
                            color_ambient: Color { r: 0.2, g: 0.2, b: 0.2 },
                            color_diffuse: Color { r: 0.6, g: 0.6, b: 0.6 },
                            color_specular: Color { r: 0.1, g: 0.1, b: 0.1 },
                            color_emissive: Color { r: 0.0, g: 0.0, b: 0.0 },
                            specular_exponent: 200_f64,
                            dissolve: 1_f64,
                            optical_density: None,
                            illumination_model: IlluminationModel::AmbientDiffuseSpecular,
                            map_ambient: None,
                            map_diffuse: None,
                            map_specular: None,
                            map_emissive: None,
                            map_specular_exponent: None,
                            map_bump: None,
                            map_displacement: None,
                            map_dissolve: Some(String::from("window.png")),
                            map_decal: None,
                        }
                    ]
                }
            },
            Test {
                data: String::from(r"
                    newmtl cube
                    Ns 10.0000
                    Ni 1.5000
                    d 1.0000
                    illum 2
                    Ka 0.0000 0.0000 0.0000
                    Kd 0.5880 0.5880 0.5880
                    Ks 0.0000 0.0000 0.0000
                    Ke 0.3000 0.3000 0.3000
                    map_Ka cube.png
                    map_Kd cube.png
                "),
                expected: MaterialSet {
                    materials: vec![
                        Material {
                            name: String::from("cube"),
                            color_ambient: Color { r: 0.0, g: 0.0, b: 0.0 },
                            color_diffuse: Color { r: 0.5880, g: 0.5880, b: 0.5880 },
                            color_specular: Color { r: 0.0, g: 0.0, b: 0.0 },
                            color_emissive: Color { r: 0.3, g: 0.3, b: 0.3 },
                            specular_exponent: 10.0,
                            dissolve: 1.0,
                            optical_density: Some(1.5),
                            illumination_model: IlluminationModel::AmbientDiffuseSpecular,
                            map_ambient: Some(String::from("cube.png")),
                            map_diffuse: Some(String::from("cube.png")),
                            map_specular: None,
                            map_emissive: None,
                            map_specular_exponent: None,
                            map_bump: None,
                            map_displacement: None,
                            map_dissolve: None,
                            map_decal: None,
                        }
                    ]
                }
            },
            Test {
                data: String::from(r"
                    newmtl fresnel_blu
                    Ka 0.0000 0.0000 0.0000
                    Kd 0.0000 0.0000 0.0000
                    Ks 0.6180 0.8760 0.1430
                    Ns 200
                    illum 1
                    map_d fresnel_blu_dissolve.png

                    newmtl real_windsh
                    Ka 0.0000 0.0000 0.0000
                    Kd 0.0000 0.0000 0.0000
                    Ks 0.0000 0.0000 0.0000
                    Ns 200
                    Ni 1.5000
                    illum 2
                    decal decal.jpg

                    newmtl fresnel_win
                    Ka 0.0000 0.0000 1.0000
                    Kd 0.0000 0.0000 1.0000
                    Ks 0.6180 0.8760 0.1430
                    Ns 200
                    Ni 1.2000
                    illum 0

                    newmtl tin
                    Ka 0.5000 0.5000 0.5000
                    Kd 0.3000 0.2540 0.3128
                    Ks 0.3245 0.2976 0.1234
                    Ns 200
                    illum 2
                    map_Bump tin_bump.png
                    map_Ka tin_Ka.png
                    map_Kd tin_Kd.png
                    map_Ks tin_Ks.png

                    newmtl material
                    Ni 3.4924
                    illum 2
                    d 0.9
                    map_Ke material_Ke.png
                    map_Ka material_Ka.png
                    map_Kd material_Kd.png
                    map_Ks material_Ks.png
                    bump material_bump.png
                    map_Ns material_Ns.png
                    disp material_displacement.png

                "),
                expected: MaterialSet {
                    materials: vec![
                        Material {
                            name: String::from("fresnel_blu"),
                            color_ambient: Color { r: 0.0, g: 0.0, b: 0.0 },
                            color_diffuse: Color { r: 0.0, g: 0.0, b: 0.0 },
                            color_specular: Color { r: 0.6180, g: 0.8760, b: 0.1430 },
                            color_emissive: Color { r: 0.0, g: 0.0, b: 0.0 },
                            specular_exponent: 200.0,
                            dissolve: 1.0,
                            optical_density: None,
                            illumination_model: IlluminationModel::AmbientDiffuse,
                            map_ambient: None,
                            map_diffuse: None,
                            map_specular: None,
                            map_emissive: None,
                            map_specular_exponent: None,
                            map_bump: None,
                            map_displacement: None,
                            map_dissolve: Some(String::from("fresnel_blu_dissolve.png")),
                            map_decal: None,
                        },
                        Material {
                            name: String::from("real_windsh"),
                            color_ambient: Color { r: 0.0, g: 0.0, b: 0.0 },
                            color_diffuse: Color { r: 0.0, g: 0.0, b: 0.0 },
                            color_specular: Color { r: 0.0, g: 0.0, b: 0.0 },
                            color_emissive: Color { r: 0.0, g: 0.0, b: 0.0 },
                            specular_exponent: 200.0,
                            dissolve: 1.0,
                            optical_density: Some(1.5),
                            illumination_model: IlluminationModel::AmbientDiffuseSpecular,
                            map_ambient: None,
                            map_diffuse: None,
                            map_specular: None,
                            map_emissive: None,
                            map_specular_exponent: None,
                            map_bump: None,
                            map_displacement: None,
                            map_dissolve: None,
                            map_decal: Some(String::from("decal.jpg")),
                        },
                        Material {
                            name: String::from("fresnel_win"),
                            color_ambient: Color { r: 0.0, g: 0.0, b: 1.0 },
                            color_diffuse: Color { r: 0.0, g: 0.0, b: 1.0 },
                            color_specular: Color { r: 0.6180, g: 0.8760, b: 0.1430 },
                            color_emissive: Color { r: 0.0, g: 0.0, b: 0.0 },
                            specular_exponent: 200.0,
                            dissolve: 1.0,
                            optical_density: Some(1.2000),
                            illumination_model: IlluminationModel::Ambient,
                            map_ambient: None,
                            map_diffuse: None,
                            map_specular: None,
                            map_emissive: None,
                            map_specular_exponent: None,
                            map_bump: None,
                            map_displacement: None,
                            map_dissolve: None,
                            map_decal: None,
                        },
                        Material {
                            name: String::from("tin"),
                            color_ambient: Color { r: 0.5000, g: 0.5000, b: 0.5000 },
                            color_diffuse: Color { r: 0.3000, g: 0.2540, b: 0.3128 },
                            color_specular: Color { r: 0.3245, g: 0.2976, b: 0.1234 },
                            color_emissive: Color { r: 0.0, g: 0.0, b: 0.0 },
                            specular_exponent: 200.0,
                            dissolve: 1.0,
                            optical_density: None,
                            illumination_model: IlluminationModel::AmbientDiffuseSpecular,
                            map_ambient: Some(String::from("tin_Ka.png")),
                            map_diffuse: Some(String::from("tin_Kd.png")),
                            map_specular: Some(String::from("tin_Ks.png")),
                            map_emissive: None,
                            map_specular_exponent: None,
                            map_bump: Some(String::from("tin_bump.png")),
                            map_displacement: None,
                            map_dissolve: None,
                            map_decal: None,
                        },
                        Material {
                            name: String::from("material"),
                            color_ambient: Color { r: 0.0000, g: 0.0000, b: 0.0000 },
                            color_diffuse: Color { r: 0.0000, g: 0.0000, b: 0.0000 },
                            color_specular: Color { r: 0.0000, g: 0.0000, b: 0.0000 },
                            color_emissive: Color { r: 0.0000, g: 0.0000, b: 0.0000 },
                            specular_exponent: 0.0,
                            dissolve: 0.9,
                            optical_density: Some(3.4924),
                            illumination_model: IlluminationModel::AmbientDiffuseSpecular,
                            map_ambient: Some(String::from("material_Ka.png")),
                            map_diffuse: Some(String::from("material_Kd.png")),
                            map_specular: Some(String::from("material_Ks.png")),
                            map_emissive: Some(String::from("material_Ke.png")),
                            map_specular_exponent: Some(String::from("material_Ns.png")),
                            map_bump: Some(String::from("material_bump.png")),
                            map_displacement: Some(String::from("material_displacement.png")),
                            map_dissolve: None,
                            map_decal: None,
                        },
                    ]
                }
            }
        ]
    }
}

/// The parser should correctly parse a Wavefront MTL file.
#[test]
fn test_parse_material_set() {
    let tests = test_cases();
    
    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result = parser.parse_mtlset().unwrap();

        assert_eq!(result, test.expected);
    }
}

/// The parser should correctly parser each material in a 
/// Wavefront MTL file.
#[test]
fn test_parse_material_set_correctly_parses_each_material() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result = parser.parse_mtlset().unwrap();
        for (result_material, expected_material) 
            in result.materials.iter().zip(test.expected.materials.iter()) {

            assert_eq!(result_material, expected_material);
        } 
    }
}

/// The parser should correctly parse the correct number of
/// materials in a material library.
#[test]
fn test_parse_material_set_lengths_match() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result = parser.parse_mtlset().unwrap();

        assert_eq!(result.materials.len(), test.expected.materials.len());
    }
}

/// The parser should correctly parse the name of each material.
#[test]
fn test_parse_material_set_should_parse_correct_material_names() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result = parser.parse_mtlset().unwrap();
        for (result_material, expected_material) 
            in result.materials.iter().zip(test.expected.materials.iter()) {

            assert_eq!(result_material.name, expected_material.name);
        } 
    }
}

/// The parser should correctly parse the ambient color of each material.
#[test]
fn test_parse_material_set_should_parse_correct_ambient_colors() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result = parser.parse_mtlset().unwrap();
        for (result_material, expected_material) 
            in result.materials.iter().zip(test.expected.materials.iter()) {

            assert_eq!(result_material.color_ambient, expected_material.color_ambient);
        } 
    }
}

/// The parser should correctly parse the diffuse color of each material.
#[test]
fn test_parse_material_set_should_parse_correct_diffuse_colors() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result = parser.parse_mtlset().unwrap();
        for (result_material, expected_material) 
            in result.materials.iter().zip(test.expected.materials.iter()) {

            assert_eq!(result_material.color_diffuse, expected_material.color_diffuse);
        } 
    }
}

/// The parser should correctly parse the specular color of each material.
#[test]
fn test_parse_material_set_should_parse_correct_specular_colors() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result = parser.parse_mtlset().unwrap();
        for (result_material, expected_material) 
            in result.materials.iter().zip(test.expected.materials.iter()) {

            assert_eq!(result_material.color_specular, expected_material.color_specular);
        } 
    }
}

/// The parser should correctly parse the emissive color of each material.
#[test]
fn test_parse_material_set_should_parse_correct_emissive_colors() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result = parser.parse_mtlset().unwrap();
        for (result_material, expected_material) 
            in result.materials.iter().zip(test.expected.materials.iter()) {

            assert_eq!(result_material.color_emissive, expected_material.color_emissive);
        } 
    }
}

/// The parser should correctly parse the specular exponent of each material.
#[test]
fn test_parse_material_set_should_parse_correct_specular_exponents() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result = parser.parse_mtlset().unwrap();
        for (result_material, expected_material) 
            in result.materials.iter().zip(test.expected.materials.iter()) {

            assert_eq!(result_material.specular_exponent, expected_material.specular_exponent);
        } 
    }
}

/// The parser should correctly parse the alpha value of each material.
#[test]
fn test_parse_material_set_should_parse_correct_alpha_values() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result = parser.parse_mtlset().unwrap();
        for (result_material, expected_material) 
            in result.materials.iter().zip(test.expected.materials.iter()) {

            assert_eq!(result_material.dissolve, expected_material.dissolve);
        } 
    }
}

/// The parser should correctly parse the optical density of each material.
#[test]
fn test_parse_material_set_should_parse_correct_optical_densities() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result = parser.parse_mtlset().unwrap();
        for (result_material, expected_material) 
            in result.materials.iter().zip(test.expected.materials.iter()) {

            assert_eq!(result_material.optical_density, expected_material.optical_density);
        } 
    }
}

/// The parser should correctly parse the illumination model of each material.
#[test]
fn test_parse_material_set_should_parse_correct_illumination_models() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result = parser.parse_mtlset().unwrap();
        for (result_material, expected_material) 
            in result.materials.iter().zip(test.expected.materials.iter()) {

            assert_eq!(result_material.illumination_model, expected_material.illumination_model);
        } 
    }
}

/// The parser should correctly parse the ambient map of each material.
#[test]
fn test_parse_material_set_should_parse_correct_ambient_maps() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result = parser.parse_mtlset().unwrap();
        for (result_material, expected_material) 
            in result.materials.iter().zip(test.expected.materials.iter()) {

            assert_eq!(result_material.map_ambient, expected_material.map_ambient);
        } 
    }
}

/// The parser should correctly parse the diffuse map of each material.
#[test]
fn test_parse_material_set_should_parse_correct_diffuse_maps() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result = parser.parse_mtlset().unwrap();
        for (result_material, expected_material) 
            in result.materials.iter().zip(test.expected.materials.iter()) {

            assert_eq!(result_material.map_diffuse, expected_material.map_diffuse);
        } 
    }
}

/// The parser should correctly parse the specular map of each material.
#[test]
fn test_parse_material_set_should_parse_correct_specular_maps() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result = parser.parse_mtlset().unwrap();
        for (result_material, expected_material) 
            in result.materials.iter().zip(test.expected.materials.iter()) {

            assert_eq!(result_material.map_specular, expected_material.map_specular);
        } 
    }
}

/// The parser should correctly parse the emissive map of each material.
#[test]
fn test_parse_material_set_should_parse_correct_emissive_maps() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result = parser.parse_mtlset().unwrap();
        for (result_material, expected_material) 
            in result.materials.iter().zip(test.expected.materials.iter()) {

            assert_eq!(result_material.map_emissive, expected_material.map_emissive);
        } 
    }
}

/// The parser should correctly parse the specular exponent map of each material.
#[test]
fn test_parse_material_set_should_parse_correct_specular_exponent_maps() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result = parser.parse_mtlset().unwrap();
        for (result_material, expected_material) 
            in result.materials.iter().zip(test.expected.materials.iter()) {

            assert_eq!(
                result_material.map_specular_exponent, 
                expected_material.map_specular_exponent
            );
        } 
    }
}

/// The parser should correctly parse the bump map of each material.
#[test]
fn test_parse_material_set_should_parse_correct_bump_maps() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result = parser.parse_mtlset().unwrap();
        for (result_material, expected_material) 
            in result.materials.iter().zip(test.expected.materials.iter()) {

            assert_eq!(result_material.map_bump, expected_material.map_bump);
        } 
    }
}

/// The parser should correctly parse the displacement map of each material.
#[test]
fn test_parse_material_set_should_parse_correct_displacement_maps() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result = parser.parse_mtlset().unwrap();
        for (result_material, expected_material) 
            in result.materials.iter().zip(test.expected.materials.iter()) {

            assert_eq!(result_material.map_displacement, expected_material.map_displacement);
        } 
    }
}

/// The parser should correctly parse the alpha map of each material.
#[test]
fn test_parse_material_set_should_parse_correct_alpha_maps() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result = parser.parse_mtlset().unwrap();
        for (result_material, expected_material) 
            in result.materials.iter().zip(test.expected.materials.iter()) {

            assert_eq!(result_material.map_dissolve, expected_material.map_dissolve);
        } 
    }
}

/// The parser should correctly parse the decal map of each material.
#[test]
fn test_parse_material_set_should_parse_correct_decal_maps() {
    let tests = test_cases();

    for test in tests.iter() {
        let mut parser = Parser::new(&test.data);
        let result = parser.parse_mtlset().unwrap();
        for (result_material, expected_material) 
            in result.materials.iter().zip(test.expected.materials.iter()) {

            assert_eq!(result_material.map_decal, expected_material.map_decal);
        } 
    }
}

