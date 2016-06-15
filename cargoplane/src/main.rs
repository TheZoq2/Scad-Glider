//"Import" the module along with the macros
#![allow(dead_code)]
#![allow(unused_variables)]
#[macro_use]
extern crate scad_generator;
extern crate nalgebra as na;
extern crate scad_util as su;

//Avoid having to write scad_generator:: everywhere
use scad_generator::*;

struct MotorPod
{
    outside_size: na::Vector3<f32>,
    screw_z_offset: f32,
    screw_locations: [f32;2],
    screw_diameter: f32,
    wall_thickness: f32,
}

impl MotorPod 
{
    pub fn new() -> MotorPod 
    {
        MotorPod {
            outside_size: vec3(140.0, 35.0, 30.0),
            screw_z_offset: 6.0,
            screw_locations: [50.0, 80.0],
            screw_diameter: 3.6,
            wall_thickness: 2.0,
        }
    }

    fn get_shape(&self, shape_size: na::Vector3<f32>) -> ScadObject
    {
        let back_chamfer_length = 0.5;
        let back_angle = {
            let back_len = shape_size.x * back_chamfer_length;

            let back_height_offset = 5.0;

            ((shape_size.z - back_height_offset) / back_len).atan()
        };

        //Creating block that will chamfer the back edge
        let chamfer = scad!(Rotate(-back_angle / std::f32::consts::PI * 180.0, vec3(0.0, 1.0, 0.0));
        {
            scad!(Translate(vec3(0.0, 0.0, -shape_size.z));
            {
                scad!(Cube(shape_size))
            })
        });


        scad!(Difference;
        {
            scad!(Cube(shape_size)),
            scad!(Translate(vec3(shape_size.x * back_chamfer_length, 0.0, 0.0));
            chamfer)
        })
    }

    fn get_screwholes(&self) -> ScadObject
    {
        let mut result = scad!(Union);

        for location in &self.screw_locations
        {
            result.add_child(scad!(Translate(vec3(location.clone(), 0.0, 0.0));
            {
                scad!(Rotate(-90.0, vec3(1.0, 0.0, 0.0));
                {
                    scad!(Cylinder(self.outside_size.y, Diameter(self.screw_diameter)))
                })
            }));
        }
        result
    }

    pub fn main_pod(&self) -> ScadObject 
    {
        //The height of the tabs that will go into the foam
        let front_wall_multiplyer = 3.0;

        //Generating the actual pod
        let outside_shape = self.get_shape(self.outside_size);
        let inside_shape = scad!(Translate(vec3(self.wall_thickness * front_wall_multiplyer, self.wall_thickness, self.wall_thickness));
        {
            self.get_shape(self.outside_size - vec3(self.wall_thickness * (front_wall_multiplyer+2.0), self.wall_thickness * 2.0, 0.0))
        });

        let motor_holes = scad!(Translate(vec3(0.0, self.outside_size.y / 2.0, self.outside_size.z / 2.0));
        {
            scad!(Rotate(90.0, vec3(0.0, 1.0, 0.0));
            {
                get_motor_holes(),
            })
        });


        scad!(Difference;
        {
            outside_shape,
            inside_shape,
            motor_holes,
            scad!(Translate(vec3(0.0, 0.0, self.outside_size.z - self.screw_z_offset));
            {
                self.get_screwholes(),
            })
        })
    }

    pub fn pod_mount(&self) -> ScadObject
    {
        let top_thickness = 5.0;
        let screw_padding = self.screw_diameter;
        let side_padding = 0.5;

        //TODO: Add hole for carbon spar

        let top_cube = scad!(Cube(vec3(self.outside_size.x, self.outside_size.y, top_thickness)));

        //Calculating the length of the mounting block.
        let mount_length = self.screw_locations.last().unwrap() - self.screw_locations.first().unwrap() + screw_padding * 2.0;
        let mount_height = self.screw_z_offset + screw_padding;
        let mount_width = self.outside_size.y - self.wall_thickness * 2.0 - side_padding * 2.0;
        
        //Create the actual cube with the mounts
        let mount_cube_x_offset = self.screw_locations.first().unwrap() - screw_padding;
        let mount_cube = scad!(Translate(vec3(mount_cube_x_offset, self.wall_thickness + side_padding, top_thickness));
        {
            scad!(Cube(vec3(mount_length, mount_width, mount_height)))
        });

        
        let carbon_spar_height = 3.0;
        let carbon_spar_radius = 10.0;
        let mut carbon_spar_hole = scad!(Translate(vec3(self.outside_size.x/2.0, 0., -carbon_spar_radius + carbon_spar_height));
        {
            scad!(Rotate(-90., vec3(1., 0., 0.));
            {
                scad!(Cylinder(self.outside_size.y, Radius(carbon_spar_radius)))
            })
        });

        //carbon_spar_hole.is_important();

        scad!(Difference;
        {
            scad!(Union;
            {
                top_cube,
                mount_cube,
            }),

            scad!(Translate(vec3(0.0, 0.0, top_thickness + self.screw_z_offset));
            {
                self.get_screwholes()
            }),
            
            carbon_spar_hole
        })
    }
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

fn body_screw_bar() -> ScadObject
{
    let length = 130.0;
    let width = 15.0;
    let height = 20.0;
    let top_thickness = 5.0;
    let side_thickness = 2.0;

    let screw_diameter = 4.5;
    let nut_width = 7.5;
    let nut_height = top_thickness * 0.5;
    let nut_offset = length * 0.66;

    let mut result = scad!(Union);

    let bar = scad!(Union;
    {
        scad!(Cube(vec3(length/2.0, width, top_thickness))),
        scad!(Translate(vec3(length/2.0 - side_thickness, 0.0, 0.0));
        {
            scad!(Cube(vec3(side_thickness, width, height))),
            scad!(Translate(vec3(0.0, width/2.0, 0.0));
            {
                scad!(Rotate(180.0, vec3(0.0,0.0,1.0));
                {
                    triangle(height * 0.75, side_thickness)
                })
            })
        }),
    });
    let screws = 
    scad!(Translate(vec3(nut_offset / 2.0, width/2.0, top_thickness));
    {
        scad!(Mirror(vec3(0.0,0.0,1.0));
        {
            su::nut(nut_width, nut_height),
            scad!(Cylinder(top_thickness, Diameter(screw_diameter)))
        })
    });

    for i in 0..2
    {
        result.add_child(
        scad!(Mirror(vec3(i as f32, 0.0, 0.0));
        {
            scad!(Difference;
            {
                bar.clone(),
                screws.clone()
            })
        })
        );
    }

    result
}

fn battery_tray() -> ScadObject
{
    let width = 45.0;
    let length = 145.0;
    let thickness = 5.0;
    let height = 10.0;
    let strap_width = 20.0;
    let strap_thickness = 4.0;
    let bottom_thickness = 2.0;

    let strap_hole = scad!(Translate(vec3(0.0, width/2.0 - strap_thickness, 0.0));
        scad!(Cube(vec3(strap_width, strap_thickness, thickness)))
    );

    scad!(Difference;
    {
        scad!(Translate(vec3(-(length + thickness)/2.0,-(width + thickness)/2.0,0.0));
              scad!(Cube(vec3(length + thickness, width + thickness, height)))
        ),
        scad!(Translate(vec3(-length/2.0,-width/2.0,bottom_thickness)); scad!(Cube(vec3(length, width, 20.0)))),

        strap_hole.clone(),
        scad!(Mirror(vec3(0.0, 1.0, 0.0)); strap_hole)
    })
}

fn get_motor_holes() -> ScadObject 
{
    su::rc::generic_motor_holes(16.0, 19.0, 3.5)
}



fn wings() -> ScadObject 
{
    let wing_front_offset = 100.0;
    let wingspan = 1600.0;
    let wing_width = 230.0;

    let tailspan = 520.0;
    let tail_width = 220.0;
    let thickness = 6.0;
    
    let total_length = 750.0;

    scad!(Union;
    {
        //Main wing
        scad!(Translate(vec3(wing_front_offset, -wingspan/2.0, 0.0));
        {
            scad!(Cube(vec3(wing_width, wingspan, thickness))),
        }),

        //Tail wingss
        scad!(Translate(vec3(total_length - tail_width, -tailspan / 2.0, 0.0));
        {
            scad!(Cube(vec3(tail_width, tailspan, thickness)))
        })
    })
}

fn nose_attacher() -> ScadObject
{
    let height = 10.0;
    let width = 9.0;
    let screw_diameter = 3.5;
    let length = 10.0;
    let bottom_width = 20.0;
    let bottom_thickness = 2.0;
    let screw_height = height * 0.7;

    let main_cube = scad!(Translate(vec3(0.0, -width/2.0, 0.0)); scad!(Cube(vec3(length, width, height))));
    let bottom_cube = scad!(Translate(vec3(0.0, -bottom_width/2.0, -bottom_thickness));
    {
        scad!(Cube(vec3(length, bottom_width, bottom_thickness)))
    });

    let screwhole = scad!(Translate(vec3(0.0, 0.0, screw_height));
    {
        scad!(Rotate(90.0, vec3(0.0, 1.0, 0.0));
        {
            scad!(Cylinder(length, Diameter(screw_diameter))) 
        })
        
    });

    let chamfer = scad!(Translate(vec3(0.0, width/2.0, height * 0.8));
    {
        scad!(Rotate(45.0, vec3(1.0, 0.0, 0.0));
            scad!(Cube(vec3(length, width, height)))
        )
    });

    scad!(Difference;
    {
        scad!(Union;
            main_cube,
            bottom_cube
        ),
        screwhole,
        chamfer.clone(),
        scad!(Mirror(vec3(0.0, 1.0, 0.0)); chamfer.clone())
    })
}

fn body() -> ScadObject
{
    let inner_width = 130.0;
    let inner_height = 110.0;
    let foam_t = 5.0;

    //The outside size at the end of the body
    let outer_back_width = 40.0;
    let outer_back_height = 30.0;

    //Calculated values
    let outer_height = inner_height + foam_t * 4.0;
    let outer_width = inner_width + foam_t * 4.0;

    scad!(Difference;
    {
        //body_shape(outer_width, outer_height, outer_back_width, outer_back_height),
        scad!(Translate(vec3(0.0, foam_t*2.0, foam_t*2.0));
            body_shape(inner_width, inner_height, outer_back_width - foam_t * 4.0, outer_back_height - foam_t* 4.0)
        )
    })
}

fn body_shape(outer_width:f32, outer_height: f32, back_width: f32, back_height: f32) -> ScadObject
{
    let max_len = 800.0;

    let bottom_chamfer_start = 350.0;
    let side_chamfer_start = 450.0;

    
    //Generating the bottom chamfer
    let bottom_chamfer_length = max_len - bottom_chamfer_start;

    //The angle of the chamfer applied to the bottom of the body
    let bottom_chamfer_angle = {
        let height = outer_height - back_height;

        ((height/bottom_chamfer_length) as f32).asin()
    };

    //The total length of the body
    let body_length = bottom_chamfer_start + bottom_chamfer_length * bottom_chamfer_angle.cos();

    let chamfer_cutoff = scad!(Translate(vec3(bottom_chamfer_start, 0.0, 0.0));
    {
        scad!(Rotate(-bottom_chamfer_angle.to_degrees(), vec3(0.0, 1.0, 0.0));
        {
            scad!(Translate(vec3(0.0, 0.0, -outer_height));
                scad!(Cube(vec3(body_length, outer_width, outer_height)))
            ),
        }),
    });

    //Calculating the chamfer for the sides
    let side_chamfer_length = body_length - side_chamfer_start;
    let side_chamfer_angle = {
        let width = outer_width / 2.0 - back_width / 2.0;
        
        ((width/side_chamfer_length) as f32).asin()
    };

    let side_chamfer_cutoff = scad!(Translate(vec3(side_chamfer_start, outer_width / 2.0, 0.0));
    {
        scad!(Rotate(-side_chamfer_angle.to_degrees(), vec3(0.0, 0.0, 1.0));
            scad!(Cube(vec3(body_length, outer_width, outer_height)))
        ),
    });

    let other_side_cutoff = scad!(Mirror(vec3(0.0, 1.0, 0.0));
        side_chamfer_cutoff.clone()
    );

    scad!(Union;
    {
        scad!(Difference;
        {
            scad!(Cube(vec3(body_length, outer_width, outer_height))),
            chamfer_cutoff,

            scad!(Translate(vec3(0.0, outer_width / 2.0, 0.0));
            {
                side_chamfer_cutoff.clone()
            }),
            scad!(Translate(vec3(0.0,outer_width / 2.0, 0.0));
            {
                other_side_cutoff
            })
        }),
    })
}

pub fn main()
{
    //Create an scad object
    let translation = right_angle_bracket();

    let mut sfile = ScadFile::new();

    sfile.set_detail(50);
    //sfile.add_object(translation);
    //sfile.add_object(MotorPod::new().main_pod());

    //sfile.add_object(scad!(Translate(vec3(0.0, 0.0, 50.0));
    //{
    //    MotorPod::new().pod_mount(),
    //}));
    sfile.add_object(MotorPod::new().pod_mount());
    //sfile.add_object(
    //    scad!(Difference;
    //    {
    //        body(),
    //        scad!(Cube(vec3(750.0, 1000.0, 1000.0))),
    //    }));
    //sfile.add_object(body_screw_bar());
    //sfile.add_object(battery_tray());
    //sfile.add_object(nose_attacher());

    sfile.write_to_file(String::from("cargo_auto.scad"));
}
