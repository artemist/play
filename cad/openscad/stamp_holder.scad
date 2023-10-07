include <stamp_holder_common.scad>


module middle() {
    difference() {
        common();
        ring(size/2-wall_thickness,center_radius);
        translate([-size/2, size/2-output_radius]) difference() {
            square(output_radius);
            translate([output_radius,0]) circle(output_radius);
        }
        translate([-size/2, size/2])
            square([size-15,output_thickness]);
        translate([size/2-15,total_width-size/2-15])
            right_arc(15-wall_thickness, 15-wall_thickness-output_thickness);
        translate([size/2-output_thickness-wall_thickness,0])
            square([output_thickness,total_width-size/2-15]);
    }
}
linear_extrude(height=wall_thickness) {
    common();
    translate([-size / 2, 0])
        round_rect((wall_thickness*3/2+output_thickness)*2,size/2+wall_thickness/2,[1,1,1,1]);
}

translate([0,0,wall_thickness]) difference() {
    linear_extrude(height=internal_height) middle();
    translate([0,0,internal_height*1/2]) {
        rotate([0,0,-30]) rotate_extrude(angle=60) square([center_radius-1,1]);
        rotate([0,0,150]) rotate_extrude(angle=60) square([center_radius-1,1]);

    }
}

translate([0,0,wall_thickness]) linear_extrude(height=internal_height+wall_thickness) hull() {
    translate([-size/2-output_thickness-wall_thickness/2,size/4-wall_thickness/2]) circle(wall_thickness/2);
    translate([-size/2-output_thickness-wall_thickness/2,-size/4+wall_thickness/2]) mirror([1,0,0]) circle(wall_thickness/2, $fn=3);
}


translate([0,0,wall_thickness+internal_height]) linear_extrude(height=wall_thickness/2) {
    intersection() {
        middle();
        translate([0,(wall_thickness+output_thickness)/2]) difference() {
            round_rect(total_length, total_width, [5, 5, 15, 1]);
            round_rect(total_length-wall_thickness, total_width-wall_thickness, [5 - wall_thickness/2, 5-wall_thickness/2, 15-wall_thickness/2, 1-wall_thickness/2]);
            
        }
    }
}