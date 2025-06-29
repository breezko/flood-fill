use wasm_bindgen::prelude::*;
use js_sys::Uint32Array;
use std::collections::{VecDeque, HashSet};

/// Compute the smart flood fill sequence starting from a given cell.
///
/// This function performs a special flood fill that:
/// 1. Fills the starting region.
/// 2. Recursively "swallows" any adjacent regions smaller than itself.
///
/// Returns a flat Uint32Array of (x, y) pairs in the order they should be filled.
#[wasm_bindgen]
pub fn compute_smart_fill(
    grid: &[u8],
    width: usize,
    height: usize,
    start_x: usize,
    start_y: usize,
) -> Uint32Array {
    // If start coordinates are out of bounds, return empty
    if start_x >= width || start_y >= height {
        return Uint32Array::new_with_length(0);
    }

    let idx = start_y * width + start_x;
    let target_color = grid[idx];

    // If already filled with yellow, do nothing
    if target_color == 3 {
        return Uint32Array::new_with_length(0);
    }

    // Clone the grid so we don't modify the input
    let mut working_grid = grid.to_vec();

    // Flood fill the initial region
    let mut yellow_positions = flood_fill_with_positions(
        &mut working_grid,
        width,
        height,
        start_x,
        start_y,
        target_color,
        3,
    );
    let mut yellow_size = yellow_positions.len();

    // Recursively swallow adjacent smaller regions
    let mut changed = true;
    while changed {
        changed = false;

        let mut neighbor_regions = Vec::new();
        let mut visited: HashSet<(usize, usize)> = HashSet::new();

        // Look for neighboring regions
        for &(x, y) in &yellow_positions {
            for (nx, ny) in neighbors_4(x, y, width, height) {
                let n_idx = ny * width + nx;
                let n_color = working_grid[n_idx];

                // Skip already-filled or already-visited cells
                if n_color != 3 && !visited.contains(&(nx, ny)) {
                    // Peek to find the full region without changing the grid
                    let region = flood_fill_peek(
                        &working_grid,
                        width,
                        height,
                        nx,
                        ny,
                        n_color,
                    );
                    visited.extend(&region);
                    neighbor_regions.push((n_color, region));
                }
            }
        }

        // Decide which neighbors to swallow
        for (_color, region) in neighbor_regions {
            if region.len() < yellow_size {
                // Fill this region and grow the yellow area
                for &(x, y) in &region {
                    let idx = y * width + x;
                    working_grid[idx] = 3;
                    yellow_positions.push((x, y));
                }
                yellow_size += region.len();
                changed = true;
            }
        }
    }

    // Flatten positions into [x0, y0, x1, y1, ...]
    let mut flat = Vec::with_capacity(yellow_positions.len() * 2);
    for (x, y) in yellow_positions {
        flat.push(x as u32);
        flat.push(y as u32);
    }

    Uint32Array::from(flat.as_slice())
}

/// Standard flood fill that records all filled positions.
///
/// This modifies the grid in-place.
fn flood_fill_with_positions(
    grid: &mut [u8],
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    target: u8,
    replacement: u8,
) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back((x, y));

    while let Some((cx, cy)) = queue.pop_front() {
        let idx = cy * width + cx;
        if grid[idx] != target {
            continue;
        }

        grid[idx] = replacement;
        positions.push((cx, cy));

        for (nx, ny) in neighbors_4(cx, cy, width, height) {
            if grid[ny * width + nx] == target {
                queue.push_back((nx, ny));
            }
        }
    }
    positions
}

/// Peeks at all cells belonging to a contiguous region without modifying the grid.
fn flood_fill_peek(
    grid: &[u8],
    width: usize,
    height: usize,
    x: usize,
    y: usize,
    target: u8,
) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((x, y));
    visited.insert((x, y));

    while let Some((cx, cy)) = queue.pop_front() {
        let idx = cy * width + cx;
        if grid[idx] != target {
            continue;
        }

        positions.push((cx, cy));

        for (nx, ny) in neighbors_4(cx, cy, width, height) {
            if !visited.contains(&(nx, ny)) && grid[ny * width + nx] == target {
                queue.push_back((nx, ny));
                visited.insert((nx, ny));
            }
        }
    }
    positions
}

/// Returns the 4-connected neighbors of a cell (up, down, left, right).
fn neighbors_4(
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    if x > 0 {
        neighbors.push((x - 1, y));
    }
    if x + 1 < width {
        neighbors.push((x + 1, y));
    }
    if y > 0 {
        neighbors.push((x, y - 1));
    }
    if y + 1 < height {
        neighbors.push((x, y + 1));
    }
    neighbors
}
