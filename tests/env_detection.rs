use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

use napari_rs::utils::env_detection::{
    Environment, EnvironmentDetectionError, check_if_pixi_env, check_if_uv_env,
    detect_environment_from_paths,
};

static TEMP_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn temp_path(label: &str) -> PathBuf {
    let counter = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "napari-rs-env-detection-{}-{}-{}",
        std::process::id(),
        label,
        counter
    ))
}

#[test]
fn environment_strings_match_python_str_enum_values() {
    assert_eq!(Environment::Pip.to_string(), "pip");
    assert_eq!(Environment::Uv.to_string(), "uv");
    assert_eq!(Environment::Conda.to_string(), "conda");
    assert_eq!(Environment::Pixi.to_string(), "pixi");

    assert_eq!("pip".parse::<Environment>().unwrap(), Environment::Pip);
    assert_eq!("uv".parse::<Environment>().unwrap(), Environment::Uv);
    assert_eq!("conda".parse::<Environment>().unwrap(), Environment::Conda);
    assert_eq!("pixi".parse::<Environment>().unwrap(), Environment::Pixi);
    assert!("Pip".parse::<Environment>().is_err());
}

#[test]
fn uv_detection_matches_python_pyvenv_cfg_marker_logic() {
    let uv_env = temp_path("uv");
    std::fs::create_dir_all(&uv_env).unwrap();
    std::fs::write(uv_env.join("pyvenv.cfg"), "home = /usr/bin\nuv = true\n").unwrap();
    assert!(check_if_uv_env(Some(&uv_env)).unwrap());

    let regular_venv = temp_path("regular-venv");
    std::fs::create_dir_all(&regular_venv).unwrap();
    std::fs::write(regular_venv.join("pyvenv.cfg"), "home = /usr/bin\n").unwrap();
    assert!(!check_if_uv_env(Some(&regular_venv)).unwrap());

    let missing_cfg = temp_path("missing-cfg");
    std::fs::create_dir_all(&missing_cfg).unwrap();
    assert!(!check_if_uv_env(Some(&missing_cfg)).unwrap());

    assert_eq!(
        check_if_uv_env(None).unwrap_err(),
        EnvironmentDetectionError::NotInVirtualEnvironment
    );

    let _ = std::fs::remove_dir_all(uv_env);
    let _ = std::fs::remove_dir_all(regular_venv);
    let _ = std::fs::remove_dir_all(missing_cfg);
}

#[test]
fn pixi_detection_matches_python_conda_meta_marker_logic() {
    let pixi_env = temp_path("pixi");
    let pixi_meta = pixi_env.join("conda-meta");
    std::fs::create_dir_all(&pixi_meta).unwrap();
    std::fs::write(pixi_meta.join("pixi_env_prefix"), "").unwrap();
    assert!(check_if_pixi_env(Some(&pixi_env)).unwrap());

    let conda_env = temp_path("conda");
    std::fs::create_dir_all(conda_env.join("conda-meta")).unwrap();
    assert!(!check_if_pixi_env(Some(&conda_env)).unwrap());

    let missing_meta = temp_path("missing-meta");
    std::fs::create_dir_all(&missing_meta).unwrap();
    assert_eq!(
        check_if_pixi_env(Some(&missing_meta)).unwrap_err(),
        EnvironmentDetectionError::NotInCondaEnvironment
    );
    assert_eq!(
        check_if_pixi_env(None).unwrap_err(),
        EnvironmentDetectionError::NotInCondaEnvironment
    );

    let _ = std::fs::remove_dir_all(pixi_env);
    let _ = std::fs::remove_dir_all(conda_env);
    let _ = std::fs::remove_dir_all(missing_meta);
}

#[test]
fn detect_environment_prefers_virtualenv_then_conda_like_python() {
    let uv_env = temp_path("detect-uv");
    std::fs::create_dir_all(&uv_env).unwrap();
    std::fs::write(uv_env.join("pyvenv.cfg"), "uv = true\n").unwrap();

    let pip_env = temp_path("detect-pip-venv");
    std::fs::create_dir_all(&pip_env).unwrap();
    std::fs::write(pip_env.join("pyvenv.cfg"), "home = /usr/bin\n").unwrap();

    let conda_env = temp_path("detect-conda");
    std::fs::create_dir_all(conda_env.join("conda-meta")).unwrap();

    let pixi_env = temp_path("detect-pixi");
    let pixi_meta = pixi_env.join("conda-meta");
    std::fs::create_dir_all(&pixi_meta).unwrap();
    std::fs::write(pixi_meta.join("pixi_env_prefix"), "").unwrap();

    assert_eq!(
        detect_environment_from_paths(Some(&uv_env), Some(&pixi_env)).unwrap(),
        Environment::Uv
    );
    assert_eq!(
        detect_environment_from_paths(Some(&pip_env), Some(&pixi_env)).unwrap(),
        Environment::Pip
    );
    assert_eq!(
        detect_environment_from_paths(None, Some(&pixi_env)).unwrap(),
        Environment::Pixi
    );
    assert_eq!(
        detect_environment_from_paths(None, Some(&conda_env)).unwrap(),
        Environment::Conda
    );
    assert_eq!(
        detect_environment_from_paths(None, None).unwrap(),
        Environment::Pip
    );

    let _ = std::fs::remove_dir_all(uv_env);
    let _ = std::fs::remove_dir_all(pip_env);
    let _ = std::fs::remove_dir_all(conda_env);
    let _ = std::fs::remove_dir_all(pixi_env);
}
