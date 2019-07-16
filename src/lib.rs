use lazy_static::lazy_static;
use nalgebra::{Point3, Vector3};
use std::collections::HashMap;

type Face = [Point3<f32>; 3];

#[derive(Clone, Copy, Debug)]
pub struct Triangle {
    pub normal: Vector3<f32>,
    pub vertices: Face,
}

fn normal(face: &Face) -> Vector3<f32> {
    let [p1, p2, p3] = face;
    let n = (p2 - p1).cross(&(p3 - p1));
    println!("{}", n);
    n
}

/// assumes:
/// X = left/right
/// Y = up/down
/// Z = in/out
fn cube() -> Vec<Triangle> {
    let vertices = [
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 1.0),
        Point3::new(0.0, 1.0, 1.0),
        Point3::new(1.0, 1.0, 1.0),
        Point3::new(1.0, 0.0, 1.0),
    ];

    let indices = [
        [3, 0, 1],
        [3, 1, 2],
        [5, 1, 0],
        [4, 5, 0],
        [2, 1, 5],
        [5, 6, 2],
        [2, 6, 7],
        [3, 2, 7],
        [0, 3, 7],
        [0, 7, 4],
        [4, 6, 5],
        [7, 6, 4],
    ];

    indices
        .iter()
        .map(|group| {
            let face = [vertices[group[0]], vertices[group[1]], vertices[group[2]]];
            let normal = normal(&face).normalize();
            Triangle {
                normal: normal,
                vertices: face,
            }
        })
        .collect()
}

macro_rules! letter {
    ($name:ident, $def:expr) => {
        fn $name() -> Vec<Triangle> {
            let mut cubes = vec![];

            for (y, row) in $def.iter().enumerate() {
                for (x, column) in row.iter().enumerate() {
                    if *column == 1 {
                        let translation = Vector3::new(x as f32, -1.0 * y as f32, 0.0);

                        for triangle in cube().iter() {
                            let t = Triangle {
                                normal: triangle.normal,
                                vertices: [
                                    triangle.vertices[0] + translation,
                                    triangle.vertices[1] + translation,
                                    triangle.vertices[2] + translation,
                                ],
                            };

                            cubes.push(t);
                        }
                    }
                }
            }

            cubes
        }
    };
}

letter! {
    zero,
    [
        [0, 1, 1, 1, 0],
        [1, 0, 0, 1, 1],
        [1, 0, 1, 0, 1],
        [1, 1, 0, 0, 1],
        [0, 1, 1, 1, 0]
    ]
}

letter! {
    one,
    [
        [0, 1, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 0, 1, 0, 0],
        [0, 1, 1, 1, 0]
    ]
}

letter! {
    two,
    [
        [0, 1, 1, 0, 0],
        [1, 0, 0, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 1, 0, 0, 0],
        [1, 1, 1, 1, 0]
    ]
}

letter! {
    three,
    [
        [1, 1, 1, 0, 0],
        [0, 0, 0, 1, 0],
        [0, 1, 1, 0, 0],
        [0, 0, 0, 1, 0],
        [1, 1, 1, 0, 0],
    ]
}

letter! {
    four,
    [
        [1, 0, 0, 1, 0],
        [1, 0, 0, 1, 0],
        [1, 1, 1, 1, 0],
        [0, 0, 0, 1, 0],
        [0, 0, 0, 1, 0],
    ]
}

letter! {
    five,
    [
        [1, 1, 1, 1, 0],
        [1, 0, 0, 0, 0],
        [1, 1, 1, 0, 0],
        [0, 0, 0, 1, 0],
        [1, 1, 1, 0, 0],
    ]
}

letter! {
    six,
    [
        [1, 1, 1, 1, 0],
        [1, 0, 0, 0, 0],
        [1, 1, 1, 1, 0],
        [1, 0, 0, 1, 0],
        [1, 1, 1, 1, 0],
    ]
}

letter! {
    seven,
    [
        [1, 1, 1, 1, 0],
        [0, 0, 0, 1, 0],
        [0, 0, 1, 0, 0],
        [0, 1, 0, 0, 0],
        [0, 1, 0, 0, 0],
    ]
}

letter! {
    eight,
    [
        [1, 1, 1, 1, 0],
        [1, 0, 0, 1, 0],
        [1, 1, 1, 1, 0],
        [1, 0, 0, 1, 0],
        [1, 1, 1, 1, 0],
    ]
}

letter! {
    nine,
    [
        [1, 1, 1, 1, 0],
        [1, 0, 0, 1, 0],
        [1, 1, 1, 1, 0],
        [0, 0, 0, 1, 0],
        [1, 1, 1, 1, 0]
    ]
}

letter! {
    a,
    [
        [0, 1, 1, 0, 0],
        [1, 0, 0, 1, 0],
        [1, 1, 1, 1, 0],
        [1, 0, 0, 1, 0],
        [1, 0, 0, 1, 0]
    ]
}

letter! {
    b,
    [
        [1, 1, 1, 0, 0],
        [1, 0, 0, 1, 0],
        [1, 1, 1, 0, 0],
        [1, 0, 0, 1, 0],
        [1, 1, 1, 0, 0]
    ]
}

letter! {
    c,
    [
        [0, 1, 1, 1, 0],
        [1, 0, 0, 0, 0],
        [1, 0, 0, 0, 0],
        [1, 0, 0, 0, 0],
        [0, 1, 1, 1, 0]

    ]
}

letter! {
    d,
    [
        [1, 1, 1, 1, 0],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 1, 1, 1, 0]
    ]
}

letter! {
    e,
    [
        [1, 1, 1, 1, 0],
        [1, 0, 0, 0, 0],
        [1, 1, 1, 1, 0],
        [1, 0, 0, 0, 0],
        [1, 1, 1, 1, 0],
    ]
}

letter! {
    f,
    [
        [1, 1, 1, 1, 0],
        [1, 0, 0, 0, 0],
        [1, 1, 1, 1, 0],
        [1, 0, 0, 0, 0],
        [1, 0, 0, 0, 0],
    ]
}

letter! {
    k,
    [
        [1, 0, 0, 1, 0],
        [1, 0, 1, 0, 0],
        [1, 1, 0, 0, 0],
        [1, 0, 1, 0, 0],
        [1, 0, 0, 1, 0],
    ]
}

letter! {
    v,
    [
        [1, 0, 0, 0, 1],
        [1, 0, 0, 0, 1],
        [1, 1, 0, 1, 1],
        [0, 1, 0, 1, 0],
        [0, 0, 1, 0, 0],
    ]
}

letter! {
    i,
    [
        [1,1,1,1,1],
        [0,0,1,0,0],
        [0,0,1,0,0],
        [0,0,1,0,0],
        [1,1,1,1,1],

    ]
}

letter! {
    p,
    [
        [1,1,1,1,0],
        [1,0,0,1,0],
        [1,1,1,1,0],
        [1,0,0,0,0],
        [1,0,0,0,0]
    ]
}

letter! {
    hyphen,
    [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 1, 1, 1, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0]
    ]
}

lazy_static! {
    pub static ref LETTERS: HashMap<String, Vec<Triangle>> = {
        let mut m = HashMap::new();

        // numbers
        m.insert("0".to_string(), zero());
        m.insert("1".to_string(), one());
        m.insert("2".to_string(), two());
        m.insert("3".to_string(), three());
        m.insert("4".to_string(), four());
        m.insert("5".to_string(), five());
        m.insert("6".to_string(), six());
        m.insert("7".to_string(), seven());
        m.insert("8".to_string(), eight());
        m.insert("9".to_string(), nine());

        // letters
        m.insert("a".to_string(), a());
        m.insert("b".to_string(), b());
        m.insert("c".to_string(), c());
        m.insert("d".to_string(), d());
        m.insert("e".to_string(), e());
        m.insert("f".to_string(), f());

        // m.insert("g".to_string(), g());
        // m.insert("h".to_string(), h());
        m.insert("i".to_string(), i());
        // m.insert("j".to_string(), j());
        m.insert("k".to_string(), k());
        // m.insert("l".to_string(), l());
        // m.insert("m".to_string(), m());
        // m.insert("n".to_string(), n());
        // m.insert("o".to_string(), o());
        m.insert("p".to_string(), p());
        // m.insert("q".to_string(), q());
        // m.insert("r".to_string(), r());
        // m.insert("s".to_string(), s());
        // m.insert("t".to_string(), t());
        // m.insert("u".to_string(), u());
        m.insert("v".to_string(), v());
        // m.insert("w".to_string(), w());
        // m.insert("x".to_string(), x());
        // m.insert("y".to_string(), y());
        // m.insert("z".to_string(), z());

        // symbols
        m.insert("-".to_string(), hyphen());

        m
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use byteorder::{LittleEndian, WriteBytesExt};
    use std::fs::File;
    use std::io::BufWriter;
    use std::io::Write;

    /// write stl binary, like this:
    ///
    /// UINT8[80] – Header
    /// UINT32 – Number of triangles
    /// foreach triangle
    ///   REAL32[3] – Normal vector
    ///   REAL32[3] – Vertex 1
    ///   REAL32[3] – Vertex 2
    ///   REAL32[3] – Vertex 3
    ///   UINT16 – Attribute byte count
    /// end
    fn write_stl(path: &str, triangles: &[Triangle]) -> std::io::Result<()> {
        let output_file = File::create(path)?;

        let mut output_buf = BufWriter::new(output_file);

        // write 80 byte header
        let header = [0u8; 80];
        output_buf.write_all(&header)?;

        // write triangles count
        output_buf.write_u32::<LittleEndian>(triangles.len() as u32)?;

        // write triangles
        for triangle in triangles {
            // write normal
            output_buf.write_f32::<LittleEndian>(triangle.normal[0])?;
            output_buf.write_f32::<LittleEndian>(triangle.normal[1])?;
            output_buf.write_f32::<LittleEndian>(triangle.normal[2])?;

            // write each coordinate of the 3 vertices
            for vertex in triangle.vertices.iter() {
                output_buf.write_f32::<LittleEndian>(vertex[0])?;
                output_buf.write_f32::<LittleEndian>(vertex[1])?;
                output_buf.write_f32::<LittleEndian>(vertex[2])?;
            }

            // write attribute byte count
            output_buf.write_u16::<LittleEndian>(0)?;
        }

        Ok(())
    }

    #[test]
    fn it_does_a_cube() {
        let cube = cube();
        write_stl("cube.stl", &cube).unwrap();
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_does_letters() {
        let input = [a(), b(), c(), d(), e(), f()];
        let mut letters = vec![];
        for (i, letter) in input.iter().enumerate() {
            let translation = Vector3::new(6.0 * i as f32, 0.0, 0.0);

            for triangle in letter {
                let t = Triangle {
                    normal: triangle.normal,
                    vertices: [
                        triangle.vertices[0] + translation,
                        triangle.vertices[1] + translation,
                        triangle.vertices[2] + translation,
                    ],
                };

                letters.push(t);
            }
        }
        write_stl("letters.stl", &letters).unwrap();
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_does_pick() {
        let david = vec![d(), a(), v(), i(), d()];
        let pick = vec![p(), i(), c(), k()];

        let mut name = vec![];

        for (i, letter) in david.iter().enumerate() {
            let translation = Vector3::new(6.0 * i as f32, 0.0, 0.0);

            for triangle in letter {
                let t = Triangle {
                    normal: triangle.normal,
                    vertices: [
                        triangle.vertices[0] + translation,
                        triangle.vertices[1] + translation,
                        triangle.vertices[2] + translation,
                    ],
                };

                name.push(t);
            }
        }

        for (i, letter) in pick.iter().enumerate() {
            let translation = Vector3::new(6.0 * i as f32, 0.0, 0.0) + Vector3::new(35.0, 0.0, 0.0);

            for triangle in letter {
                let t = Triangle {
                    normal: triangle.normal,
                    vertices: [
                        triangle.vertices[0] + translation,
                        triangle.vertices[1] + translation,
                        triangle.vertices[2] + translation,
                    ],
                };

                name.push(t);
            }
        }
        // let mut stl: Vec<Triangle> = vec![];
        // let translation = Vector3::new(0.0, 0.0, 0.0);

        // for c in "david pick".chars() {
        //     if c != ' ' {
        //         let stl_letter = &LETTERS[&c.to_string()];
        //         stl.extend(stl_letter);
        //     } else {
        //     }
        // }

        write_stl("davidpick.stl", &name).unwrap();
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_does_numbers() {
        let nums = vec![
            zero(),
            one(),
            two(),
            three(),
            four(),
            five(),
            six(),
            seven(),
            eight(),
            nine(),
            hyphen(),
        ];
        let mut numbers = vec![];

        for (i, letter) in nums.iter().enumerate() {
            let translation = Vector3::new(6.0 * i as f32, 0.0, 0.0);

            for triangle in letter {
                let t = Triangle {
                    normal: triangle.normal,
                    vertices: [
                        triangle.vertices[0] + translation,
                        triangle.vertices[1] + translation,
                        triangle.vertices[2] + translation,
                    ],
                };

                numbers.push(t);
            }
        }
        write_stl("numbers.stl", &numbers).unwrap();
        assert_eq!(2 + 2, 4);
    }
}
