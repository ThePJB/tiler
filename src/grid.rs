pub struct Grid<T> {
    pub w: usize,
    pub h: usize,
    pub elements: Vec<T>,
}

#[derive(Clone, Copy)]
pub enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    pub fn opposite(&self) -> Dir {
        match self {
            Dir::North => Dir::South,
            Dir::East => Dir::West,
            Dir::South => Dir::North,
            Dir::West => Dir::East,
        }
    }
}

pub fn idx_in_dir(i: usize, j: usize, dir: Dir) -> (usize, usize) {
    match dir {
        Dir::North => (i, j-1),
        Dir::South => (i, j+1),
        Dir::West => (i-1, j),
        Dir::East => (i+1, j),
    }
}


impl<T: std::marker::Copy> Grid<T> {
    pub fn new(w: usize, h: usize, default: T) -> Grid<T>{
        Grid {
            w,
            h,
            elements: vec![default; w*h],
        }
    }

    pub fn get(&self, i: usize, j: usize) -> T {
        self.elements[i*self.w + j]
    }

    pub fn set(&mut self, i: usize, j: usize, elem: T) {
        self.elements[i*self.w + j] = elem;
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> &mut T {
        &mut self.elements[i*self.w + j]
    }

    pub fn offset_mut(&mut self, i: usize, j: usize, oi: i32, oj: i32) -> Option<&mut T> {
        let ti = i as i32 + oi;
        let tj = j as i32 + oj;
        if ti < 0 || ti >= self.w as i32 {
            return None;
        }
        if tj < 0 || tj >= self.h as i32 {
            return None;
        } 

        return Some(&mut self.elements[ti as usize*self.w + tj as usize])
    }

    pub fn neighbour_mut(&mut self, i: usize, j: usize, dir: Dir) -> Option<&mut T> {
        match dir {
            Dir::North => {
                if j > 0 {
                    Some(&mut self.elements[i*self.w + (j-1)])
                } else {
                    None
                }
            },
            Dir::South => {
                if j < self.h - 1 {
                    Some(&mut self.elements[i*self.w + (j+1)])
                } else {
                    None
                }
            },
            Dir::West => {
                if i > 0 {
                    Some(&mut self.elements[(i-1)*self.w + j])
                } else {
                    None
                }
            },
            Dir::East => {
                if i < self.w - 1 {
                    Some(&mut self.elements[(i+1)*self.w + j])
                } else {
                    None
                }
            },
        }
    }
}