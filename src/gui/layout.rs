use crate::util::Repack;
use core::ops::{Add, AddAssign};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Position(pub i32, pub i32);
impl Position {
    pub const fn x(self) -> i32 {
        self.0
    }
    pub const fn y(self) -> i32 {
        self.1
    }
}
impl Add for Position {
    type Output = Self;
    fn add(self, Self(x1, y1): Self) -> Self::Output {
        Self(self.0 + x1, self.1 + y1)
    }
}
impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}
impl AddAssign<Size> for Position {
    fn add_assign(&mut self, rhs: Size) {
        *self = *self + rhs
    }
}
impl Add<Size> for Position {
    type Output = Self;
    fn add(self, Size(x1, y1): Size) -> Self::Output {
        Self(self.0 + x1, self.1 + y1)
    }
}
impl<T> Repack<T> for Position
where
    [i32; 2]: Repack<T>,
{
    fn repack(self) -> T {
        [self.0, self.1].repack()
    }
}
impl<T: Repack<[i32; 2]>> Repack<Position> for T {
    fn repack(self) -> Position {
        let repacked = self.repack();
        Position(repacked[0], repacked[1])
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Size(pub i32, pub i32);
impl Size {
    pub const fn w(self) -> i32 {
        self.0
    }
    pub const fn h(self) -> i32 {
        self.1
    }
}
impl Add for Size {
    type Output = Self;
    fn add(self, Self(w1, h1): Self) -> Self::Output {
        Self(self.0 + w1, self.1 + h1)
    }
}
impl AddAssign for Size {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}
impl<T> Repack<T> for Size
where
    [i32; 2]: Repack<T>,
{
    fn repack(self) -> T {
        [self.0, self.1].repack()
    }
}
impl<T: Repack<[i32; 2]>> Repack<Size> for T {
    fn repack(self) -> Size {
        let repacked = self.repack();
        Size(repacked[0], repacked[1])
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Rect(pub Position, pub Size);
impl Rect {
    pub fn with_added_pos(mut self, pos: Position) -> Self {
        self.0 += pos;
        self
    }
    pub fn with_added_size(mut self, size: Size) -> Self {
        self.1 += size;
        self
    }
    pub const fn pos(self) -> Position {
        self.0
    }
    pub const fn size(self) -> Size {
        self.1
    }
    pub const fn x(self) -> i32 {
        self.pos().x()
    }
    pub const fn y(self) -> i32 {
        self.pos().y()
    }
    pub const fn w(self) -> i32 {
        self.size().w()
    }
    pub const fn h(self) -> i32 {
        self.size().h()
    }
    pub const fn to_right(self, padding: i32) -> Position {
        let x = self.x() + self.w() + padding;
        Position(x, self.y())
    }
    pub const fn to_bottom(self, padding: i32) -> Position {
        let y = self.y() + self.h() + padding;
        Position(self.x(), y)
    }
}

pub trait LayoutGen<'a> {
    type Arguments: 'a;
    type Layout;
    fn generate_layout(&self, arguments: Self::Arguments) -> Self::Layout;
}

macro_rules! make_layout {
    ($v:vis $name:ident, has $($entry:ident),+ $(,)?) => {
        #[derive(Copy, Clone, Debug)]
        $v struct $name {
            pub total_size: $crate::layout::Size,
            pub $($entry: $crate::layout::Rect,)+
        }
    };
}
