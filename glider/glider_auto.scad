difference()
{
	difference()
	{
		union()
		{
			translate([0,0,0])
			{
				hull()
				{
					cylinder(h=0.0000001,d=25);
					translate([0,0,20])
					{
						cylinder(h=0.0000001,d=34);
					}
				}
			}
			translate([0,0,20])
			{
				hull()
				{
					cylinder(h=0.0000001,d=34);
					translate([0,0,40])
					{
						cylinder(h=0.0000001,d=40);
					}
				}
			}
			translate([0,0,60])
			{
				hull()
				{
					cylinder(h=0.0000001,d=40);
					translate([0,0,80])
					{
						cylinder(h=0.0000001,d=45);
					}
				}
			}
			translate([0,0,140])
			{
				hull()
				{
					cylinder(h=0.0000001,d=45);
					translate([0,0,80])
					{
						cylinder(h=0.0000001,d=40);
					}
				}
			}
			translate([0,0,220])
			{
				hull()
				{
					cylinder(h=0.0000001,d=40);
					translate([0,0,140])
					{
						cylinder(h=0.0000001,d=10);
					}
				}
			}
		}
		scale([0.95,0.95,0.95])
		{
			union()
			{
				translate([0,0,0])
				{
					hull()
					{
						cylinder(h=0.0000001,d=25);
						translate([0,0,20])
						{
							cylinder(h=0.0000001,d=34);
						}
					}
				}
				translate([0,0,20])
				{
					hull()
					{
						cylinder(h=0.0000001,d=34);
						translate([0,0,40])
						{
							cylinder(h=0.0000001,d=40);
						}
					}
				}
				translate([0,0,60])
				{
					hull()
					{
						cylinder(h=0.0000001,d=40);
						translate([0,0,80])
						{
							cylinder(h=0.0000001,d=45);
						}
					}
				}
				translate([0,0,140])
				{
					hull()
					{
						cylinder(h=0.0000001,d=45);
						translate([0,0,80])
						{
							cylinder(h=0.0000001,d=40);
						}
					}
				}
				translate([0,0,220])
				{
					hull()
					{
						cylinder(h=0.0000001,d=40);
						translate([0,0,140])
						{
							cylinder(h=0.0000001,d=10);
						}
					}
				}
			}
		}
	}
	translate([-40,-40,30])
	{
		cube([80,80,400]);
	}
}