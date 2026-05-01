#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridCanvas {
    pub stride: isize,
    pub shape: (isize, isize),
    pub enabled: bool,
    pub spacing: f64,
}

impl Default for GridCanvas {
    fn default() -> Self {
        Self {
            stride: 1,
            shape: (-1, -1),
            enabled: false,
            spacing: 0.0,
        }
    }
}

impl GridCanvas {
    pub fn actual_shape(&self, nlayers: usize) -> (usize, usize) {
        if !self.enabled || nlayers == 0 {
            return (1, 1);
        }

        let (mut n_row, mut n_column) = self.shape;
        let n_grid_squares = (nlayers as f64 / self.stride.unsigned_abs() as f64).ceil() as isize;

        if n_row == -1 && n_column == -1 {
            n_column = (n_grid_squares as f64).sqrt().ceil() as isize;
            n_row = (n_grid_squares as f64 / n_column as f64).ceil() as isize;
        } else if n_row == -1 {
            n_row = (n_grid_squares as f64 / n_column as f64).ceil() as isize;
        } else if n_column == -1 {
            n_column = (n_grid_squares as f64 / n_row as f64).ceil() as isize;
        }

        (n_row.max(1) as usize, n_column.max(1) as usize)
    }

    pub fn position(&self, index: usize, nlayers: usize) -> (usize, usize) {
        if !self.enabled {
            return (0, 0);
        }

        let (n_row, n_column) = self.actual_shape(nlayers);
        let adjusted_index = if self.stride < 0 {
            nlayers - index - 1
        } else {
            index
        };
        let adjusted_index = adjusted_index / self.stride.unsigned_abs();
        let adjusted_index = adjusted_index % (n_row * n_column);

        (adjusted_index / n_column, adjusted_index % n_column)
    }

    pub fn contents_at(&self, position: (usize, usize), nlayers: usize) -> Vec<usize> {
        (0..nlayers)
            .filter(|&index| self.position(index, nlayers) == position)
            .collect()
    }

    pub fn viewboxes(&self, nlayers: usize) -> Vec<((usize, usize), Vec<usize>)> {
        let (rows, cols) = self.actual_shape(nlayers);
        let mut viewboxes = Vec::with_capacity(rows * cols);
        for row in 0..rows {
            for col in 0..cols {
                let position = (row, col);
                viewboxes.push((position, self.contents_at(position, nlayers)));
            }
        }
        viewboxes
    }

    pub fn compute_canvas_spacing(&self, canvas_size: (usize, usize), nlayers: usize) -> usize {
        let (rows, cols) = self.actual_shape(nlayers);
        let (canvas_width, canvas_height) = canvas_size;
        let minimum_viewbox_size = 20_usize;

        let max_horizontal_spacing = if cols > 1 {
            (canvas_width as isize - (cols * minimum_viewbox_size) as isize) / (cols - 1) as isize
        } else {
            canvas_width as isize - (cols * minimum_viewbox_size) as isize
        };
        let max_vertical_spacing = if rows > 1 {
            (canvas_height as isize - (rows * minimum_viewbox_size) as isize) / (rows - 1) as isize
        } else {
            canvas_height as isize - (rows * minimum_viewbox_size) as isize
        };
        let safe_spacing = max_horizontal_spacing.min(max_vertical_spacing).max(0) as usize;

        self.compute_canvas_spacing_raw(canvas_size, nlayers)
            .min(safe_spacing)
    }

    pub fn compute_canvas_spacing_raw(&self, canvas_size: (usize, usize), nlayers: usize) -> usize {
        let (rows, cols) = self.actual_shape(nlayers);
        let (canvas_width, canvas_height) = canvas_size;

        if self.spacing >= 1.0 {
            self.spacing as usize
        } else {
            let unspaced_width = canvas_width as f64 / cols as f64;
            let unspaced_height = canvas_height as f64 / rows as f64;
            (self.spacing * ((unspaced_width + unspaced_height) / 2.0)) as usize
        }
    }
}
