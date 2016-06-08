//"Import" the module along with the macros
#[macro_use]
extern crate scad_generator;
extern crate nalgebra as na;

//Avoid having to write scad_generator:: everywhere
use scad_generator::*;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;


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

fn generic_motor_holes(small_diameter: f32, big_diameter: f32, screw_diameter: f32) -> ScadObject 
{
    let center_hole_diameter = 9.0;

    let height = 30.0;
    let mut result = scad!(Translate(vec3(0.0,0.0,-height/2.0)));

    //Add the center hole
    result.add_child(scad!(Cylinder(height, Diameter(center_hole_diameter))));

    //Add the screwholes
    for i in &[-1.0,1.0]
    {
        result.add_child(
            scad!(Translate(vec3(i * small_diameter / 2.0, 0.0, 0.0));
            {
                scad!(Cylinder(height, Diameter(screw_diameter)))
            }),
        );
        result.add_child(
            scad!(Translate(vec3(0.0, i * big_diameter / 2.0, 0.0));
            {
                scad!(Cylinder(height, Diameter(screw_diameter)))
            }),
        );
    }

    return result;
}
fn get_motor_holes() -> ScadObject 
{
    generic_motor_holes(16.0, 19.0, 3.5)
}

fn motor_pod_shape(outside_size: na::Vector3<f32>) -> ScadObject
{
    let back_chamfer_length = 0.5;
    let back_angle = {
        let back_len = outside_size.x * back_chamfer_length;

        let back_height_offset = 5.0;

        ((outside_size.z - back_height_offset) / back_len).atan()
    };

    //Creating block that will chamfer the back edge
    let chamfer = scad!(Rotate(-back_angle / std::f32::consts::PI * 180.0, vec3(0.0, 1.0, 0.0));
    {
        scad!(Translate(vec3(0.0, 0.0, -outside_size.z));
        {
            scad!(Cube(outside_size))
        })
    });


    scad!(Difference;
    {
        scad!(Cube(outside_size)),
        scad!(Translate(vec3(outside_size.x * back_chamfer_length, 0.0, 0.0));
        chamfer)
    })
}

fn motor_pod() -> ScadObject 
{
    let outside_size = vec3(140.0, 35.0, 30.0);
    //The height of the tabs that will go into the foam
    let wall_thickness = 2.0;
    let front_wall_multiplyer = 3.0;

    let skewer_z_offset = 6.0;
    let skewer_locations = [50.0, 80.0, 100.0];
    let skewer_diameter = 3.0;

    //Generating the actual pod
    let outside_shape = motor_pod_shape(outside_size);
    let inside_shape = scad!(Translate(vec3(wall_thickness * front_wall_multiplyer, wall_thickness, wall_thickness));
    {
        motor_pod_shape(outside_size - vec3(wall_thickness * (front_wall_multiplyer+2.0), wall_thickness * 2.0, 0.0))
    });

    let motor_holes = scad!(Translate(vec3(0.0, outside_size.y / 2.0, outside_size.z / 2.0));
    {
        scad!(Rotate(90.0, vec3(0.0, 1.0, 0.0));
        {
            get_motor_holes(),
        })
    });

    let skewer_holes = {
        let mut result = scad!(Union);

        for location in &skewer_locations
        {
            result.add_child(scad!(Translate(vec3(location.clone(), 0.0, outside_size.z - skewer_z_offset));
            {
                scad!(Rotate(-90.0, vec3(1.0, 0.0, 0.0));
                {
                    scad!(Cylinder(outside_size.y, Diameter(skewer_diameter)))
                })
            }));
        }
        result
    };

    scad!(Difference;
    {
        outside_shape,
        inside_shape,
        motor_holes,
        skewer_holes,
    })
}

pub fn main()
{
    //Create an scad object
    let translation = right_angle_bracket();

    let mut sfile = ScadFile::new();

    sfile.set_detail(50);
    //sfile.add_object(translation);
    sfile.add_object(motor_pod());
    sfile.write_to_file(String::from("cargo_auto.scad"));
}
