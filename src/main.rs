//"Import" the module along with the macros
#[macro_use]
extern crate scad_generator;

//Avoid having to write scad_generator:: everywhere
use scad_generator::*;

pub fn main()
{
    //Create an scad object
    let translation = scad!(Translate(vec3(2.0, 2.0, 3.0));
            {
                scad!(Cube(vec3(2.0,1.0,4.0)))
            }
        );


    //Print the result
    println!("{}", translation.get_code());
}
