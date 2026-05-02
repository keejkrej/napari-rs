use std::collections::BTreeMap;

use napari_rs::layers::tracks::track_utils::{SliceRange, TrackManager, TrackManagerError};

#[test]
fn fast_points_lookup_matches_python_slice_boundaries() {
    let time_points = [0, 1, 3, 5, 10];
    let repeats = [3, 4, 6, 3, 5];
    let mut sorted_time = Vec::new();
    for (&time, &repeat) in time_points.iter().zip(&repeats) {
        sorted_time.extend(std::iter::repeat_n(time, repeat));
    }

    let lookup = TrackManager::fast_points_lookup(&sorted_time);

    assert_eq!(lookup.len(), time_points.len());
    let mut start = 0;
    for (&time, &repeat) in time_points.iter().zip(&repeats) {
        let stop = start + repeat;
        assert_eq!(lookup.get(&time), Some(&SliceRange { start, stop }));
        assert!(sorted_time[start..stop].iter().all(|&value| value == time));
        start = stop;
    }
    assert_eq!(start, sorted_time.len());
}

#[test]
fn track_manager_validates_and_sorts_data_by_id_then_time_like_python() {
    let data = vec![
        vec![1.0, 1.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0, 0.0],
        vec![2.0, 0.0, 0.0, 0.0],
        vec![0.0, 0.0, 0.0, 0.0],
        vec![1.0, 0.0, 0.0, 0.0],
    ];

    let manager = TrackManager::new(data).unwrap();

    assert_eq!(manager.order(), &[3, 1, 4, 0, 2]);
    assert_eq!(
        manager.data(),
        &[
            vec![0.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![1.0, 0.0, 0.0, 0.0],
            vec![1.0, 1.0, 0.0, 0.0],
            vec![2.0, 0.0, 0.0, 0.0],
        ]
    );
    assert_eq!(manager.track_ids(), vec![0, 0, 1, 1, 2]);
    assert_eq!(manager.unique_track_ids(), vec![0, 1, 2]);
    assert_eq!(manager.len(), 3);
    assert_eq!(manager.ndim(), 3);
}

#[test]
fn track_manager_rejects_malformed_data_like_python() {
    assert_eq!(
        TrackManager::new(vec![vec![0.0, 0.0, 0.0]]).unwrap_err(),
        TrackManagerError::InvalidDimensionality(3)
    );
    assert_eq!(
        TrackManager::new(vec![vec![0.5, 0.0, 0.0, 0.0]]).unwrap_err(),
        TrackManagerError::NonIntegerTrackId
    );
    assert_eq!(
        TrackManager::new(vec![vec![0.0, -1.0, 0.0, 0.0]]).unwrap_err(),
        TrackManagerError::NegativeTimestamp
    );
    assert_eq!(
        TrackManager::new(vec![vec![0.0, 0.0, 0.0, 0.0], vec![1.0, 0.0, 0.0]]).unwrap_err(),
        TrackManagerError::RaggedData
    );
}

#[test]
fn build_tracks_creates_vertices_and_connex_breaks_at_track_boundaries() {
    let mut data = Vec::new();
    for track_id in 1..=5 {
        data.push(vec![track_id as f64, 0.0, track_id as f64, 0.0]);
        data.push(vec![track_id as f64, 1.0, track_id as f64, 1.0]);
    }
    data.push(vec![6.0, 0.0, 6.0, 0.0]);

    let manager = TrackManager::new(data).unwrap();

    assert_eq!(manager.track_vertices().unwrap().len(), 11);
    assert_eq!(manager.track_vertices().unwrap()[0], vec![0.0, 1.0, 0.0]);
    let connex = manager.track_connex().unwrap();
    assert_eq!(connex.iter().filter(|&&value| !value).count(), 6);
    assert_eq!(
        connex,
        vec![
            true, false, true, false, true, false, true, false, true, false, false
        ]
    );
}

#[test]
fn graph_normalization_and_arrays_match_python_manager() {
    let data = vec![
        vec![0.0, 0.0, 10.0, 10.0],
        vec![0.0, 1.0, 11.0, 11.0],
        vec![1.0, 2.0, 20.0, 20.0],
        vec![1.0, 3.0, 21.0, 21.0],
        vec![2.0, 4.0, 30.0, 30.0],
    ];
    let mut manager = TrackManager::new(data).unwrap();

    manager
        .set_graph(BTreeMap::from([(1, vec![0]), (2, vec![1])]))
        .unwrap();

    assert_eq!(
        manager.graph(),
        &BTreeMap::from([(1, vec![0]), (2, vec![1])])
    );
    assert_eq!(
        manager.graph_vertices(),
        Some(
            [
                vec![2.0, 20.0, 20.0],
                vec![1.0, 11.0, 11.0],
                vec![4.0, 30.0, 30.0],
                vec![3.0, 21.0, 21.0],
            ]
            .as_slice()
        )
    );
    assert_eq!(
        manager.graph_connex(),
        Some([true, false, true, false].as_slice())
    );

    assert_eq!(
        manager.set_graph(BTreeMap::from([(2, vec![33])])),
        Err(TrackManagerError::GraphNodeNotFound(2))
    );
}

#[test]
fn completed_track_masking_matches_python_hide_completed_tracks_behavior() {
    let data = vec![
        vec![0.0, 0.0, 10.0, 10.0],
        vec![0.0, 1.0, 11.0, 11.0],
        vec![0.0, 2.0, 12.0, 12.0],
        vec![1.0, 3.0, 20.0, 20.0],
        vec![1.0, 4.0, 21.0, 21.0],
        vec![1.0, 5.0, 22.0, 22.0],
        vec![2.0, 0.0, 30.0, 30.0],
        vec![2.0, 1.0, 31.0, 31.0],
        vec![2.0, 2.0, 32.0, 32.0],
        vec![2.0, 3.0, 33.0, 33.0],
        vec![2.0, 4.0, 34.0, 34.0],
        vec![2.0, 5.0, 35.0, 35.0],
        vec![2.0, 6.0, 36.0, 36.0],
        vec![2.0, 7.0, 37.0, 37.0],
    ];
    let mut manager = TrackManager::new(data).unwrap();
    let original_connex = manager.track_connex().unwrap();

    assert!(!manager.hide_completed_tracks());
    manager.set_hide_completed_tracks(true);
    manager.set_current_time(Some(4.0));

    assert_eq!(
        manager.completed_tracks_mask(),
        vec![
            true, true, true, false, false, false, false, false, false, false, false, false, false,
            false
        ]
    );
    let masked_connex = manager.track_connex().unwrap();
    assert!(masked_connex[0..3].iter().all(|&value| !value));
    assert_eq!(&masked_connex[3..], &original_connex[3..]);

    manager.set_current_time(Some(6.0));
    let masked_connex = manager.track_connex().unwrap();
    assert!(masked_connex[0..6].iter().all(|&value| !value));
    assert_eq!(&masked_connex[6..], &original_connex[6..]);

    manager.set_current_time(Some(8.0));
    assert!(manager.track_connex().unwrap().iter().all(|&value| !value));

    manager.set_hide_completed_tracks(false);
    assert_eq!(manager.track_connex().unwrap(), original_connex);
}

#[test]
fn track_times_max_time_end_times_and_labels_match_python_manager() {
    let data = vec![
        vec![0.0, 5.0, 2.0, 3.0],
        vec![1.0, 5.0, 3.0, 4.0],
        vec![2.0, 5.0, 4.0, 5.0],
    ];
    let manager = TrackManager::new(data).unwrap();

    assert_eq!(manager.track_times(), Some(vec![5.0, 5.0, 5.0]));
    assert_eq!(manager.max_time(), Some(5));
    assert_eq!(manager.track_end_times(), vec![5.0, 5.0, 5.0]);

    let labels = manager.track_labels(5);
    assert_eq!(labels.labels, vec!["ID:0", "ID:1", "ID:2"]);
    assert_eq!(
        labels.positions,
        vec![
            vec![5.0, 2.0, 3.0],
            vec![5.0, 3.0, 4.0],
            vec![5.0, 4.0, 5.0],
        ]
    );

    let labels = manager.track_labels(6);
    assert!(labels.labels.is_empty());
    assert!(labels.positions.is_empty());
}
