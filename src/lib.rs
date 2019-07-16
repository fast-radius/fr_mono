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
    (p2 - p1).cross(&(p3 - p1))
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

pub fn to_stl(s: &str) -> Vec<Triangle> {
    let mut output = vec![];
    for (i, c) in s.chars().enumerate() {
        let translation = Vector3::new(6.0 * i as f32, 0.0, 0.0);

        if c != ' ' {
            let stl_letter = char_to_stl_letter(&c);

            for triangle in stl_letter {
                let t = Triangle {
                    normal: triangle.normal,
                    vertices: [
                        triangle.vertices[0] + translation,
                        triangle.vertices[1] + translation,
                        triangle.vertices[2] + translation,
                    ],
                };

                output.push(t);
            }
        }
    }

    output
}

fn char_to_stl_letter(c: &char) -> Vec<Triangle> {
    LETTERS[&c.to_lowercase().to_string().chars().next().unwrap()].clone()
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
    pub static ref LETTERS: HashMap<char, Vec<Triangle>> = {
        let mut m = HashMap::new();

        // numbers
        m.insert('0', zero());
        m.insert('1', one());
        m.insert('2', two());
        m.insert('3', three());
        m.insert('4', four());
        m.insert('5', five());
        m.insert('6', six());
        m.insert('7', seven());
        m.insert('8', eight());
        m.insert('9', nine());

        // letters
        m.insert('a', a());
        m.insert('b', b());
        m.insert('c', c());
        m.insert('d', d());
        m.insert('e', e());
        m.insert('f', f());

        // m.insert('g', g());
        // m.insert('h', h());
        m.insert('i', i());
        // m.insert('j', j());
        m.insert('k', k());
        // m.insert('l', l());
        // m.insert('m', m());
        // m.insert('n', n());
        // m.insert('o', o());
        m.insert('p', p());
        // m.insert('q', q());
        // m.insert('r', r());
        // m.insert('s', s());
        // m.insert('t', t());
        // m.insert('u', u());
        m.insert('v', v());
        // m.insert('w', w());
        // m.insert('x', x());
        // m.insert('y', y());
        // m.insert('z', z());

        // symbols
        m.insert('-', hyphen());

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
    fn it_uses_the_high_level_api() {
        let s = "abcdef-0123456789";
        let stl = to_stl(s);
        write_stl("hello.stl", &stl).unwrap();
        assert_eq!(2 + 2, 4);
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
