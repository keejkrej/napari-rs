use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SliceRange {
    pub start: usize,
    pub stop: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrackLabels {
    pub labels: Vec<String>,
    pub positions: Vec<Vec<f64>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrackManager {
    data: Vec<Vec<f64>>,
    order: Vec<usize>,
    ordered_points_idx: Vec<usize>,
    points: Vec<Vec<f64>>,
    points_id: Vec<u32>,
    points_lookup: BTreeMap<u64, SliceRange>,
    graph: BTreeMap<u32, Vec<u32>>,
    track_vertices: Option<Vec<Vec<f64>>>,
    track_connex: Option<Vec<bool>>,
    graph_vertices: Option<Vec<Vec<f64>>>,
    graph_connex: Option<Vec<bool>>,
    hide_completed_tracks: bool,
    current_time: Option<f64>,
}

impl TrackManager {
    pub fn new(data: Vec<Vec<f64>>) -> Result<Self, TrackManagerError> {
        let mut manager = Self {
            data: Vec::new(),
            order: Vec::new(),
            ordered_points_idx: Vec::new(),
            points: Vec::new(),
            points_id: Vec::new(),
            points_lookup: BTreeMap::new(),
            graph: BTreeMap::new(),
            track_vertices: None,
            track_connex: None,
            graph_vertices: None,
            graph_connex: None,
            hide_completed_tracks: false,
            current_time: None,
        };
        manager.set_data(data)?;
        Ok(manager)
    }

    pub fn fast_points_lookup(sorted_time: &[u64]) -> BTreeMap<u64, SliceRange> {
        if sorted_time.is_empty() {
            return BTreeMap::new();
        }

        let mut lookup = BTreeMap::new();
        let mut start = 0;
        for index in 1..=sorted_time.len() {
            if index == sorted_time.len() || sorted_time[index] != sorted_time[start] {
                lookup.insert(sorted_time[start], SliceRange { start, stop: index });
                start = index;
            }
        }
        lookup
    }

    pub fn set_data(&mut self, data: Vec<Vec<f64>>) -> Result<(), TrackManagerError> {
        validate_track_data(&data)?;

        let mut order: Vec<usize> = (0..data.len()).collect();
        order.sort_by(|&left, &right| {
            let left_row = &data[left];
            let right_row = &data[right];
            left_row[0]
                .total_cmp(&right_row[0])
                .then_with(|| left_row[1].total_cmp(&right_row[1]))
        });

        self.data = order.iter().map(|&index| data[index].clone()).collect();
        self.order = order;

        self.ordered_points_idx = (0..self.data.len()).collect();
        self.ordered_points_idx.sort_by(|&left, &right| {
            self.data[left][1]
                .total_cmp(&self.data[right][1])
                .then_with(|| self.data[left][0].total_cmp(&self.data[right][0]))
        });
        self.points = self
            .ordered_points_idx
            .iter()
            .map(|&index| self.data[index][1..].to_vec())
            .collect();
        self.points_id = self
            .ordered_points_idx
            .iter()
            .map(|&index| self.data[index][0] as u32)
            .collect();
        let time: Vec<u64> = self
            .points
            .iter()
            .map(|point| point[0].round().max(0.0) as u64)
            .collect();
        self.points_lookup = Self::fast_points_lookup(&time);

        self.track_vertices = None;
        self.track_connex = None;
        self.graph_vertices = None;
        self.graph_connex = None;
        self.graph.clear();
        self.build_tracks();
        Ok(())
    }

    pub fn data(&self) -> &[Vec<f64>] {
        &self.data
    }

    pub fn order(&self) -> &[usize] {
        &self.order
    }

    pub fn points(&self) -> &[Vec<f64>] {
        &self.points
    }

    pub fn points_id(&self) -> &[u32] {
        &self.points_id
    }

    pub fn points_lookup(&self) -> &BTreeMap<u64, SliceRange> {
        &self.points_lookup
    }

    pub fn graph(&self) -> &BTreeMap<u32, Vec<u32>> {
        &self.graph
    }

    pub fn ndim(&self) -> usize {
        self.data.first().map_or(0, |row| row.len() - 1)
    }

    pub fn track_ids(&self) -> Vec<u32> {
        self.data.iter().map(|row| row[0] as u32).collect()
    }

    pub fn unique_track_ids(&self) -> Vec<u32> {
        self.track_ids()
            .into_iter()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }

    pub fn len(&self) -> usize {
        self.unique_track_ids().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn set_graph(&mut self, graph: BTreeMap<u32, Vec<u32>>) -> Result<(), TrackManagerError> {
        self.graph = self.normalize_track_graph(graph)?;
        self.build_graph();
        Ok(())
    }

    pub fn set_hide_completed_tracks(&mut self, hide_completed_tracks: bool) {
        self.hide_completed_tracks = hide_completed_tracks;
    }

    pub fn hide_completed_tracks(&self) -> bool {
        self.hide_completed_tracks
    }

    pub fn set_current_time(&mut self, current_time: Option<f64>) {
        self.current_time = current_time;
    }

    pub fn current_time(&self) -> Option<f64> {
        self.current_time
    }

    pub fn build_tracks(&mut self) {
        if self.data.is_empty() {
            self.track_vertices = Some(Vec::new());
            self.track_connex = Some(Vec::new());
            return;
        }

        let track_vertices: Vec<Vec<f64>> = self.data.iter().map(|row| row[1..].to_vec()).collect();
        let mut track_connex = vec![true; self.data.len()];
        for (index, connected) in track_connex
            .iter_mut()
            .enumerate()
            .take(self.data.len() - 1)
        {
            if self.data[index][0] != self.data[index + 1][0] {
                *connected = false;
            }
        }
        if let Some(last) = track_connex.last_mut() {
            *last = false;
        }

        self.track_vertices = Some(track_vertices);
        self.track_connex = Some(track_connex);
    }

    pub fn build_graph(&mut self) {
        let mut graph_vertices = Vec::new();
        let mut graph_connex = Vec::new();

        for (&node_id, parent_ids) in &self.graph {
            let Some(&node_start) = self.vertex_indices_from_id(node_id).first() else {
                continue;
            };
            let node = self.data[node_start][1..].to_vec();

            for &parent_id in parent_ids {
                let Some(parent_stop) = self.vertex_indices_from_id(parent_id).last().copied()
                else {
                    continue;
                };
                let parent = self.data[parent_stop][1..].to_vec();
                graph_vertices.push(node.clone());
                graph_vertices.push(parent);
                graph_connex.push(true);
                graph_connex.push(false);
            }
        }

        if graph_vertices.is_empty() {
            self.graph_vertices = None;
            self.graph_connex = None;
        } else {
            self.graph_vertices = Some(graph_vertices);
            self.graph_connex = Some(graph_connex);
        }
    }

    pub fn track_vertices(&self) -> Option<&[Vec<f64>]> {
        self.track_vertices.as_deref()
    }

    pub fn track_connex(&self) -> Option<Vec<bool>> {
        let original = self.track_connex.as_ref()?;
        if !self.hide_completed_tracks || self.current_time.is_none() {
            return Some(original.clone());
        }

        let completed_mask = self.completed_tracks_mask();
        Some(
            original
                .iter()
                .zip(completed_mask)
                .map(|(&connected, completed)| connected && !completed)
                .collect(),
        )
    }

    pub fn graph_vertices(&self) -> Option<&[Vec<f64>]> {
        self.graph_vertices.as_deref()
    }

    pub fn graph_connex(&self) -> Option<&[bool]> {
        self.graph_connex.as_deref()
    }

    pub fn track_times(&self) -> Option<Vec<f64>> {
        self.track_vertices
            .as_ref()
            .map(|vertices| vertices.iter().map(|vertex| vertex[0]).collect())
    }

    pub fn graph_times(&self) -> Option<Vec<f64>> {
        self.graph_vertices
            .as_ref()
            .map(|vertices| vertices.iter().map(|vertex| vertex[0]).collect())
    }

    pub fn max_time(&self) -> Option<u64> {
        self.track_times().and_then(|times| {
            times
                .into_iter()
                .max_by(f64::total_cmp)
                .map(|time| time as u64)
        })
    }

    pub fn track_end_times(&self) -> Vec<f64> {
        self.unique_track_ids()
            .iter()
            .map(|&track_id| {
                self.vertex_indices_from_id(track_id)
                    .iter()
                    .map(|&index| self.data[index][1])
                    .max_by(f64::total_cmp)
                    .unwrap_or(0.0)
            })
            .collect()
    }

    pub fn completed_tracks_mask(&self) -> Vec<bool> {
        let Some(current_time) = self.current_time else {
            return vec![false; self.data.len()];
        };

        let unique_ids = self.unique_track_ids();
        let completed_ids: BTreeSet<u32> = unique_ids
            .into_iter()
            .zip(self.track_end_times())
            .filter_map(|(track_id, end_time)| (end_time < current_time).then_some(track_id))
            .collect();

        self.track_ids()
            .into_iter()
            .map(|track_id| completed_ids.contains(&track_id))
            .collect()
    }

    pub fn track_labels(&self, current_time: u64) -> TrackLabels {
        let Some(range) = self.points_lookup.get(&current_time) else {
            return TrackLabels {
                labels: Vec::new(),
                positions: Vec::new(),
            };
        };

        let labels = self.points_id[range.start..range.stop]
            .iter()
            .map(|track_id| format!("ID:{track_id}"))
            .collect();
        let positions = self.points[range.start..range.stop].to_vec();
        TrackLabels { labels, positions }
    }

    pub fn vertex_indices_from_id(&self, track_id: u32) -> Vec<usize> {
        self.data
            .iter()
            .enumerate()
            .filter_map(|(index, row)| ((row[0] as u32) == track_id).then_some(index))
            .collect()
    }

    fn normalize_track_graph(
        &self,
        graph: BTreeMap<u32, Vec<u32>>,
    ) -> Result<BTreeMap<u32, Vec<u32>>, TrackManagerError> {
        let unique_track_ids: BTreeSet<u32> = self.unique_track_ids().into_iter().collect();
        for (&node_id, parent_ids) in &graph {
            for node in std::iter::once(node_id).chain(parent_ids.iter().copied()) {
                if !unique_track_ids.contains(&node) {
                    return Err(TrackManagerError::GraphNodeNotFound(node_id));
                }
            }
        }
        Ok(graph)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TrackManagerError {
    InvalidDataShape,
    InvalidDimensionality(usize),
    NonIntegerTrackId,
    NegativeTimestamp,
    RaggedData,
    GraphNodeNotFound(u32),
}

impl fmt::Display for TrackManagerError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDataShape => formatter.write_str("track vertices should be a NxD array"),
            Self::InvalidDimensionality(dimensions) => write!(
                formatter,
                "track vertices should be 4 or 5-dimensional, got {dimensions}"
            ),
            Self::NonIntegerTrackId => formatter.write_str("track id must be an integer"),
            Self::NegativeTimestamp => {
                formatter.write_str("track timestamps must be greater than zero")
            }
            Self::RaggedData => formatter.write_str("track vertices must not be ragged"),
            Self::GraphNodeNotFound(node) => write!(formatter, "graph node {node} not found"),
        }
    }
}

impl Error for TrackManagerError {}

fn validate_track_data(data: &[Vec<f64>]) -> Result<(), TrackManagerError> {
    if data.is_empty() {
        return Err(TrackManagerError::InvalidDataShape);
    }
    let dimensions = data[0].len();
    if !(4..=5).contains(&dimensions) {
        return Err(TrackManagerError::InvalidDimensionality(dimensions));
    }
    if data.iter().any(|row| row.len() != dimensions) {
        return Err(TrackManagerError::RaggedData);
    }
    if data.iter().any(|row| row[0].floor() != row[0]) {
        return Err(TrackManagerError::NonIntegerTrackId);
    }
    if data.iter().any(|row| row[1] < 0.0) {
        return Err(TrackManagerError::NegativeTimestamp);
    }
    Ok(())
}
