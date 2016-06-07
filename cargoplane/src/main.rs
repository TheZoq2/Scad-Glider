//"Import" the module along with the macros
#[macro_use]
extern crate scad_generator;

//Avoid having to write scad_generator:: everywhere
use scad_generator::*;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;


fn write_result(object: &ScadObject)
{
    //Writing the result to file
    let path = Path::new("cargo_auto.scad");

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(_) => panic!("couldn't write file"),
        Ok(file) => file,
    };

    match file.write(object.get_code().as_bytes()) {
        Err(_) => panic!("Failed to write output file"),
        Ok(_) => {}
    };
}

fn triangle(height: f32, thickness: f32) -> ScadObject
{
    let bounds = scad!(Cube(vec3(height, thickness, height)));
    
    let cube_side_len = height * (2.0 as f32).sqrt();

    let cube = scad!(Rotate(45.0, vec3(0.0,1.0,0.0));
        {
            scad!(
            Translate(vec3(-cube_side_len / 2.0, 0.0, -cube_side_len / 2.0));
            {
                scad!(Cube(vec3(cube_side_len, thickness, cube_side_len)))
            }),
        });


    scad!(Translate(vec3(0.0, -thickness/2.0, 0.0));
    {
        scad!(Intersection;
        {
            bounds,
            cube
        }),
    })
}

fn right_angle_bracket() -> ScadObject
{
    let height = 1.0;
    let thickness = 0.1;
    let width = 0.5;

    let triangle_height = 0.7;
    let triangle_thickness = 0.1;

    scad!(Union;
    {
        scad!(Translate(vec3(0.0, -width/2.0, 0.0));
        {
            scad!(Cube(vec3(height, width, thickness))),
            scad!(Cube(vec3(thickness, width, height)))
        }),
        triangle(triangle_height, triangle_thickness)
    })
}

pub fn main()
{
    //Create an scad object
    let translation = right_angle_bracket();


    write_result(&translation);

    //Print the result
    println!("{}", translation.get_code());
}
