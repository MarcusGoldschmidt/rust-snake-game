use std::collections::LinkedList;
use std::fmt;

pub enum WalkDirection {
    Up,
    Down,
    Left,
    Right,
}

pub struct Point {
    pub x: u16,
    pub y: u16,
}

impl Point {
    pub fn new(x: u16, y: u16) -> self::Point {
        self::Point { x, y }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

pub struct Snake {
    max_height: u16,
    max_width: u16,
    pub body: LinkedList<Point>,
}

impl fmt::Debug for Snake {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _head = self.body.front().unwrap();

        f.debug_struct("Snake")
            .field("head_x", &_head.x)
            .field("head_y", &_head.x)
            .finish()
    }
}

impl Snake {
    pub fn new(x: u16, y: u16, max_width: u16, max_height: u16) -> self::Snake {
        let mut body = LinkedList::new();

        body.push_front(self::Point::new(x, y));

        self::Snake {
            body,
            max_height,
            max_width,
        }
    }

    // FIXME: não deveria pérder se passar pela última parte se ela vai se mover tbm
    fn lose_game(&self, x: u16, y: u16) -> bool {
        let compare = self::Point::new(x, y);

        //skip fist
        self.body
            .iter()
            // Jump head, and the previous head
            .skip(2)
            .any(|data| *data == compare)
    }

    pub fn walk(&mut self, direction: WalkDirection, can_grow: bool) -> bool {
        let head = self.body.front().unwrap();

        let mut new_point = match direction {
            self::WalkDirection::Up => {
                if head.y == 0 {
                    (head.x, head.y)
                } else {
                    (head.x, head.y - 1)
                }
            }
            self::WalkDirection::Down => (head.x, head.y + 1),
            self::WalkDirection::Left => (head.x - 1, head.y),
            self::WalkDirection::Right => (head.x + 1, head.y),
        };

        new_point.0 = match new_point.0 {
            x if x == 0 => self.max_width,
            x if x > self.max_width => 0,
            x => x,
        };

        new_point.1 = match new_point.1 {
            x if x == 0 => self.max_height,
            x if x > self.max_height => 0,
            x => x,
        };

        let next = self::Point::new(new_point.0, new_point.1);

        if self.lose_game(next.x, next.y) {
            false
        } else {
            self.body.push_front(next);
            if !can_grow {
                self.body.pop_back();
            }
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walk_four_directions() {
        // Create snake
        let mut cobra = Snake::new(20, 20, 40, 40);

        // UP
        cobra.walk(WalkDirection::Up, true);
        let head = cobra.body.front().unwrap();
        assert_eq!(head, &Point::new(20, 19));

        // RIGHT
        cobra.walk(WalkDirection::Right, true);
        let head = cobra.body.front().unwrap();
        assert_eq!(head, &Point::new(21, 19));

        // DOWN
        cobra.walk(WalkDirection::Down, true);
        let head = cobra.body.front().unwrap();
        assert_eq!(head, &Point::new(21, 20));

        // DOWN
        cobra.walk(WalkDirection::Down, true);
        let head = cobra.body.front().unwrap();
        assert_eq!(head, &Point::new(21, 21));

        // LEFT
        cobra.walk(WalkDirection::Left, true);
        let head = cobra.body.front().unwrap();
        assert_eq!(head, &Point::new(20, 21));
    }

    #[test]
    fn grow_snake_body() {
        let mut cobra = Snake::new(20, 20, 40, 40);

        cobra.walk(WalkDirection::Up, true);
        cobra.walk(WalkDirection::Up, true);
        cobra.walk(WalkDirection::Up, true);
        cobra.walk(WalkDirection::Up, true);

        assert_eq!(cobra.body.len(), 5);
    }

    #[test]
    fn not_grow_snake_body() {
        let mut cobra = Snake::new(20, 20, 40, 40);

        cobra.walk(WalkDirection::Up, false);
        cobra.walk(WalkDirection::Up, false);
        cobra.walk(WalkDirection::Up, false);
        cobra.walk(WalkDirection::Up, false);

        assert_eq!(cobra.body.len(), 1);
    }

    #[test]
    fn should_not_lose_if_head_go_back() {
        // Create snake
        let mut cobra = Snake::new(20, 20, 40, 40);

        // UP
        cobra.walk(WalkDirection::Up, true);
        assert_eq!(cobra.walk(WalkDirection::Down, true), true);
    }
}
