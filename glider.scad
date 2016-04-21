$fn=30;

module cut_cone(radius1, radius2, height)
{
    hull()
    {
        cylinder(r=radius1, h=0.1);
        translate([0,0.3,1] * height)
        scale([1, 1.2, 1])
        cylinder(r=radius2, h=0.1);
    };
}

cut_cone(20, 25, 25);
