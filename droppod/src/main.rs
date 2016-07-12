//"Import" the module along with the macros
#![allow(dead_code)]
#![allow(unused_variables)]
#[macro_use]
extern crate scad_generator;
extern crate nalgebra as na;
extern crate scad_util as su;

use std::vec::Vec;

//Avoid having to write scad_generator:: everywhere
use scad_generator::*;

struct DropPod 
{
    pub outer_radius: f32,
    pub screw_height: f32,
}

impl DropPod
{
    pub fn get_mid_section(&self, height: f32) -> ScadObject 
    {
        let inner_radius = self.outer_radius - 5.;

        let outer_cylinder = scad!(Cylinder(height, Radius(self.outer_radius)));
        let inner_cylinder = scad!(Cylinder(height, Radius(inner_radius)));

        let bottom_screw = self.get_screw(true);

        let top_screw = scad!(Translate(vec3(0., 0., height - self.screw_height));{self.get_screw(true)});

        scad!(Difference;{outer_cylinder, inner_cylinder, bottom_screw, top_screw})
    }

    pub fn get_nose_cone(&self, height: f32) -> ScadObject
    {
        let ring_amount = 10;
        let ring_radii = 
        {
            let mut result = Vec::<f32>::new();

            for i in 0..ring_amount
            {
                result.push((i as f32 / ring_amount as f32).sqrt() * self.outer_radius + 0.01);
            }

            result.push(self.outer_radius);

            result
        };

        let mut cone = scad!(Union);

        for i in 0..ring_amount
        {
            let section = 
            scad!(Hull;{
                scad!(Cylinder(0.01 as f32, Radius(ring_radii[i]))),
                scad!(Translate(vec3(0., 0., (height / ring_amount as f32)));
                {
                    scad!(Cylinder(0.01 as f32, Radius(ring_radii[i + 1]))),
                }),
            });

            let section = scad!(Translate(vec3(0., 0., i as f32 * height/ring_amount as f32));{section});

            cone.add_child(section);
        }
        
        scad!(Union;{
            cone,
            scad!(Translate(vec3(0., 0., height));{self.get_screw(false)}),
        })
    }

    fn get_screw(&self, is_outer: bool) -> ScadObject 
    {
        let height = self.screw_height;
        let outside_padding = 4.;
        let thread_width = 1.5;
        let thread_height = 3.;

        let mut inner_radius = self.outer_radius - thread_width - outside_padding + 0.01;

        if is_outer
        {
            inner_radius = inner_radius + 0.4;
        }

        let center_piece = scad!(Cylinder(height, Radius(inner_radius)));
        let threads = su::threads::thread(height, inner_radius, thread_width, thread_height);
        
        scad!(Union;{
            center_piece,
            threads,
        })
    }
}


struct MountPoint 
{
    pub pod_radius: f32,
    pub length: f32,
}

impl MountPoint
{
    pub fn get(&self) -> ScadObject 
    {
        scad!(Union;{
            self.pod_mount(),
            self.mid_section(),
        })
    }

    fn pod_mount(&self) -> ScadObject 
    {
        let shell_radius = 4.;
        let outer_radius = shell_radius + self.pod_radius;

        let width_factor = 0.5;

        let outer_cylinder = scad!(Cylinder(self.length, Radius(outer_radius)));
        let inner_cylinder = scad!(Cylinder(self.length, Radius(self.pod_radius)));
        
        //Cut off most of the exess
        let back_cutter = 
            scad!(Translate(vec3(-outer_radius, -outer_radius * width_factor, 0.));{
                scad!(Cube(vec3(outer_radius * 2., outer_radius * 2., self.length)))
            });

        let cut_version = scad!(Difference;{
            outer_cylinder,
            inner_cylinder,
            back_cutter,
        });

        let translated = scad!(Translate(vec3(0., self.pod_radius, 0.));{cut_version});
        scad!(Mirror(vec3(0., 1., 0.));{translated})
    }

    fn mid_section(&self) -> ScadObject
    {
        let thickness = 5.;
        let height = 40.;

        let main_cube = scad!(Translate(vec3(-thickness / 2., 0., 0.));
        {
            scad!(Cube(vec3(thickness, height, self.length)))
        });

        main_cube
    }

    fn hole_beam(&self) -> ScadObject 
    {
        let hole_radius = 3.5;
        let hole_padding = 2.;
    }
}


pub fn main()
{
    let mut sfile = ScadFile::new();

    sfile.set_detail(50);

    let pod = DropPod
    {
        outer_radius: 20.,
        screw_height: 8.,
    };

    let mount = MountPoint
    {
        pod_radius: 20.,
        length: 40.
    };

    //sfile.add_object(pod.get_nose_cone(40.));
    //sfile.add_object(pod.get_mid_section(80.));
    sfile.add_object(mount.get());

    sfile.write_to_file(String::from("cargo_auto.scad"));
}
