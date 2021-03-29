extern crate kiss3d;
extern crate nalgebra as na;
extern crate multiarray;

use kiss3d::light::Light;
use kiss3d::window::Window;
use kiss3d::scene::SceneNode;
use na::{Translation3, Vector3, UnitQuaternion};
use multiarray::Array3D;
use arr_macro::arr;
use rand::Rng;
use std::env;

struct Cube {
    node: SceneNode,
    face: usize,
}

impl Cube {
    fn new(window: &mut Window) -> Cube {
        Cube {
            node: window.add_group(),
            face: 1,
        }
    }
}


fn main() {
    let mut window = Window::new("Kiss3d: cube");

    let mut cn = Array3D::new([3,3,3], 0);
    let mut gs = arr![Cube::new(&mut window); 27];
    let mut x = 0;
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                cn[[i, j, k]] = x;
                x += 1;
            }
        }
    }

    let mut c1 = gs[cn[[0, 2, 2]]].node.add_cube(0.9, 0.9, 0.0);
    let mut c2 = gs[cn[[0, 2, 2]]].node.add_cube(0.0, 0.9, 0.9);
    let mut c3 = gs[cn[[0, 2, 2]]].node.add_cube(0.9, 0.0, 0.9);
    
    c1.set_local_translation(Translation3::new(1.0, 1.0, 1.5));
    c2.set_local_translation(Translation3::new(1.5, 1.0, 1.0));
    c3.set_local_translation(Translation3::new(1.0, 1.5, 1.0));

    c1.set_color(1.0, 0.0, 0.0);
    c2.set_color(0.0, 1.0, 0.0);
    c3.set_color(0.0, 0.0, 1.0);

    let mut c4 = gs[cn[[2, 2, 2]]].node.add_cube(0.9, 0.9, 0.0);
    let mut c5 = gs[cn[[2, 2, 2]]].node.add_cube(0.0, 0.9, 0.9);
    let mut c6 = gs[cn[[2, 2, 2]]].node.add_cube(0.9, 0.0, 0.9);
    
    c4.set_local_translation(Translation3::new(-1.0, 1.0, 1.5));
    c5.set_local_translation(Translation3::new(-1.5, 1.0, 1.0));
    c6.set_local_translation(Translation3::new(-1.0, 1.5, 1.0));

    c4.set_color(1.0, 0.0, 0.0);
    c5.set_color(1.0, 1.0, 1.0);
    c6.set_color(0.0, 0.0, 1.0);

    let mut c7 = gs[cn[[1, 2, 2]]].node.add_cube(0.9, 0.9, 0.0);
    let mut c8 = gs[cn[[1, 2, 2]]].node.add_cube(0.9, 0.0, 0.9);
    
    c7.set_local_translation(Translation3::new(0.0, 1.0, 1.5));
    c8.set_local_translation(Translation3::new(0.0, 1.5, 1.0));

    c7.set_color(1.0, 0.0, 0.0);
    c8.set_color(0.0, 0.0, 1.0);

    let mut c9 = gs[cn[[0, 0, 2]]].node.add_cube(0.9, 0.9, 0.0);
    let mut c10 = gs[cn[[0, 0, 2]]].node.add_cube(0.0, 0.9, 0.9);
    let mut c11 = gs[cn[[0, 0, 2]]].node.add_cube(0.9, 0.0, 0.9);
    
    c9.set_local_translation(Translation3::new(1.0, -1.0, 1.5));
    c10.set_local_translation(Translation3::new(1.5, -1.0, 1.0));
    c11.set_local_translation(Translation3::new(1.0, -1.5, 1.0));

    c9.set_color(1.0, 0.0, 0.0);
    c10.set_color(0.0, 1.0, 0.0);
    c11.set_color(0.4, 0.3, 0.6);

    let mut c12 = gs[cn[[2, 0, 2]]].node.add_cube(0.9, 0.9, 0.0);
    let mut c13 = gs[cn[[2, 0, 2]]].node.add_cube(0.0, 0.9, 0.9);
    let mut c14 = gs[cn[[2, 0, 2]]].node.add_cube(0.9, 0.0, 0.9);
    
    c12.set_local_translation(Translation3::new(-1.0, -1.0, 1.5));
    c13.set_local_translation(Translation3::new(-1.5, -1.0, 1.0));
    c14.set_local_translation(Translation3::new(-1.0, -1.5, 1.0));

    c12.set_color(1.0, 0.0, 0.0);
    c13.set_color(1.0, 1.0, 1.0);
    c14.set_color(0.4, 0.3, 0.6);

    let mut c15 = gs[cn[[1, 0, 2]]].node.add_cube(0.9, 0.9, 0.0);
    let mut c16 = gs[cn[[1, 0, 2]]].node.add_cube(0.9, 0.0, 0.9);
    
    c15.set_local_translation(Translation3::new(0.0, -1.0, 1.5));
    c16.set_local_translation(Translation3::new(0.0, -1.5, 1.0));

    c15.set_color(1.0, 0.0, 0.0);
    c16.set_color(0.4, 0.3, 0.6);


    let mut c17 = gs[cn[[0, 2, 0]]].node.add_cube(0.9, 0.9, 0.0);
    let mut c18 = gs[cn[[0, 2, 0]]].node.add_cube(0.0, 0.9, 0.9);
    let mut c19 = gs[cn[[0, 2, 0]]].node.add_cube(0.9, 0.0, 0.9);
    
    c17.set_local_translation(Translation3::new(1.0, 1.0, -1.5));
    c18.set_local_translation(Translation3::new(1.5, 1.0, -1.0));
    c19.set_local_translation(Translation3::new(1.0, 1.5, -1.0));

    c17.set_color(0.9, 0.5, 0.1);
    c18.set_color(0.0, 1.0, 0.0);
    c19.set_color(0.0, 0.0, 1.0);

    let mut c20 = gs[cn[[2, 2, 0]]].node.add_cube(0.9, 0.9, 0.0);
    let mut c21 = gs[cn[[2, 2, 0]]].node.add_cube(0.0, 0.9, 0.9);
    let mut c22 = gs[cn[[2, 2, 0]]].node.add_cube(0.9, 0.0, 0.9);
    
    c20.set_local_translation(Translation3::new(-1.0, 1.0, -1.5));
    c21.set_local_translation(Translation3::new(-1.5, 1.0, -1.0));
    c22.set_local_translation(Translation3::new(-1.0, 1.5, -1.0));

    c20.set_color(0.9, 0.5, 0.1);
    c21.set_color(1.0, 1.0, 1.0);
    c22.set_color(0.0, 0.0, 1.0);

    let mut c23 = gs[cn[[1, 2, 0]]].node.add_cube(0.9, 0.9, 0.0);
    let mut c24 = gs[cn[[1, 2, 0]]].node.add_cube(0.9, 0.0, 0.9);
    
    c23.set_local_translation(Translation3::new(0.0, 1.0, -1.5));
    c24.set_local_translation(Translation3::new(0.0, 1.5, -1.0));

    c23.set_color(0.9, 0.5, 0.1);
    c24.set_color(0.0, 0.0, 1.0);

    let mut c25 = gs[cn[[0, 0, 0]]].node.add_cube(0.9, 0.9, 0.0);
    let mut c26 = gs[cn[[0, 0, 0]]].node.add_cube(0.0, 0.9, 0.9);
    let mut c27 = gs[cn[[0, 0, 0]]].node.add_cube(0.9, 0.0, 0.9);
    
    c25.set_local_translation(Translation3::new(1.0, -1.0, -1.5));
    c26.set_local_translation(Translation3::new(1.5, -1.0, -1.0));
    c27.set_local_translation(Translation3::new(1.0, -1.5, -1.0));

    c25.set_color(0.9, 0.5, 0.1);
    c26.set_color(0.0, 1.0, 0.0);
    c27.set_color(0.4, 0.3, 0.6);

    let mut c28 = gs[cn[[2, 0, 0]]].node.add_cube(0.9, 0.9, 0.0);
    let mut c29 = gs[cn[[2, 0, 0]]].node.add_cube(0.0, 0.9, 0.9);
    let mut c30 = gs[cn[[2, 0, 0]]].node.add_cube(0.9, 0.0, 0.9);
    
    c28.set_local_translation(Translation3::new(-1.0, -1.0, -1.5));
    c29.set_local_translation(Translation3::new(-1.5, -1.0, -1.0));
    c30.set_local_translation(Translation3::new(-1.0, -1.5, -1.0));

    c28.set_color(0.9, 0.5, 0.1);
    c29.set_color(1.0, 1.0, 1.0);
    c30.set_color(0.4, 0.3, 0.6);

    let mut c31 = gs[cn[[1, 0, 0]]].node.add_cube(0.9, 0.9, 0.0);
    let mut c32 = gs[cn[[1, 0, 0]]].node.add_cube(0.9, 0.0, 0.9);
    
    c31.set_local_translation(Translation3::new(0.0, -1.0, -1.5));
    c32.set_local_translation(Translation3::new(0.0, -1.5, -1.0));

    c31.set_color(0.9, 0.5, 0.1);
    c32.set_color(0.4, 0.3, 0.6);


    let mut c33 = gs[cn[[0, 1, 2]]].node.add_cube(0.0, 0.9, 0.9);
    let mut c34 = gs[cn[[0, 1, 2]]].node.add_cube(0.9, 0.9, 0.0);
    
    c33.set_local_translation(Translation3::new(1.5, 0.0, 1.0));
    c34.set_local_translation(Translation3::new(1.0, 0.0, 1.5));

    c33.set_color(0.0, 1.0, 0.0);
    c34.set_color(1.0, 0.0, 0.0);


    let mut c35 = gs[cn[[2, 1, 2]]].node.add_cube(0.0, 0.9, 0.9);
    let mut c36 = gs[cn[[2, 1, 2]]].node.add_cube(0.9, 0.9, 0.0);
    
    c35.set_local_translation(Translation3::new(-1.5, 0.0, 1.0));
    c36.set_local_translation(Translation3::new(-1.0, 0.0, 1.5));

    c35.set_color(1.0, 1.0, 1.0);
    c36.set_color(1.0, 0.0, 0.0);


    let mut c37 = gs[cn[[0, 1, 0]]].node.add_cube(0.0, 0.9, 0.9);
    let mut c38 = gs[cn[[0, 1, 0]]].node.add_cube(0.9, 0.9, 0.0);
    
    c37.set_local_translation(Translation3::new(1.5, 0.0, -1.0));
    c38.set_local_translation(Translation3::new(1.0, 0.0, -1.5));

    c37.set_color(0.0, 1.0, 0.0);
    c38.set_color(0.9, 0.5, 0.1);


    let mut c39 = gs[cn[[2, 1, 0]]].node.add_cube(0.0, 0.9, 0.9);
    let mut c40 = gs[cn[[2, 1, 0]]].node.add_cube(0.9, 0.9, 0.0);
    
    c39.set_local_translation(Translation3::new(-1.5, 0.0, -1.0));
    c40.set_local_translation(Translation3::new(-1.0, 0.0, -1.5));

    c39.set_color(1.0, 1.0, 1.0);
    c40.set_color(0.9, 0.5, 0.1);



    let mut c41 = gs[cn[[0, 2, 1]]].node.add_cube(0.0, 0.9, 0.9);
    let mut c42 = gs[cn[[0, 2, 1]]].node.add_cube(0.9, 0.0, 0.9);
    
    c41.set_local_translation(Translation3::new(1.5, 1.0, 0.0));
    c42.set_local_translation(Translation3::new(1.0, 1.5, 0.0));

    c41.set_color(0.0, 1.0, 0.0);
    c42.set_color(0.0, 0.0, 1.0);


    let mut c43 = gs[cn[[2, 2, 1]]].node.add_cube(0.0, 0.9, 0.9);
    let mut c44 = gs[cn[[2, 2, 1]]].node.add_cube(0.9, 0.0, 0.9);
    
    c43.set_local_translation(Translation3::new(-1.5, 1.0, 0.0));
    c44.set_local_translation(Translation3::new(-1.0, 1.5, 0.0));

    c43.set_color(1.0, 1.0, 1.0);
    c44.set_color(0.0, 0.0, 1.0);


    let mut c45 = gs[cn[[0, 0, 1]]].node.add_cube(0.0, 0.9, 0.9);
    let mut c46 = gs[cn[[0, 0, 1]]].node.add_cube(0.9, 0.0, 0.9);
    
    c45.set_local_translation(Translation3::new(1.5, -1.0, 0.0));
    c46.set_local_translation(Translation3::new(1.0, -1.5, 0.0));

    c45.set_color(0.0, 1.0, 0.0);
    c46.set_color(0.4, 0.3, 0.6);


    let mut c47 = gs[cn[[2, 0, 1]]].node.add_cube(0.0, 0.9, 0.9);
    let mut c48 = gs[cn[[2, 0, 1]]].node.add_cube(0.9, 0.0, 0.9);
    
    c47.set_local_translation(Translation3::new(-1.5, -1.0, 0.0));
    c48.set_local_translation(Translation3::new(-1.0, -1.5, 0.0));

    c47.set_color(1.0, 1.0, 1.0);
    c48.set_color(0.4, 0.3, 0.6);

    let mut c50 = gs[cn[[1, 0, 1]]].node.add_cube(0.9, 0.0, 0.9);
    c50.set_local_translation(Translation3::new(0.0, -1.5, 0.0));
    c50.set_color(0.4, 0.3, 0.6);

    let mut c51 = gs[cn[[1, 2, 1]]].node.add_cube(0.9, 0.0, 0.9);
    c51.set_local_translation(Translation3::new(0.0, 1.5, 0.0));
    c51.set_color(0.0, 0.0, 1.0);

    let mut c52 = gs[cn[[0, 1, 1]]].node.add_cube(0.0, 0.9, 0.9);
    c52.set_local_translation(Translation3::new(1.5, 0.0, 0.0));
    c52.set_color(0.0, 1.0, 0.0);

    let mut c53 = gs[cn[[2, 1, 1]]].node.add_cube(0.0, 0.9, 0.9);
    c53.set_local_translation(Translation3::new(-1.5, 0.0, 0.0));
    c53.set_color(1.0, 1.0, 1.0);

    let mut c54 = gs[cn[[1, 1, 2]]].node.add_cube(0.9, 0.9, 0.0);
    c54.set_local_translation(Translation3::new(0.0, 0.0, 1.5));
    c54.set_color(1.0, 0.0, 0.0);

    let mut c55 = gs[cn[[1, 1, 0]]].node.add_cube(0.9, 0.9, 0.0);
    c55.set_local_translation(Translation3::new(0.0, 0.0, -1.5));
    c55.set_color(0.9, 0.5, 0.1);


    window.set_light(Light::StickToCamera);
    let speed = match env::args().nth(1) {
        Some(i) => i.parse::<i32>().unwrap_or(10),
        None => 10,
    };

    let rot =  [
        [UnitQuaternion::from_axis_angle(&Vector3::x_axis(), -std::f32::consts::PI/2.0/(speed as f32)), UnitQuaternion::from_axis_angle(&Vector3::x_axis(), std::f32::consts::PI/2.0/(speed as f32))],
        [UnitQuaternion::from_axis_angle(&Vector3::y_axis(), std::f32::consts::PI/2.0/(speed as f32)), UnitQuaternion::from_axis_angle(&Vector3::y_axis(), -std::f32::consts::PI/2.0/(speed as f32))],
        [UnitQuaternion::from_axis_angle(&Vector3::z_axis(), std::f32::consts::PI/2.0/(speed as f32)), UnitQuaternion::from_axis_angle(&Vector3::z_axis(), -std::f32::consts::PI/2.0/(speed as f32))],
        ];

    let get_index = |k: usize, axis: usize, index: usize| -> [usize; 3] {
        let i: usize;
        let j: usize;
        match k {
            0 => {
                i = 0;
                j = 0;
            },
            1 => {
                i = 0;
                j = 1;
            },
            2 => {
                i = 0;
                j = 2;
            },
            3 => {
                i = 1;
                j = 2;
            },
            4 => {
                i = 2;
                j = 2;
            },
            5 => {
                i = 2;
                j = 1;
            },
            6 => {
                i = 2;
                j = 0;
            },
            _ => {
                i = 1;
                j = 0;
            },
        };
        match axis {
            0 => [index, i, j],
            1 => [j, index, i],
            _ => [i, j, index],
        }
    };

    let mut count = 0;
    let mut rotate = |axis :usize, index :usize, direct :usize| {
        count += 1;
        println!("{}", count);
        for _ in 0..speed {
            for i in 0..3 {
                for j in 0..3 {
                    let (x, y, z) = match axis {
                        0 => (index, i, j),
                        1 => (j, index, i),
                        _ => (i, j, index),
                    };
                    let c = cn[[x, y, z]];
                    gs[c].node.append_rotation(&rot[axis][direct]);
                }
            };
            window.render();
        }

        for i in 0..3 {
            for j in 0..3 {
                let (x, y, z) = match axis {
                    0 => (index, i, j),
                    1 => (j, index, i),
                    _ => (i, j, index),
                };
                let c = cn[[x, y, z]];
                if gs[c].face != axis {
                    gs[c].face = match axis {
                        0 => match gs[c].face {
                            2 => 1,
                            _ => 2,
                        },
                        1 => match gs[c].face {
                            2 => 0,
                            _ => 2,
                        },
                        _ => match gs[c].face {
                            1 => 0,
                            _ => 1,
                        },
                    };
                }
            }
        };
        
        let mut arr = [0; 8];
        let mut k = 0;
        for i in 0..8 {
            arr[k] = cn[get_index(i, axis, index)];
            k += 1;
        }

        for i in 0..8 {
            let mut j: i8;
            if direct == 1 {
                j = i + 2;
                if j > 7 {
                    j -= 8;
                }
            } else {
                j = i - 2;
                if j < 0 {
                    j += 8;
                }
            }
            cn[get_index(i as usize, axis, index)] = arr[j as usize];
        }
        let mut x = 0;
        for i in 0..3 {
            for j in 0..3 {
                for k in 0..3 {
                    let index = cn[[i, j, k]];
                    if index != x {
                        return;
                    }
                    x += 1;
                }
            }
        }
        for i in (0..3).step_by(2) {
            for j in (0..3).step_by(2) {
                for k in (0..3).step_by(2) {
                    let index = cn[[i, j, k]];
                    if gs[index].face != 1 {
                        return;
                    }
                }
            }
        }

        while  window.render() {
        }
        std::process::exit(0);
    };

    // loop {
    //     rotate(2, 0, 1);
    //     rotate(0, 0, 1);
    //     rotate(2, 2, 0);
    //     rotate(0, 2, 0);
    // }
    let mut rng = rand::thread_rng();

    loop {
        let axis: usize = rng.gen_range(0..3);
        let index: usize = rng.gen_range(0..2) * 2;
        let direct: usize = rng.gen_range(0..2);
        rotate(axis, index, direct);
    }
}

    //00 01 02 12 22 21 20 10

    //02 12 22 21 20 10 00 01
    //20 10 00 01 02 12 22 21

    //x -> yz
    //y -> zx
    //z -> xy

    //  6  15  24
    //  3  12  21
    //  0  9   18
    
    //  7  16  25
    //  4  13  22
    //  1  10  19

    //  8  17  26
    //  5  14  23
    //  2  11  20
