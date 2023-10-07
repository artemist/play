$fn = 500;
ir = 65 / 2;
er = ir + 1.5;

module main() {
    difference() {
        circle(er);
        circle(ir);
    }
    for(i = [0:3:359]) {
        translate([er*sin(i), er*cos(i)]) circle(r=1.5);
    }
}

difference() {
    linear_extrude(30) {
        difference() {
            main();
            translate([0, -0.5]) square([er + 1.5, 1], center=false);
        }
    }
    for(i = [0:120:240]) translate([0, 0, 17])  rotate(i, [0, 0, 1]) rotate(90, [0, 1, 0]) cylinder(h=er+1.5, r=1);
}