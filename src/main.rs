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
    let path = Path::new("glider_auto.scad");

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

fn cut_cone(d1: f32, d2: f32, height: f32) -> ScadObject
{
    let cyl_height = 0.0000001;
    scad!(Hull; 
        {
            scad!(Cylinder(cyl_height, Diameter(d1))),

            scad!(Translate(vec3(0.0, 0.0, height));{
                scad!(Cylinder(cyl_height, Diameter(d2)))
            }),
        })
}

fn servo() -> ScadObject
{
    let box_size = vec3(24.0, 13.0, 22.0);
    let screw_diameter = 3.0;
    let screw_mount_size = vec3(33.0, box_size.y, 2.5);
    let screw_height = 10.0;
    let screw_edge_offset = 2.0;

    let mount_offset = vec3(0.0, 0.0, 16.0);


    let mut base_cube = scad!(
        Translate(-vec3(box_size.x / 2.0, 0.0, 0.0));
        {
            scad!(Cube(box_size))
        });

    //Creating the screws for the servo
    let mut screws = {
        let mut result = scad!(Translate(vec3(0.0, box_size.y/2.0, mount_offset.z - screw_height / 2.0)));

        for i in &[-1.0,1.0]
        {
            let x_offset = screw_mount_size.x / 2.0 - screw_edge_offset;

            let mut screw = 
            scad!(
                Translate(vec3(i * x_offset, 0.0, 0.0));
                scad!(Cylinder(screw_height, Diameter(screw_diameter)))
            );

            result.add_child(screw);
        }
        result
    };

    let screw_mount = scad!(
        Translate(
                -vec3(screw_mount_size.x / 2.0, 0.0, 0.0) +
                mount_offset
            );
        {
            scad!(Cube(screw_mount_size)),
        });

    scad!(Union;{base_cube, screw_mount, screws})
}

fn glider_body_shape() -> ScadObject
{
    let segments = [
            (25.0, 34.0, 20.0),
            (34.0, 40.0, 40.0),
            (40.0, 45.0, 80.0),
            (45.0, 40.0, 80.0),
            (40.0, 10.0, 140.0),
        ];

    let mut result = scad!(Union);

    let mut current_height = 0.0;
    for i in 0..segments.len()
    {
        result.add_child(scad!(Translate(vec3(0.0, 0.0, current_height));
            {
                cut_cone(segments[i].0, segments[i].1, segments[i].2),
            }));

        current_height += segments[i].2;

    }

    result
}
fn glider_body() -> ScadObject
{
    let inner_scale = 0.9;

    scad!(Difference;
    {
        glider_body_shape(),
        scad!(Scale(vec3(inner_scale, inner_scale, inner_scale));
        {
            glider_body_shape(),
        }),
    })
}



pub fn main()
{
    //Create an scad object
    let translation = 
        scad!(Difference;
        {
            glider_body(),
            scad!(Translate(vec3(-40.0, -40.0, 30.0));
            {
                scad!(Cube(vec3(80.0, 80.0, 400.0))),
            })
        });


    //write_result(&translation);
    write_result(&servo());

    //Print the result
    //println!("{}", translation.get_code());
}
