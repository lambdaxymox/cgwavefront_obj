use wavefront_obj::mtl;
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

