pub type Rgba = [f32; 4];

pub fn normalize_and_broadcast_colors(num_entries: usize, colors: &[Rgba]) -> Vec<Rgba> {
    if colors.len() == num_entries || num_entries == 0 {
        return colors.to_vec();
    }
    if colors.len() != 1 {
        return vec![[1.0, 1.0, 1.0, 1.0]; num_entries];
    }
    vec![colors[0]; num_entries]
}
