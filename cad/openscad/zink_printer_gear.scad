include <threads.scad>;

$fn = 100;

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

module outside() {
    linear_extrude(5) difference() {
        circle(d=5);
        circle(d=1.9);
    }

    translate([0, 0, 5]) linear_extrude(12.5) difference() {
        circle(d=2.9);
        circle(d=1.9);
    }

    translate([0, 0, 5]) metric_thread (diameter=4.3, pitch=1, length=10);

    translate([0, 0, 2]) linear_extrude(0.5) intersection() {
        circle(4);
        square(4);
    }
}

difference() {
    outside();
    translate([0, 0, -1]) cylinder(h=20, d=1.9);
}