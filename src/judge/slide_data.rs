use crate::insn::TouchSensor;

pub(super) struct HitAreaData {
    pub hit_points: &'static [TouchSensor],
    pub push_distance: f64,
    pub release_distance: f64,
}

impl HitAreaData {
    pub const fn new(
        hit_points: &'static [TouchSensor],
        push_distance: f64,
        release_distance: f64,
    ) -> Self {
        Self {
            hit_points,
            push_distance,
            release_distance,
        }
    }
}

#[rustfmt::skip]
pub static STRAIGHT_DATA: [&[HitAreaData]; 8] = [
    &[],
    &[],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 130.1907196044922, 129.1265869140625),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(1)), TouchSensor::new_unchecked('B', Some(1))], 159.60830688476562, 129.1265869140625),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(2))], 130.1907196044922, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 159.002197265625, 130.99996948242188),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(1))], 139.280029296875, 28.289979934692383),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(2))], 139.280029296875, 130.99996948242188),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(3))], 159.002197265625, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 156.42124938964844, 43.27423858642578),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(0))], 128.9917755126953, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('C', None)], 218.6302947998047, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(4))], 128.9917755126953, 43.27423858642578),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(4))], 156.42124938964844, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 159.002197265625, 130.99996948242188),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(7))], 139.280029296875, 28.289979934692383),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(6))], 139.280029296875, 130.99996948242188),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(5))], 159.002197265625, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 130.1907196044922, 129.1265869140625),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(7)), TouchSensor::new_unchecked('B', Some(7))], 159.60830688476562, 129.1265869140625),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(6))], 130.1907196044922, 0.0),
    ],
    &[],
];

#[rustfmt::skip]
pub static CIRCLE_L_DATA: [&[HitAreaData]; 8] = [
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 113.34442138671875, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(7))], 226.6888427734375, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(6))], 226.6888427734375, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(5))], 226.6888427734375, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(4))], 226.6888427734375, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(3))], 226.6888427734375, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(2))], 226.6888427734375, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(1))], 226.6888427734375, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 113.34442138671875, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 113.34442138671875, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(7))], 226.6888427734375, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(6))], 226.6888427734375, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(5))], 226.6888427734375, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(4))], 226.6888427734375, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(3))], 226.6888427734375, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(2))], 226.6888427734375, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(1))], 113.34442138671875, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 113.34442138671875, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(7))], 226.6888427734375, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(6))], 226.6888427734375, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(5))], 226.6888427734375, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(4))], 226.6888427734375, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(3))], 226.6888427734375, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(2))], 113.34442138671875, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 113.34442138671875, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(7))], 226.6888427734375, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(6))], 226.6888427734375, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(5))], 226.6888427734375, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(4))], 226.6888427734375, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(3))], 113.34442138671875, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 113.34442138671875, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(7))], 226.6888427734375, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(6))], 226.6888427734375, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(5))], 226.6888427734375, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(4))], 113.34442138671875, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 113.34442138671875, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(7))], 226.6888427734375, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(6))], 226.6888427734375, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(5))], 113.34442138671875, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 113.34442138671875, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(7))], 226.6888427734375, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(6))], 113.34442138671875, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 113.34442138671875, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(7))], 113.34442138671875, 0.0),
    ],
];

#[rustfmt::skip]
pub static CURVE_L_DATA: [&[HitAreaData]; 8] = [
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 159.3672332763672, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(7))], 145.26956176757812, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(6))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(5))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(4))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(3))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(2))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(1))], 145.26956176757812, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 159.3672332763672, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 159.3672332763672, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(7))], 145.26956176757812, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(6))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(5))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(4))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(3))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(2))], 145.26956176757812, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(1))], 159.3672332763672, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 159.3672332763672, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(7))], 145.26956176757812, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(6))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(5))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(4))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(3))], 145.26956176757812, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(2))], 159.3672332763672, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 159.3672332763672, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(7))], 145.26956176757812, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(6))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(5))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(4))], 145.26956176757812, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(3))], 159.3672332763672, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 159.3672332763672, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(7))], 145.26956176757812, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(6))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(5))], 145.26956176757812, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(4))], 159.3672332763672, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 159.3672332763672, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(7))], 145.26956176757812, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(6))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(5))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(4))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(3))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(2))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(1))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(0))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(7))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(6))], 145.26956176757812, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(5))], 159.3672332763672, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 159.3672332763672, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(7))], 145.26956176757812, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(6))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(5))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(4))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(3))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(2))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(1))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(0))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(7))], 145.26956176757812, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(6))], 159.3672332763672, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 159.3672332763672, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(7))], 145.26956176757812, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(6))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(5))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(4))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(3))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(2))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(1))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(0))], 145.26956176757812, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(7))], 159.3672332763672, 0.0),
    ],
];

#[rustfmt::skip]
pub static THUNDER_L_DATA: [&[HitAreaData]; 8] = [
    &[],
    &[],
    &[],
    &[],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 159.3672332763672, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(7))], 145.26956176757812, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(6))], 151.8427276611328, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('C', None)], 218.6302947998047, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(2))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(3))], 145.26956176757812, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(4))], 159.3672332763672, 0.0),
    ],
    &[],
    &[],
    &[],
];

#[rustfmt::skip]
pub static CORNER_DATA: [&[HitAreaData]; 8] = [
    &[],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 156.42124938964844, 43.27423858642578),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(0))], 128.9917755126953, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('C', None)], 218.6302947998047, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(1))], 128.9917755126953, 43.27423858642578),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(1))], 156.42124938964844, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 156.42124938964844, 43.27423858642578),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(0))], 128.9917755126953, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('C', None)], 218.6302947998047, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(2))], 128.9917755126953, 43.27423858642578),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(2))], 156.42124938964844, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 156.42124938964844, 43.27423858642578),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(0))], 128.9917755126953, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('C', None)], 218.6302947998047, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(3))], 128.9917755126953, 43.27423858642578),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(3))], 156.42124938964844, 0.0),
    ],
    &[],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 156.42124938964844, 43.27423858642578),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(0))], 128.9917755126953, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('C', None)], 218.6302947998047, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(5))], 128.9917755126953, 43.27423858642578),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(5))], 156.42124938964844, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 156.42124938964844, 43.27423858642578),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(0))], 128.9917755126953, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('C', None)], 218.6302947998047, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(6))], 128.9917755126953, 43.27423858642578),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(6))], 156.42124938964844, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 156.42124938964844, 43.27423858642578),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(0))], 128.9917755126953, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('C', None)], 218.6302947998047, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(7))], 128.9917755126953, 43.27423858642578),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(7))], 156.42124938964844, 0.0),
    ],
];

#[rustfmt::skip]
pub static BEND_L_DATA: [&[HitAreaData]; 8] = [
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 156.42124938964844, 43.27423858642578),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(0))], 128.9917755126953, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('C', None)], 218.6302947998047, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(3))], 133.84408569335938, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(2))], 272.711669921875, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(1))], 226.6888427734375, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 113.34442138671875, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 156.42124938964844, 43.27423858642578),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(0))], 128.9917755126953, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('C', None)], 218.6302947998047, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(3))], 133.84408569335938, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(2))], 272.711669921875, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(1))], 113.34442138671875, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 156.42124938964844, 43.27423858642578),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(0))], 128.9917755126953, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('C', None)], 218.6302947998047, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(3))], 133.84408569335938, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(2))], 159.3672332763672, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 156.42124938964844, 43.27423858642578),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(0))], 128.9917755126953, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('C', None)], 218.6302947998047, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(3))], 133.84408569335938, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(2))], 272.711669921875, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(1))], 159.3672332763672, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(0))], 133.84408569335938, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('C', None)], 218.6302947998047, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(3))], 128.9917755126953, 43.27423858642578),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(3))], 156.42124938964844, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 156.42124938964844, 43.27423858642578),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(0))], 128.9917755126953, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('C', None)], 218.6302947998047, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(3))], 133.84408569335938, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(2))], 272.711669921875, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(1))], 159.3672332763672, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(0))], 133.84408569335938, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('C', None)], 218.6302947998047, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(4))], 128.9917755126953, 43.27423858642578),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(4))], 156.42124938964844, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 156.42124938964844, 43.27423858642578),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(0))], 128.9917755126953, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('C', None)], 218.6302947998047, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(3))], 133.84408569335938, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(2))], 272.711669921875, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(1))], 159.3672332763672, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(0))], 145.26956176757812, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(7)), TouchSensor::new_unchecked('C', None)], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(6)), TouchSensor::new_unchecked('B', Some(5))], 145.26956176757812, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(5))], 159.3672332763672, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 156.42124938964844, 43.27423858642578),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(0))], 128.9917755126953, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('C', None)], 218.6302947998047, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(3))], 133.84408569335938, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(2))], 272.711669921875, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(1))], 159.3672332763672, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(0))], 145.26956176757812, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(7))], 151.8427276611328, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(6))], 159.3672332763672, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 156.42124938964844, 43.27423858642578),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(0))], 128.9917755126953, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('C', None)], 218.6302947998047, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(3))], 133.84408569335938, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(2))], 272.711669921875, 16.928831100463867),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(1))], 159.3672332763672, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0)), TouchSensor::new_unchecked('B', Some(0))], 159.3672332763672, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(7))], 113.34442138671875, 0.0),
    ],
];

#[rustfmt::skip]
pub static SKIP_L_DATA: [&[HitAreaData]; 8] = [
    &[],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 130.1907196044922, 129.1265869140625),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(7)), TouchSensor::new_unchecked('B', Some(7))], 159.60830688476562, 129.1265869140625),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(6))], 289.5579528808594, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(7))], 145.26956176757812, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(0))], 145.26956176757812, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(1))], 159.3672332763672, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 130.1907196044922, 129.1265869140625),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(7)), TouchSensor::new_unchecked('B', Some(7))], 159.60830688476562, 129.1265869140625),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(6))], 286.6119689941406, 43.27423858642578),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(6))], 128.9917755126953, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('C', None)], 218.6302947998047, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(2))], 128.9917755126953, 43.27423858642578),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(2))], 156.42124938964844, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 130.1907196044922, 129.1265869140625),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(7)), TouchSensor::new_unchecked('B', Some(7))], 159.60830688476562, 129.1265869140625),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(6))], 289.5579528808594, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(5))], 145.26956176757812, 16.465057373046875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(4))], 145.26956176757812, 131.0873260498047),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(3))], 159.3672332763672, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 130.1907196044922, 129.1265869140625),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(7)), TouchSensor::new_unchecked('B', Some(7))], 159.60830688476562, 129.1265869140625),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(6))], 260.3814392089844, 129.1265869140625),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(5)), TouchSensor::new_unchecked('B', Some(5))], 159.60830688476562, 129.1265869140625),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(4))], 130.1907196044922, 0.0),
    ],
    &[],
    &[],
    &[],
];

#[rustfmt::skip]
pub static FAN_DATA: [&[HitAreaData]; 8] = [
    &[],
    &[],
    &[],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 159.002197265625, 130.99996948242188),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(1))], 139.280029296875, 28.289979934692383),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(2))], 139.280029296875, 130.99996948242188),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(3)), TouchSensor::new_unchecked('D', Some(4))], 159.002197265625, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 156.42124938964844, 43.27423858642578),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(0))], 128.9917755126953, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('C', None)], 218.6302947998047, 42.19921875),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(4))], 128.9917755126953, 43.27423858642578),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(4))], 156.42124938964844, 0.0),
    ],
    &[
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(0))], 159.002197265625, 130.99996948242188),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(7))], 139.280029296875, 28.289979934692383),
        HitAreaData::new(&[TouchSensor::new_unchecked('B', Some(6))], 139.280029296875, 130.99996948242188),
        HitAreaData::new(&[TouchSensor::new_unchecked('A', Some(5)), TouchSensor::new_unchecked('D', Some(5))], 159.002197265625, 0.0),
    ],
    &[],
    &[],
];
