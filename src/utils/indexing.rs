use std::collections::BTreeMap;

pub type FancyIndex = Vec<Vec<isize>>;

pub fn elements_in_slice(
    index: &[Vec<isize>],
    position_in_axes: &BTreeMap<usize, isize>,
) -> Vec<bool> {
    let len = index.first().map_or(0, Vec::len);
    let mut visible = vec![true; len];

    for (&axis, &position) in position_in_axes {
        let axis_index = &index[axis];
        for (item_visible, &value) in visible.iter_mut().zip(axis_index) {
            *item_visible &= value == position;
        }
    }

    visible
}

pub fn index_in_slice(
    index: &[Vec<isize>],
    position_in_axes: &BTreeMap<usize, isize>,
    indices_order: &[usize],
) -> FancyIndex {
    let visible = elements_in_slice(index, position_in_axes);

    indices_order
        .iter()
        .filter(|axis| !position_in_axes.contains_key(axis))
        .map(|&axis| {
            index[axis]
                .iter()
                .zip(&visible)
                .filter_map(|(&value, &is_visible)| is_visible.then_some(value))
                .collect()
        })
        .collect()
}
