wall_thickness = 1.5;
output_thickness = 1.5;
internal_height = 26;
center_radius = 10;
output_radius = 3;
size = 40;
$fn = 500;

total_length = size;
total_width = size + wall_thickness + output_thickness;
total_height = internal_height + wall_thickness * 3/2;


module round_rect(length, width, radii) {
    points = [
        [-length/2 + radii[0], -width/2 + radii[0]],
        [ length/2 - radii[1], -width/2 + radii[1]],
        [ length/2 - radii[2],  width/2 - radii[2]],
        [-length/2 + radii[3],  width/2 - radii[3]]
       ];
    
    hull() {
        for(i = [0:3]) {
            translate(points[i]) circle(r=radii[i]);
        }
    }
}

module ring(or, ir) {
    difference() {
        circle(or);
        circle(ir);
    }
}

module right_arc(or, ir) {
    intersection() {
        ring(or, ir);
        square(or);
    }
}

module common() {
    difference() {
        translate([0,(wall_thickness+output_thickness)/2])
            round_rect(total_length, total_width, [5, 5, 15, 1]);
        circle(center_radius - wall_thickness);
    }
}
