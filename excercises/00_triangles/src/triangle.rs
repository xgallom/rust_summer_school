use std::convert::TryFrom;
use std::io;
use std::io::Write;

use num_enum::TryFromPrimitive;

pub const TRIANGLE_SIDES: usize = 3;

#[derive(Debug, TryFromPrimitive)]
#[repr(usize)]
pub enum TriangleSide {
    A,
    B,
    C,
}

#[derive(Debug)]
pub enum TriangleType {
    Equilateral,
    Isoceles,
    Scalene,
}

pub type TriangleSides = [u32; TRIANGLE_SIDES];

#[derive(Debug)]
pub struct Triangle {
    sides: TriangleSides,
    squares: TriangleSides,
    perimeter: u32,
    squares_sum: u32,
}

impl Triangle {
    pub fn read_from_console() -> Triangle {
        let mut result = TriangleSides::default();
        let mut line = String::new();

        for triangle_side in 0..TRIANGLE_SIDES {
            line.clear();

            print!(
                "Enter side {:?}: ",
                TriangleSide::try_from(triangle_side).unwrap()
            );

            io::stdout().flush().expect("Failed to flush stdout");
            io::stdin()
                .read_line(&mut line)
                .expect("Failed to read from stdin");

            result[triangle_side] = line.trim().parse().unwrap();
        }

        Triangle::from_sides(result)
    }

    fn from_sides(sides: TriangleSides) -> Triangle {
        let mut squares = TriangleSides::default();
        for (side, square) in sides.iter().zip(squares.iter_mut()) {
            *square = side * side;
        }

        let perimeter: u32 = sides.iter().sum();
        let squares_sum: u32 = squares.iter().sum();

        Triangle {
            sides,
            squares,
            perimeter,
            squares_sum,
        }
    }

    pub fn sides(&self) -> &TriangleSides {
        &self.sides
    }
    pub fn side(&self, index: usize) -> u32 {
        self.sides[index]
    }
    pub fn squares(&self) -> &TriangleSides {
        &self.squares
    }
    pub fn perimeter(&self) -> u32 {
        self.perimeter
    }
    pub fn squares_sum(&self) -> u32 {
        self.squares_sum
    }

    pub fn is_valid(&self) -> bool {
        for triangle_side in self.sides() {
            if self.perimeter() <= triangle_side * 2 {
                return false;
            }
        }

        true
    }

    pub fn is_right(&self) -> bool {
        self.squares()
            .iter()
            .any(|square| self.squares_sum() == square * 2)
    }

    fn is_equiliteral(&self) -> Option<TriangleType> {
        let first_side = self.side(TriangleSide::A as usize);

        if self.sides().iter().skip(1).all(|side| *side == first_side) {
            return Some(TriangleType::Equilateral);
        }

        None
    }

    fn is_isoceles(&self) -> Option<TriangleType> {
        for first_side in 0..(TRIANGLE_SIDES - 1) {
            for second_side in (first_side + 1)..TRIANGLE_SIDES {
                if self.side(first_side) == self.side(second_side) {
                    return Some(TriangleType::Isoceles);
                }
            }
        }

        None
    }

    pub fn triangle_type(&self) -> TriangleType {
        self.is_equiliteral()
            .or(self.is_isoceles())
            .unwrap_or(TriangleType::Scalene)
    }
}
