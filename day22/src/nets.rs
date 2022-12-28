use crate::{BoardTile, Heading};

#[derive(Debug)]
pub enum Net {
    Net1_0,
    Net1_90,
    Net1_180,
    Net1_270,
    Net2_0,
    Net2_90,
    Net2_180,
    Net2_270,
    Net3_0,
    Net3_90,
    Net3_180,
    Net3_270,
}

impl Net {
    pub fn find_net(board: &Vec<BoardTile>, board_size: usize) -> Self {
        let grid:Vec<_> = (0..16).map(
                |i| {
                    if board[crate::to_index(&((i % 4) * board_size, (i / 4) * board_size), board_size)].is_void() {
                        0
                    } else {
                        1
                    }
                }
            )
            .collect();
        match grid.as_slice() {
            [0, 0, 1, 0,
             1, 1, 1, 1,
             0, 0, 1, 0,
             0, 0, 0, 0] => Net::Net1_0,
            [0, 1, 0, 0,
             1, 1, 1, 0,
             0, 1, 0, 0,
             0, 1, 0, 0] => Net::Net1_90,
            [0, 1, 0, 0,
             1, 1, 1, 1,
             0, 1, 0, 0,
             0, 0, 0, 0] => Net::Net1_180,
            [0, 1, 0, 0,
             0, 1, 0, 0,
             1, 1, 1, 0,
             0, 1, 0, 0] => Net::Net1_270,
            [0, 0, 1, 0,
             1, 1, 1, 0,
             0, 0, 1, 1,
             0, 0, 0, 0] => Net::Net2_0,
            [0, 0, 1, 0,
             1, 1, 1, 0,
             0, 1, 0, 0,
             0, 1, 0, 0] => Net::Net2_90,
            [1, 1, 0, 0,
             0, 1, 1, 1,
             0, 1, 0, 0,
             0, 0, 0, 0] => Net::Net2_180,
            [0, 1, 0, 0,
             0, 1, 0, 0,
             1, 1, 1, 0,
             1, 0, 0, 0] => Net::Net2_270,
            [0, 1, 1, 0,
             0, 1, 0, 0,
             1, 1, 0, 0,
             1, 0, 0, 0] => Net::Net3_0,
            [1, 0, 0, 0,
             1, 1, 1, 0,
             0, 0, 1, 1,
             0, 0, 0, 0] => Net::Net3_90,
            [0, 0, 1, 0,
             0, 1, 1, 0,
             0, 1, 0, 0,
             1, 1, 0, 0] => Net::Net3_180,
            [1, 1, 0, 0,
             0, 1, 1, 1,
             0, 0, 0, 1,
             0, 0, 0, 0] => Net::Net3_270,
            _ => panic!("Unresolved Cube net")
        }
    }

    fn edge_wrapping(
        (x, y): &(usize, usize),
        target_gridface: usize,
        exit_heading: &Heading,
        entry_heading: &Heading,
        board_size: usize
    ) -> ((usize, usize), Heading) {
        let npos = match (exit_heading, entry_heading) {
            (Heading::North, Heading::North) => (
                ((target_gridface % 4) * board_size) + (x % board_size),
                (((target_gridface / 4) + 1) * board_size) - 1
            ),
            (Heading::North, Heading::South) => (
                ((((target_gridface % 4) + 1) * board_size) - 1) - (x % board_size),
                (target_gridface / 4) * board_size
            ),
            (Heading::North, Heading::East) => (
                (target_gridface % 4) * board_size,
                ((target_gridface / 4) * board_size) + (x % board_size)
            ),
            (Heading::North, Heading::West) => (
                (((target_gridface % 4) + 1) * board_size) - 1,
                ((((target_gridface / 4) + 1) * board_size) - 1) - (x % board_size)
            ),
            (Heading::South, Heading::North) => (
                ((((target_gridface % 4) + 1) * board_size) - 1) - (x % board_size),
                (((target_gridface / 4) + 1) * board_size) - 1
            ),
            (Heading::South, Heading::South) => (
                ((target_gridface % 4) * board_size) + (x % board_size),
                (target_gridface / 4) * board_size
            ),
            (Heading::South, Heading::East) => (
                (target_gridface % 4) * board_size,
                ((((target_gridface / 4) + 1) * board_size) - 1) - (x % board_size)
            ),
            (Heading::South, Heading::West) => (
                (((target_gridface % 4) + 1) * board_size) - 1,
                ((target_gridface / 4) * board_size) + (x % board_size)
            ),
            (Heading::East, Heading::North) => (
                ((target_gridface % 4) * board_size) + (y % board_size),
                (((target_gridface / 4) + 1) * board_size) - 1
            ),
            (Heading::East, Heading::South) => (
                ((((target_gridface % 4) + 1) * board_size) - 1) - (y % board_size),
                (target_gridface / 4) * board_size
            ),
            (Heading::East, Heading::East) => (
                (target_gridface % 4) * board_size,
                ((target_gridface / 4) * board_size) + (y % board_size)
            ),
            (Heading::East, Heading::West) => (
                (((target_gridface % 4) + 1) * board_size) - 1,
                ((((target_gridface / 4) + 1) * board_size) - 1) - (y % board_size)
            ),
            (Heading::West, Heading::North) => (
                ((((target_gridface % 4) + 1) * board_size) - 1) - (y % board_size),
                (((target_gridface / 4) + 1) * board_size) - 1
            ),
            (Heading::West, Heading::South) => (
                ((target_gridface % 4) * board_size) + (y % board_size),
                (target_gridface / 4) * board_size
            ),
            (Heading::West, Heading::East) => (
                (target_gridface % 4) * board_size,
                ((((target_gridface / 4) + 1) * board_size) - 1) - (y % board_size)
            ),
            (Heading::West, Heading::West) => (
                (((target_gridface % 4) + 1) * board_size) - 1,
                ((target_gridface / 4) * board_size) + (y % board_size)
            ),
        };
        (npos, *entry_heading)
    }

    pub fn get_edge_wrappings(&self, exit_gridface: usize, exit_heading: &Heading) -> (usize, Heading) {
        match (self, exit_gridface, exit_heading) {
            // ..#.
            // ####
            // ..#.
            // ....
            (Net::Net1_0, 2, Heading::North) => (4, Heading::South),
            (Net::Net1_0, 2, Heading::South) => (6, Heading::South),
            (Net::Net1_0, 2, Heading::East) => (7, Heading::South),
            (Net::Net1_0, 2, Heading::West) => (5, Heading::South),
            (Net::Net1_0, 4, Heading::North) => (2, Heading::South),
            (Net::Net1_0, 4, Heading::South) => (10, Heading::North),
            (Net::Net1_0, 4, Heading::East) => (5, Heading::East),
            (Net::Net1_0, 4, Heading::West) => (7, Heading::West),
            (Net::Net1_0, 5, Heading::North) => (2, Heading::East),
            (Net::Net1_0, 5, Heading::South) => (10, Heading::East),
            (Net::Net1_0, 5, Heading::East) => (6, Heading::East),
            (Net::Net1_0, 5, Heading::West) => (4, Heading::West),
            (Net::Net1_0, 6, Heading::North) => (2, Heading::North),
            (Net::Net1_0, 6, Heading::South) => (10, Heading::South),
            (Net::Net1_0, 6, Heading::East) => (7, Heading::East),
            (Net::Net1_0, 6, Heading::West) => (5, Heading::West),
            (Net::Net1_0, 7, Heading::North) => (2, Heading::West),
            (Net::Net1_0, 7, Heading::South) => (10, Heading::West),
            (Net::Net1_0, 7, Heading::East) => (4, Heading::East),
            (Net::Net1_0, 7, Heading::West) => (6, Heading::West),
            (Net::Net1_0, 10, Heading::North) => (6, Heading::North),
            (Net::Net1_0, 10, Heading::South) => (4, Heading::North),
            (Net::Net1_0, 10, Heading::East) => (7, Heading::North),
            (Net::Net1_0, 10, Heading::West) => (5, Heading::North),
            (Net::Net1_90, _, _) => todo!(),
            (Net::Net1_180, _, _) => todo!(),
            (Net::Net1_270, _, _) => todo!(),
            // ..#.
            // ###.
            // ..##
            // ....
            (Net::Net2_0, 2, Heading::North) => (4, Heading::South),
            (Net::Net2_0, 2, Heading::South) => (6, Heading::South),
            (Net::Net2_0, 2, Heading::East) => (11, Heading::West),
            (Net::Net2_0, 2, Heading::West) => (5, Heading::South),
            (Net::Net2_0, 4, Heading::North) => (2, Heading::South),
            (Net::Net2_0, 4, Heading::South) => (10, Heading::North),
            (Net::Net2_0, 4, Heading::East) => (5, Heading::East),
            (Net::Net2_0, 4, Heading::West) => (7, Heading::North),
            (Net::Net2_0, 5, Heading::North) => (2, Heading::East),
            (Net::Net2_0, 5, Heading::South) => (10, Heading::East),
            (Net::Net2_0, 5, Heading::East) => (6, Heading::East),
            (Net::Net2_0, 5, Heading::West) => (4, Heading::West),
            (Net::Net2_0, 6, Heading::North) => (2, Heading::North),
            (Net::Net2_0, 6, Heading::South) => (10, Heading::South),
            (Net::Net2_0, 6, Heading::East) => (11, Heading::South),
            (Net::Net2_0, 6, Heading::West) => (5, Heading::West),
            (Net::Net2_0, 10, Heading::North) => (6, Heading::North),
            (Net::Net2_0, 10, Heading::South) => (4, Heading::North),
            (Net::Net2_0, 10, Heading::East) => (11, Heading::East),
            (Net::Net2_0, 10, Heading::West) => (5, Heading::North),
            (Net::Net2_0, 11, Heading::North) => (6, Heading::West),
            (Net::Net2_0, 11, Heading::South) => (4, Heading::East),
            (Net::Net2_0, 11, Heading::East) => (2, Heading::West),
            (Net::Net2_0, 11, Heading::West) => (10, Heading::West),
            // .##.
            // .#..
            // ##..
            // #...
            (Net::Net3_0, 1, Heading::North) => (12, Heading::East),
            (Net::Net3_0, 1, Heading::South) => (5, Heading::South),
            (Net::Net3_0, 1, Heading::East) => (2, Heading::East),
            (Net::Net3_0, 1, Heading::West) => (8, Heading::East),
            (Net::Net3_0, 2, Heading::North) => (12, Heading::North),
            (Net::Net3_0, 2, Heading::South) => (5, Heading::West),
            (Net::Net3_0, 2, Heading::East) => (9, Heading::West),
            (Net::Net3_0, 2, Heading::West) => (1, Heading::West),
            (Net::Net3_0, 5, Heading::North) => (1, Heading::North),
            (Net::Net3_0, 5, Heading::South) => (9, Heading::South),
            (Net::Net3_0, 5, Heading::East) => (2, Heading::North),
            (Net::Net3_0, 5, Heading::West) => (8, Heading::South),
            (Net::Net3_0, 8, Heading::North) => (5, Heading::East),
            (Net::Net3_0, 8, Heading::South) => (12, Heading::South),
            (Net::Net3_0, 8, Heading::East) => (9, Heading::East),
            (Net::Net3_0, 8, Heading::West) => (1, Heading::East),
            (Net::Net3_0, 9, Heading::North) => (5, Heading::North),
            (Net::Net3_0, 9, Heading::South) => (12, Heading::West),
            (Net::Net3_0, 9, Heading::East) => (2, Heading::West),
            (Net::Net3_0, 9, Heading::West) => (8, Heading::West),
            (Net::Net3_0, 12, Heading::North) => (8, Heading::North),
            (Net::Net3_0, 12, Heading::South) => (2, Heading::South),
            (Net::Net3_0, 12, Heading::East) => (9, Heading::North),
            (Net::Net3_0, 12, Heading::West) => (1, Heading::South),
            _ => panic!("Net '{:?}' does not have face '{}'", self, exit_gridface)
        }
    }

    pub fn move_to_adj_face(
        &self,
        pos: &(usize, usize),
        heading: &Heading,
        board_size: usize
    ) -> ((usize, usize), Heading) {
        let (x, y) = pos;
        let gridface = (x / board_size) + ((y / board_size) * 4);
        let up_edge = y % board_size == 0 && matches!(heading, Heading::North);
        let down_edge = y % board_size == (board_size - 1) && matches!(heading, Heading::South);
        let right_edge = x % board_size == (board_size - 1) && matches!(heading, Heading::East);
        let left_edge = x % board_size == 0 && matches!(heading, Heading::West);
        if up_edge || down_edge || right_edge || left_edge {
            let (entry_gridface, entry_heading) = self.get_edge_wrappings(gridface, heading);
            Self::edge_wrapping(pos, entry_gridface, heading, &entry_heading, board_size)
        } else {
            panic!("Not in an face edge or facing into edge.")
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn norht_moves() {
        let exit_heading = Heading::North;
        assert_eq!(Net::edge_wrapping(&(0, 0), 1, &exit_heading, &Heading::North, 4).0, (4, 3));
        assert_eq!(Net::edge_wrapping(&(1, 0), 1, &exit_heading, &Heading::North, 4).0, (5, 3));
        assert_eq!(Net::edge_wrapping(&(2, 0), 1, &exit_heading, &Heading::North, 4).0, (6, 3));
        assert_eq!(Net::edge_wrapping(&(3, 0), 1, &exit_heading, &Heading::North, 4).0, (7, 3));
        assert_eq!(Net::edge_wrapping(&(0, 0), 15, &exit_heading, &Heading::North, 4).0, (12, 15));
        assert_eq!(Net::edge_wrapping(&(1, 0), 15, &exit_heading, &Heading::North, 4).0, (13, 15));
        assert_eq!(Net::edge_wrapping(&(2, 0), 15, &exit_heading, &Heading::North, 4).0, (14, 15));
        assert_eq!(Net::edge_wrapping(&(3, 0), 15, &exit_heading, &Heading::North, 4).0, (15, 15));
        assert_eq!(Net::edge_wrapping(&(0, 0), 1, &exit_heading, &Heading::South, 4).0, (7, 0));
        assert_eq!(Net::edge_wrapping(&(1, 0), 1, &exit_heading, &Heading::South, 4).0, (6, 0));
        assert_eq!(Net::edge_wrapping(&(2, 0), 1, &exit_heading, &Heading::South, 4).0, (5, 0));
        assert_eq!(Net::edge_wrapping(&(3, 0), 1, &exit_heading, &Heading::South, 4).0, (4, 0));
        assert_eq!(Net::edge_wrapping(&(0, 0), 15, &exit_heading, &Heading::South, 4).0, (15, 12));
        assert_eq!(Net::edge_wrapping(&(1, 0), 15, &exit_heading, &Heading::South, 4).0, (14, 12));
        assert_eq!(Net::edge_wrapping(&(2, 0), 15, &exit_heading, &Heading::South, 4).0, (13, 12));
        assert_eq!(Net::edge_wrapping(&(3, 0), 15, &exit_heading, &Heading::South, 4).0, (12, 12));
        assert_eq!(Net::edge_wrapping(&(0, 0), 1, &exit_heading, &Heading::East, 4).0, (4, 0));
        assert_eq!(Net::edge_wrapping(&(1, 0), 1, &exit_heading, &Heading::East, 4).0, (4, 1));
        assert_eq!(Net::edge_wrapping(&(2, 0), 1, &exit_heading, &Heading::East, 4).0, (4, 2));
        assert_eq!(Net::edge_wrapping(&(3, 0), 1, &exit_heading, &Heading::East, 4).0, (4, 3));
        assert_eq!(Net::edge_wrapping(&(0, 0), 15, &exit_heading, &Heading::East, 4).0, (12, 12));
        assert_eq!(Net::edge_wrapping(&(1, 0), 15, &exit_heading, &Heading::East, 4).0, (12, 13));
        assert_eq!(Net::edge_wrapping(&(2, 0), 15, &exit_heading, &Heading::East, 4).0, (12, 14));
        assert_eq!(Net::edge_wrapping(&(3, 0), 15, &exit_heading, &Heading::East, 4).0, (12, 15));
        assert_eq!(Net::edge_wrapping(&(0, 0), 1, &exit_heading, &Heading::West, 4).0, (7, 3));
        assert_eq!(Net::edge_wrapping(&(1, 0), 1, &exit_heading, &Heading::West, 4).0, (7, 2));
        assert_eq!(Net::edge_wrapping(&(2, 0), 1, &exit_heading, &Heading::West, 4).0, (7, 1));
        assert_eq!(Net::edge_wrapping(&(3, 0), 1, &exit_heading, &Heading::West, 4).0, (7, 0));
        assert_eq!(Net::edge_wrapping(&(0, 0), 15, &exit_heading, &Heading::West, 4).0, (15, 15));
        assert_eq!(Net::edge_wrapping(&(1, 0), 15, &exit_heading, &Heading::West, 4).0, (15, 14));
        assert_eq!(Net::edge_wrapping(&(2, 0), 15, &exit_heading, &Heading::West, 4).0, (15, 13));
        assert_eq!(Net::edge_wrapping(&(3, 0), 15, &exit_heading, &Heading::West, 4).0, (15, 12));
    }

    #[test]
    fn south_moves() {
        let exit_heading = Heading::South;
        assert_eq!(Net::edge_wrapping(&(0, 3), 1, &exit_heading, &Heading::North, 4).0, (7, 3));
        assert_eq!(Net::edge_wrapping(&(1, 3), 1, &exit_heading, &Heading::North, 4).0, (6, 3));
        assert_eq!(Net::edge_wrapping(&(2, 3), 1, &exit_heading, &Heading::North, 4).0, (5, 3));
        assert_eq!(Net::edge_wrapping(&(3, 3), 1, &exit_heading, &Heading::North, 4).0, (4, 3));
        assert_eq!(Net::edge_wrapping(&(0, 3), 15, &exit_heading, &Heading::North, 4).0, (15, 15));
        assert_eq!(Net::edge_wrapping(&(1, 3), 15, &exit_heading, &Heading::North, 4).0, (14, 15));
        assert_eq!(Net::edge_wrapping(&(2, 3), 15, &exit_heading, &Heading::North, 4).0, (13, 15));
        assert_eq!(Net::edge_wrapping(&(3, 3), 15, &exit_heading, &Heading::North, 4).0, (12, 15));
        assert_eq!(Net::edge_wrapping(&(0, 3), 1, &exit_heading, &Heading::South, 4).0, (4, 0));
        assert_eq!(Net::edge_wrapping(&(1, 3), 1, &exit_heading, &Heading::South, 4).0, (5, 0));
        assert_eq!(Net::edge_wrapping(&(2, 3), 1, &exit_heading, &Heading::South, 4).0, (6, 0));
        assert_eq!(Net::edge_wrapping(&(3, 3), 1, &exit_heading, &Heading::South, 4).0, (7, 0));
        assert_eq!(Net::edge_wrapping(&(0, 3), 15, &exit_heading, &Heading::South, 4).0, (12, 12));
        assert_eq!(Net::edge_wrapping(&(1, 3), 15, &exit_heading, &Heading::South, 4).0, (13, 12));
        assert_eq!(Net::edge_wrapping(&(2, 3), 15, &exit_heading, &Heading::South, 4).0, (14, 12));
        assert_eq!(Net::edge_wrapping(&(3, 3), 15, &exit_heading, &Heading::South, 4).0, (15, 12));
        assert_eq!(Net::edge_wrapping(&(0, 3), 1, &exit_heading, &Heading::East, 4).0, (4, 3));
        assert_eq!(Net::edge_wrapping(&(1, 3), 1, &exit_heading, &Heading::East, 4).0, (4, 2));
        assert_eq!(Net::edge_wrapping(&(2, 3), 1, &exit_heading, &Heading::East, 4).0, (4, 1));
        assert_eq!(Net::edge_wrapping(&(3, 3), 1, &exit_heading, &Heading::East, 4).0, (4, 0));
        assert_eq!(Net::edge_wrapping(&(0, 3), 15, &exit_heading, &Heading::East, 4).0, (12, 15));
        assert_eq!(Net::edge_wrapping(&(1, 3), 15, &exit_heading, &Heading::East, 4).0, (12, 14));
        assert_eq!(Net::edge_wrapping(&(2, 3), 15, &exit_heading, &Heading::East, 4).0, (12, 13));
        assert_eq!(Net::edge_wrapping(&(3, 3), 15, &exit_heading, &Heading::East, 4).0, (12, 12));
        assert_eq!(Net::edge_wrapping(&(0, 3), 1, &exit_heading, &Heading::West, 4).0, (7, 0));
        assert_eq!(Net::edge_wrapping(&(1, 3), 1, &exit_heading, &Heading::West, 4).0, (7, 1));
        assert_eq!(Net::edge_wrapping(&(2, 3), 1, &exit_heading, &Heading::West, 4).0, (7, 2));
        assert_eq!(Net::edge_wrapping(&(3, 3), 1, &exit_heading, &Heading::West, 4).0, (7, 3));
        assert_eq!(Net::edge_wrapping(&(0, 3), 15, &exit_heading, &Heading::West, 4).0, (15, 12));
        assert_eq!(Net::edge_wrapping(&(1, 3), 15, &exit_heading, &Heading::West, 4).0, (15, 13));
        assert_eq!(Net::edge_wrapping(&(2, 3), 15, &exit_heading, &Heading::West, 4).0, (15, 14));
        assert_eq!(Net::edge_wrapping(&(3, 3), 15, &exit_heading, &Heading::West, 4).0, (15, 15));
    }

    #[test]
    fn east_moves() {
        let exit_heading = Heading::East;
        assert_eq!(Net::edge_wrapping(&(3, 0), 1, &exit_heading, &Heading::North, 4).0, (4, 3));
        assert_eq!(Net::edge_wrapping(&(3, 1), 1, &exit_heading, &Heading::North, 4).0, (5, 3));
        assert_eq!(Net::edge_wrapping(&(3, 2), 1, &exit_heading, &Heading::North, 4).0, (6, 3));
        assert_eq!(Net::edge_wrapping(&(3, 3), 1, &exit_heading, &Heading::North, 4).0, (7, 3));
        assert_eq!(Net::edge_wrapping(&(3, 0), 15, &exit_heading, &Heading::North, 4).0, (12, 15));
        assert_eq!(Net::edge_wrapping(&(3, 1), 15, &exit_heading, &Heading::North, 4).0, (13, 15));
        assert_eq!(Net::edge_wrapping(&(3, 2), 15, &exit_heading, &Heading::North, 4).0, (14, 15));
        assert_eq!(Net::edge_wrapping(&(3, 3), 15, &exit_heading, &Heading::North, 4).0, (15, 15));
        assert_eq!(Net::edge_wrapping(&(3, 0), 1, &exit_heading, &Heading::South, 4).0, (7, 0));
        assert_eq!(Net::edge_wrapping(&(3, 1), 1, &exit_heading, &Heading::South, 4).0, (6, 0));
        assert_eq!(Net::edge_wrapping(&(3, 2), 1, &exit_heading, &Heading::South, 4).0, (5, 0));
        assert_eq!(Net::edge_wrapping(&(3, 3), 1, &exit_heading, &Heading::South, 4).0, (4, 0));
        assert_eq!(Net::edge_wrapping(&(3, 0), 15, &exit_heading, &Heading::South, 4).0, (15, 12));
        assert_eq!(Net::edge_wrapping(&(3, 1), 15, &exit_heading, &Heading::South, 4).0, (14, 12));
        assert_eq!(Net::edge_wrapping(&(3, 2), 15, &exit_heading, &Heading::South, 4).0, (13, 12));
        assert_eq!(Net::edge_wrapping(&(3, 3), 15, &exit_heading, &Heading::South, 4).0, (12, 12));
        assert_eq!(Net::edge_wrapping(&(3, 0), 1, &exit_heading, &Heading::East, 4).0, (4, 0));
        assert_eq!(Net::edge_wrapping(&(3, 1), 1, &exit_heading, &Heading::East, 4).0, (4, 1));
        assert_eq!(Net::edge_wrapping(&(3, 2), 1, &exit_heading, &Heading::East, 4).0, (4, 2));
        assert_eq!(Net::edge_wrapping(&(3, 3), 1, &exit_heading, &Heading::East, 4).0, (4, 3));
        assert_eq!(Net::edge_wrapping(&(3, 0), 15, &exit_heading, &Heading::East, 4).0, (12, 12));
        assert_eq!(Net::edge_wrapping(&(3, 1), 15, &exit_heading, &Heading::East, 4).0, (12, 13));
        assert_eq!(Net::edge_wrapping(&(3, 2), 15, &exit_heading, &Heading::East, 4).0, (12, 14));
        assert_eq!(Net::edge_wrapping(&(3, 3), 15, &exit_heading, &Heading::East, 4).0, (12, 15));
        assert_eq!(Net::edge_wrapping(&(3, 0), 1, &exit_heading, &Heading::West, 4).0, (7, 3));
        assert_eq!(Net::edge_wrapping(&(3, 1), 1, &exit_heading, &Heading::West, 4).0, (7, 2));
        assert_eq!(Net::edge_wrapping(&(3, 2), 1, &exit_heading, &Heading::West, 4).0, (7, 1));
        assert_eq!(Net::edge_wrapping(&(3, 3), 1, &exit_heading, &Heading::West, 4).0, (7, 0));
        assert_eq!(Net::edge_wrapping(&(3, 0), 15, &exit_heading, &Heading::West, 4).0, (15, 15));
        assert_eq!(Net::edge_wrapping(&(3, 1), 15, &exit_heading, &Heading::West, 4).0, (15, 14));
        assert_eq!(Net::edge_wrapping(&(3, 2), 15, &exit_heading, &Heading::West, 4).0, (15, 13));
        assert_eq!(Net::edge_wrapping(&(3, 3), 15, &exit_heading, &Heading::West, 4).0, (15, 12));
    }

    #[test]
    fn west_moves() {
        let exit_heading = Heading::West;
        assert_eq!(Net::edge_wrapping(&(0, 0), 1, &exit_heading, &Heading::North, 4).0, (7, 3));
        assert_eq!(Net::edge_wrapping(&(0, 1), 1, &exit_heading, &Heading::North, 4).0, (6, 3));
        assert_eq!(Net::edge_wrapping(&(0, 2), 1, &exit_heading, &Heading::North, 4).0, (5, 3));
        assert_eq!(Net::edge_wrapping(&(0, 3), 1, &exit_heading, &Heading::North, 4).0, (4, 3));
        assert_eq!(Net::edge_wrapping(&(0, 0), 15, &exit_heading, &Heading::North, 4).0, (15, 15));
        assert_eq!(Net::edge_wrapping(&(0, 1), 15, &exit_heading, &Heading::North, 4).0, (14, 15));
        assert_eq!(Net::edge_wrapping(&(0, 2), 15, &exit_heading, &Heading::North, 4).0, (13, 15));
        assert_eq!(Net::edge_wrapping(&(0, 3), 15, &exit_heading, &Heading::North, 4).0, (12, 15));
        assert_eq!(Net::edge_wrapping(&(0, 0), 1, &exit_heading, &Heading::South, 4).0, (4, 0));
        assert_eq!(Net::edge_wrapping(&(0, 1), 1, &exit_heading, &Heading::South, 4).0, (5, 0));
        assert_eq!(Net::edge_wrapping(&(0, 2), 1, &exit_heading, &Heading::South, 4).0, (6, 0));
        assert_eq!(Net::edge_wrapping(&(0, 3), 1, &exit_heading, &Heading::South, 4).0, (7, 0));
        assert_eq!(Net::edge_wrapping(&(0, 0), 15, &exit_heading, &Heading::South, 4).0, (12, 12));
        assert_eq!(Net::edge_wrapping(&(0, 1), 15, &exit_heading, &Heading::South, 4).0, (13, 12));
        assert_eq!(Net::edge_wrapping(&(0, 2), 15, &exit_heading, &Heading::South, 4).0, (14, 12));
        assert_eq!(Net::edge_wrapping(&(0, 3), 15, &exit_heading, &Heading::South, 4).0, (15, 12));
        assert_eq!(Net::edge_wrapping(&(0, 0), 1, &exit_heading, &Heading::East, 4).0, (4, 3));
        assert_eq!(Net::edge_wrapping(&(0, 1), 1, &exit_heading, &Heading::East, 4).0, (4, 2));
        assert_eq!(Net::edge_wrapping(&(0, 2), 1, &exit_heading, &Heading::East, 4).0, (4, 1));
        assert_eq!(Net::edge_wrapping(&(0, 3), 1, &exit_heading, &Heading::East, 4).0, (4, 0));
        assert_eq!(Net::edge_wrapping(&(0, 0), 15, &exit_heading, &Heading::East, 4).0, (12, 15));
        assert_eq!(Net::edge_wrapping(&(0, 1), 15, &exit_heading, &Heading::East, 4).0, (12, 14));
        assert_eq!(Net::edge_wrapping(&(0, 2), 15, &exit_heading, &Heading::East, 4).0, (12, 13));
        assert_eq!(Net::edge_wrapping(&(0, 3), 15, &exit_heading, &Heading::East, 4).0, (12, 12));
        assert_eq!(Net::edge_wrapping(&(0, 0), 1, &exit_heading, &Heading::West, 4).0, (7, 0));
        assert_eq!(Net::edge_wrapping(&(0, 1), 1, &exit_heading, &Heading::West, 4).0, (7, 1));
        assert_eq!(Net::edge_wrapping(&(0, 2), 1, &exit_heading, &Heading::West, 4).0, (7, 2));
        assert_eq!(Net::edge_wrapping(&(0, 3), 1, &exit_heading, &Heading::West, 4).0, (7, 3));
        assert_eq!(Net::edge_wrapping(&(0, 0), 15, &exit_heading, &Heading::West, 4).0, (15, 12));
        assert_eq!(Net::edge_wrapping(&(0, 1), 15, &exit_heading, &Heading::West, 4).0, (15, 13));
        assert_eq!(Net::edge_wrapping(&(0, 2), 15, &exit_heading, &Heading::West, 4).0, (15, 14));
        assert_eq!(Net::edge_wrapping(&(0, 3), 15, &exit_heading, &Heading::West, 4).0, (15, 15));
    }
}