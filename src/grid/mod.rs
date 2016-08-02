pub mod grid {
    #[derive(Copy, Clone)]
    struct Tile(bool);

    struct TileColumns {
        tiles: Vec<Tile>
    }

    struct TileSet {
        rows: Vec<TileColumns>
    }

    pub struct QRGrid {
        size: usize,
        grid: TileSet
    }

    impl TileColumns {
        fn new(size: usize) -> TileColumns {
            let mut tileset: Vec<Tile> = vec![];
            for _ in 0..size {
                tileset.push(Tile(false));
            }
            TileColumns { tiles: tileset }
        }
    }

    impl TileSet {
        fn new(size: usize) -> TileSet {
            let mut cols: Vec<TileColumns> = vec![];
            for _ in 0..size {
                let row = TileColumns::new(size);
                cols.push(row);
            }
            TileSet { rows: cols }
        }
    }

    impl QRGrid {
        fn new(size: usize) -> QRGrid {
            QRGrid { size: size, grid: TileSet::new(size) }
        }
    }

    pub fn create_grid(size: usize) -> QRGrid {
        QRGrid::new(size)
    }

    pub fn draw_fixed_point(qr: &mut QRGrid, start_point: &[u8], end_point: &[u8]) {
        // start_point: [3, 3], end_point: [0, 0]
        let tile_row = qr.grid.rows[start_point[0]];
        let mut start_point: Tile = tile_row.tiles[start_point[1]];
    }


    // impl QRSquare {
    //     fn new(size: usize) -> QRSquare {
    //         QRSquare { size: size, tiles: vec![Tile::new(); size] }
    //     }
    //
    //     fn get(&self, x: usize, y: usize) -> &Tile {
    //         &self.tiles[(x * self.size) + y]
    //     }
    //
    //     fn set(&mut self, x: usize, y: usize, is_bit: bool) {
    //         self.tiles[(x * self.size) + y] = Tile(is_bit);
    //     }
    // }
}
