$fn = 500;
ir = 65 / 2;
er = ir + 1.5;

linear_extrude(30) {
    difference() {
        circle(er);
        circle(ir);
    }
    for(i = [0:3:359]) {
        translate([er*sin(i), er*cos(i)]) circle(r=1.5);
    }
}