$fn=50;
union()
{
	translate([-12,0,0])
	{
		cube([24,13,22]);
	}
	translate([-16.5,0,16])
	{
		cube([33,13,2.5]);
	}
	translate([0,6.5,11])
	{
		translate([-14.5,0,0])
		{
			cylinder(h=10,d=3);
		}
		translate([14.5,0,0])
		{
			cylinder(h=10,d=3);
		}
	}
}
