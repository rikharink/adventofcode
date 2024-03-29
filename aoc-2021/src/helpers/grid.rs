use glam::{ivec2, IVec2};

lazy_static! {
    static ref DIRECTIONS_4: [IVec2; 4] = [
        //LEFT
        ivec2(-1, 0),
        //RIGHT
        ivec2(1, 0),
        //DOWN
        ivec2(0, -1),
        //UP
        ivec2(0, 1),
    ];

    static ref DIRECTIONS_8: [IVec2; 8] = [
        //LEFT
        ivec2(-1, 0),
        //RIGHT
        ivec2(1, 0),
        //DOWN
        ivec2(0, -1),
        //UP
        ivec2(0, 1),
        //UP LEFT
        ivec2(-1, 1),
        //UP RIGHT
        ivec2(1, 1),
        //DOWN LEFT
        ivec2(-1, -1),
        //DOWN RIGHT
        ivec2(1, -1),
    ];

}

pub trait Grid<T> {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn values(&self) -> &[T];

    fn get_index(&self, pos: IVec2) -> usize {
        let x = pos.x as usize;
        let y = pos.y as usize;
        x + y * self.width()
    }

    fn try_get_index(&self, pos: IVec2) -> Option<usize> {
        if self.has_position(pos) {
            Some(self.get_index(pos))
        } else {
            None
        }
    }

    fn has_position(&self, pos: IVec2) -> bool {
        !(pos.x < 0 || pos.x >= self.width() as i32 || pos.y < 0 || pos.y >= self.height() as i32)
    }

    fn get_position(&self, pos: IVec2) -> &T {
        &self.values()[self.get_index(pos)]
    }

    fn try_get_position(&self, pos: IVec2) -> Option<&T> {
        if self.has_position(pos) {
            Some(self.get_position(pos))
        } else {
            None
        }
    }

    fn get_neighbours(&self, pos: IVec2, include_diagonal: bool) -> Vec<IVec2> {
        match include_diagonal {
            true => DIRECTIONS_8
                .into_iter()
                .map(|d| pos + d)
                .filter(|p| self.has_position(*p))
                .collect(),
            false => DIRECTIONS_4
                .into_iter()
                .map(|d| pos + d)
                .filter(|p| self.has_position(*p))
                .collect(),
        }
    }
}
