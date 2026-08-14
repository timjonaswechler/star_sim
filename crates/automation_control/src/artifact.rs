use bevy::image::Image;
use std::{
    fs::{self, File, OpenOptions},
    io::{Cursor, Write},
    path::{Component, Path, PathBuf},
};

#[derive(Debug)]
pub struct ArtifactRoot(PathBuf);

impl ArtifactRoot {
    pub fn new(path: impl AsRef<Path>) -> Result<Self, ArtifactError> {
        fs::create_dir_all(path.as_ref()).map_err(ArtifactError::Io)?;
        Ok(Self(
            path.as_ref().canonicalize().map_err(ArtifactError::Io)?,
        ))
    }

    pub fn path(&self) -> &Path {
        &self.0
    }

    /// Reserves a PNG destination. The returned handle prevents accidental overwrite between
    /// validation and writing. Parent directories are canonicalized beneath the configured root.
    pub fn reserve(
        &self,
        requested: impl AsRef<Path>,
        overwrite: bool,
    ) -> Result<ArtifactDestination, ArtifactError> {
        let relative = requested.as_ref();
        if relative.as_os_str().is_empty()
            || relative.is_absolute()
            || relative.extension().and_then(|value| value.to_str()) != Some("png")
            || relative
                .components()
                .any(|component| !matches!(component, Component::Normal(_)))
        {
            return Err(ArtifactError::InvalidPath);
        }
        let path = self.0.join(relative);
        let parent = path.parent().ok_or(ArtifactError::InvalidPath)?;
        fs::create_dir_all(parent).map_err(ArtifactError::Io)?;
        let canonical_parent = parent.canonicalize().map_err(ArtifactError::Io)?;
        if !canonical_parent.starts_with(&self.0) {
            return Err(ArtifactError::EscapesRoot);
        }
        let path = canonical_parent.join(path.file_name().ok_or(ArtifactError::InvalidPath)?);
        if let Ok(metadata) = fs::symlink_metadata(&path)
            && metadata.file_type().is_symlink()
        {
            return Err(ArtifactError::EscapesRoot);
        }
        let mut options = OpenOptions::new();
        options.write(true);
        if overwrite {
            options.create(true).truncate(true);
        } else {
            options.create_new(true);
        }
        let file = options.open(&path).map_err(|error| {
            if error.kind() == std::io::ErrorKind::AlreadyExists {
                ArtifactError::Exists
            } else {
                ArtifactError::Io(error)
            }
        })?;
        Ok(ArtifactDestination { path, file })
    }
}

#[derive(Debug)]
pub struct ArtifactDestination {
    path: PathBuf,
    file: File,
}

impl ArtifactDestination {
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Encodes and fully flushes a captured Bevy image before returning its normalized path.
    pub fn write_png(mut self, captured: Image) -> Result<PathBuf, ArtifactError> {
        let dynamic = captured
            .try_into_dynamic()
            .map_err(|error| ArtifactError::Encode(error.to_string()))?;
        let mut bytes = Cursor::new(Vec::new());
        dynamic
            .to_rgb8()
            .write_to(&mut bytes, image::ImageFormat::Png)
            .map_err(|error| ArtifactError::Encode(error.to_string()))?;
        self.file
            .write_all(bytes.get_ref())
            .map_err(ArtifactError::Io)?;
        self.file.sync_all().map_err(ArtifactError::Io)?;
        Ok(self.path)
    }
}

#[derive(Debug)]
pub enum ArtifactError {
    InvalidPath,
    EscapesRoot,
    Exists,
    Io(std::io::Error),
    Encode(String),
}

impl std::fmt::Display for ArtifactError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPath => {
                formatter.write_str("path must be a relative .png path without traversal")
            }
            Self::EscapesRoot => formatter.write_str("artifact path escapes the configured root"),
            Self::Exists => formatter.write_str("artifact already exists and overwrite is false"),
            Self::Io(error) => write!(formatter, "artifact I/O failed: {error}"),
            Self::Encode(error) => write!(formatter, "PNG encoding failed: {error}"),
        }
    }
}

impl std::error::Error for ArtifactError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn temporary_root(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("automation-control-{name}-{}", std::process::id()))
    }

    #[test]
    fn rejects_traversal_absolute_non_png_and_overwrite() {
        let path = temporary_root("paths");
        let _ = fs::remove_dir_all(&path);
        let root = ArtifactRoot::new(&path).unwrap();
        assert!(root.reserve("screenshots/good.png", false).is_ok());
        assert!(matches!(
            root.reserve("screenshots/good.png", false),
            Err(ArtifactError::Exists)
        ));
        assert!(root.reserve("screenshots/good.png", true).is_ok());
        assert!(matches!(
            root.reserve("../escape.png", false),
            Err(ArtifactError::InvalidPath)
        ));
        assert!(matches!(
            root.reserve("image.jpg", false),
            Err(ArtifactError::InvalidPath)
        ));
        assert!(matches!(
            root.reserve(path.join("absolute.png"), false),
            Err(ArtifactError::InvalidPath)
        ));
        fs::remove_dir_all(path).unwrap();
    }

    #[test]
    fn response_can_only_follow_a_fully_written_png() {
        use bevy::{
            asset::RenderAssetUsages,
            render::render_resource::{Extent3d, TextureDimension, TextureFormat},
        };
        let path = temporary_root("write");
        let _ = fs::remove_dir_all(&path);
        let root = ArtifactRoot::new(&path).unwrap();
        let destination = root.reserve("capture.png", false).unwrap();
        let image = Image::new_fill(
            Extent3d {
                width: 2,
                height: 1,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &[255, 0, 0, 255],
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::MAIN_WORLD,
        );
        let written = destination.write_png(image).unwrap();
        let data = fs::read(&written).unwrap();
        assert_eq!(&data[..8], b"\x89PNG\r\n\x1a\n");
        assert!(data.len() > 24);
        fs::remove_dir_all(path).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn rejects_parent_and_leaf_symlink_escape() {
        use std::os::unix::fs::symlink;
        let path = temporary_root("symlink");
        let outside = temporary_root("outside");
        let _ = fs::remove_dir_all(&path);
        let _ = fs::remove_dir_all(&outside);
        fs::create_dir_all(&outside).unwrap();
        let root = ArtifactRoot::new(&path).unwrap();
        symlink(&outside, path.join("linked")).unwrap();
        assert!(matches!(
            root.reserve("linked/escape.png", false),
            Err(ArtifactError::EscapesRoot)
        ));
        symlink(outside.join("leaf.png"), path.join("leaf.png")).unwrap();
        assert!(matches!(
            root.reserve("leaf.png", true),
            Err(ArtifactError::EscapesRoot)
        ));
        fs::remove_dir_all(path).unwrap();
        fs::remove_dir_all(outside).unwrap();
    }
}
