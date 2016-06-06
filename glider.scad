$fn=30;

module cut_cone(radius1, radius2, height)
{
    hull()
    {
        cylinder(r=radius1, h=0.1);
        translate([0,0,1] * height)
        cylinder(r=radius2, h=0.1);
    };
}

//Array of sections that will make up the body
//Contains  the radius  of the section followed by the length
section_amount = 5;
sections = [
    [25, 60],
    [35, 80],
    [40, 80],
    [35, 100],
    [10, 110],
];


current_offset = 0;
for(i = [0: section_amount - 2])
{
    translate([0,0,$current_offset])
    cut_cone(sections[i][0], sections[i + 1][0], sections[i][1]);

    //let(current_offset=current_offset + sections[i][1]);
    let(current_offset,0);

    echo("New  section");
    echo(current_offset);
    echo(sections[i][1]);
}
cylinder(r=1, h=350);
