include <stamp_holder_common.scad>

module finger() {
    translate([center_radius-wall_thickness*2-0.25,0,0]) square([wall_thickness,internal_height*1/3]);
}

module hook() {
    translate([center_radius-wall_thickness*2-0.25,0,0]) {
        square([wall_thickness,internal_height*1/3+2]);
        translate([wall_thickness,internal_height*1/3])
            polygon([[0,-1],[1,0.5],[0,2]]);
    }
}

linear_extrude(wall_thickness) {
    difference() {
        mirror([1,0,0]) {
            translate([0,(wall_thickness+output_thickness)/2])
                round_rect(total_length, total_width, [5, 5, 15, 1]);
            translate([-size / 2, 0])
                round_rect((wall_thickness+output_thickness)*2,size/2,[1,1,1,1]);
        }
        circle(center_radius-wall_thickness*2-0.25);
    }
}

translate([0,0,wall_thickness]) linear_extrude(wall_thickness/2) {
    difference() {
        mirror([1,0,0]) {
            translate([0,(wall_thickness+output_thickness)/2])
                round_rect(total_length-wall_thickness-.5, total_width-wall_thickness-.5, [5, 5, 15, 1]);
            difference() {
                hull() {
                    translate([-size/2-output_thickness-wall_thickness/2,size/4-wall_thickness/2]) circle(wall_thickness/2);
                    translate([-size/2-output_thickness-wall_thickness/2,-size/4+wall_thickness/2]) circle(wall_thickness/2);
                }
                hull() {
                    translate([-size/2-output_thickness-wall_thickness/2,size/4-wall_thickness/2]) circle(wall_thickness*3/8);
                    translate([-size/2-output_thickness-wall_thickness/2,-size/4+wall_thickness/2]) circle(wall_thickness*3/8);
                }
            }
        }
        circle(center_radius-wall_thickness*2-0.25);
    }
}

translate([0,0,wall_thickness*3/2]) linear_extrude(internal_height / 6) {
    ring(center_radius-wall_thickness-0.25, center_radius-wall_thickness*2-0.25);
}

translate([0,0,wall_thickness*3/2+internal_height/6]) {
    rotate([0,0,-25]) rotate_extrude(angle=50) hook();
    rotate([0,0,35]) rotate_extrude(angle=110) finger();
    rotate([0,0,155]) rotate_extrude(angle=50) hook();
    rotate([0,0,215]) rotate_extrude(angle=110) finger();

}