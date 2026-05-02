use std::env;
use std::fmt;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Environment {
    Pip,
    Uv,
    Conda,
    Pixi,
}

impl Environment {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pip => "pip",
            Self::Uv => "uv",
            Self::Conda => "conda",
            Self::Pixi => "pixi",
        }
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseEnvironmentError {
    value: String,
}

impl fmt::Display for ParseEnvironmentError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "unknown environment: {}", self.value)
    }
}

impl std::error::Error for ParseEnvironmentError {}

impl FromStr for Environment {
    type Err = ParseEnvironmentError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "pip" => Ok(Self::Pip),
            "uv" => Ok(Self::Uv),
            "conda" => Ok(Self::Conda),
            "pixi" => Ok(Self::Pixi),
            _ => Err(ParseEnvironmentError {
                value: value.to_string(),
            }),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EnvironmentDetectionError {
    NotInVirtualEnvironment,
    NotInCondaEnvironment,
    PyvenvCfgRead { path: PathBuf, message: String },
}

impl fmt::Display for EnvironmentDetectionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotInVirtualEnvironment => formatter.write_str("Not in a virtual environment"),
            Self::NotInCondaEnvironment => formatter.write_str("Not in a conda environment"),
            Self::PyvenvCfgRead { path, message } => {
                write!(
                    formatter,
                    "failed to read pyvenv.cfg at {}: {}",
                    path.display(),
                    message
                )
            }
        }
    }
}

impl std::error::Error for EnvironmentDetectionError {}

pub fn get_venv_path() -> Option<PathBuf> {
    env::var_os("VIRTUAL_ENV").map(PathBuf::from)
}

pub fn get_conda_path() -> Option<PathBuf> {
    env::var_os("CONDA_PREFIX").map(PathBuf::from)
}

pub fn check_if_uv_env(venv_path: Option<&Path>) -> Result<bool, EnvironmentDetectionError> {
    let venv_path = venv_path.ok_or(EnvironmentDetectionError::NotInVirtualEnvironment)?;
    let pyvenv_cfg = venv_path.join("pyvenv.cfg");
    if !pyvenv_cfg.exists() {
        return Ok(false);
    }
    let text = std::fs::read_to_string(&pyvenv_cfg).map_err(|error| {
        EnvironmentDetectionError::PyvenvCfgRead {
            path: pyvenv_cfg,
            message: error.to_string(),
        }
    })?;
    Ok(text.contains("uv ="))
}

pub fn check_if_pixi_env(conda_path: Option<&Path>) -> Result<bool, EnvironmentDetectionError> {
    let conda_path = conda_path.ok_or(EnvironmentDetectionError::NotInCondaEnvironment)?;
    let conda_meta = conda_path.join("conda-meta");
    if !conda_meta.exists() {
        return Err(EnvironmentDetectionError::NotInCondaEnvironment);
    }
    Ok(conda_meta.join("pixi_env_prefix").exists())
}

pub fn detect_environment_from_paths(
    venv_path: Option<&Path>,
    conda_path: Option<&Path>,
) -> Result<Environment, EnvironmentDetectionError> {
    if let Some(venv_path) = venv_path {
        if check_if_uv_env(Some(venv_path))? {
            return Ok(Environment::Uv);
        }
        return Ok(Environment::Pip);
    }
    if let Some(conda_path) = conda_path {
        if check_if_pixi_env(Some(conda_path))? {
            return Ok(Environment::Pixi);
        }
        return Ok(Environment::Conda);
    }
    Ok(Environment::Pip)
}

pub fn detect_environment() -> Result<Environment, EnvironmentDetectionError> {
    detect_environment_from_paths(get_venv_path().as_deref(), get_conda_path().as_deref())
}
