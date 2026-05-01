use napari_rs::layers::source::{Source, SourceOverrides, current_source, with_layer_source};

#[test]
fn current_source_defaults_to_empty_source() {
    assert_eq!(current_source(), Source::default());
}

#[test]
fn layer_source_context_sets_current_source_and_resets_afterward() {
    assert_eq!(current_source(), Source::default());

    with_layer_source(
        SourceOverrides::path("some_path").with_reader_plugin("napari"),
        || {
            assert_eq!(
                current_source(),
                Source {
                    path: Some("some_path".to_owned()),
                    reader_plugin: Some("napari".to_owned()),
                    ..Source::default()
                }
            );
        },
    );

    assert_eq!(current_source(), Source::default());
}

#[test]
fn nested_source_contexts_override_deepest_values_and_restore_outer_values() {
    with_layer_source(SourceOverrides::sample("samp", "name"), || {
        assert_eq!(
            current_source(),
            Source {
                sample: Some(("samp".to_owned(), "name".to_owned())),
                ..Source::default()
            }
        );

        with_layer_source(
            SourceOverrides::path("a").with_reader_plugin("plug"),
            || {
                assert_eq!(
                    current_source(),
                    Source {
                        path: Some("a".to_owned()),
                        reader_plugin: Some("plug".to_owned()),
                        sample: Some(("samp".to_owned(), "name".to_owned())),
                        ..Source::default()
                    }
                );

                with_layer_source(SourceOverrides::path("b"), || {
                    assert_eq!(
                        current_source(),
                        Source {
                            path: Some("b".to_owned()),
                            reader_plugin: Some("plug".to_owned()),
                            sample: Some(("samp".to_owned(), "name".to_owned())),
                            ..Source::default()
                        }
                    );
                });

                assert_eq!(
                    current_source(),
                    Source {
                        path: Some("a".to_owned()),
                        reader_plugin: Some("plug".to_owned()),
                        sample: Some(("samp".to_owned(), "name".to_owned())),
                        ..Source::default()
                    }
                );
            },
        );

        assert_eq!(
            current_source(),
            Source {
                sample: Some(("samp".to_owned(), "name".to_owned())),
                ..Source::default()
            }
        );
    });

    assert_eq!(current_source(), Source::default());
}

#[test]
fn source_context_can_record_parent_id_without_layer_type_dependency() {
    with_layer_source(SourceOverrides::sample("samp", "name"), || {
        with_layer_source(SourceOverrides::parent(42), || {
            assert_eq!(
                current_source(),
                Source {
                    sample: Some(("samp".to_owned(), "name".to_owned())),
                    parent: Some(42),
                    ..Source::default()
                }
            );
        });
    });
}
