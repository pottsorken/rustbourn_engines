use crate::common::PlayerGrid;

pub fn increment_grid_pos(grid: &mut PlayerGrid) {
    // increment grid pos
    grid.next_free_pos.0 += 1;
    if grid.next_free_pos == (0, 0) {
        grid.next_free_pos.0 += 1;
    }
    if grid.next_free_pos.0 > grid.grid_size.0 {
        grid.next_free_pos.0 = -grid.grid_size.0;
        grid.next_free_pos.1 -= 1;
    }

    //player.block_count += 1;
}
