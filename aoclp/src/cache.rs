use std::env;
use std::env::VarError;
use std::error::Error;
use std::fs::{create_dir_all, read_to_string, rename, write};
use std::path::PathBuf;
use std::str::FromStr;

use crate::anyhow::Context;

#[derive(Debug)]
pub struct DiskCache {
    path: PathBuf,
}

const CACHE_PATH_ENV_VAR: &str = "AOCLP_CACHE_PATH";
const DEFAULT_CACHE_PATH: &str = ".aoclp";

const CACHE_FILE_EXT: &str = ".txt";
const CACHE_FILE_TMP_EXT: &str = ".txt.tmp";
const CACHE_FILE_PC_EXT: &str = ".txt.pc";

impl DiskCache {
    pub fn new() -> crate::Result<Self> {
        let path: PathBuf = match env::var(CACHE_PATH_ENV_VAR) {
            Ok(path) => path.into(),
            Err(VarError::NotPresent) => DEFAULT_CACHE_PATH.into(),
            Err(err) => {
                return Err(err).with_context(|| "failed to get cache path from environment");
            },
        };
        let path = path
            .canonicalize()
            .with_context(|| "failed to get absolute cache path")?;
        create_dir_all(&path).with_context(|| "failed to create cache directory")?;

        Ok(Self { path })
    }

    pub fn get<K, T>(&self, key: K) -> crate::Result<Option<T>>
    where
        K: AsRef<str>,
        T: FromStr,
        <T as FromStr>::Err: Error + Send + Sync + 'static,
    {
        let file_path = self.path_for(key.as_ref(), CACHE_FILE_EXT);
        let pc_file_path = self.path_for(key.as_ref(), CACHE_FILE_PC_EXT);
        if pc_file_path.is_file() {
            rename(&pc_file_path, &file_path).with_context(|| {
                format!(
                    "failed to rename pre-commit cache file for key '{}' from '{}' to '{}",
                    key.as_ref(),
                    pc_file_path.display(),
                    file_path.display(),
                )
            })?;
        }

        if !file_path.is_file() {
            return Ok(None);
        }

        Ok(Some(
            read_to_string(file_path)
                .with_context(|| format!("failed to read cache file for key '{}'", key.as_ref()))?
                .parse()
                .with_context(|| {
                    format!("failed to parse cache file for key '{}'", key.as_ref())
                })?,
        ))
    }

    pub fn set<K, T>(&self, key: K, value: T) -> crate::Result<()>
    where
        K: AsRef<str>,
        T: TryInto<String>,
        <T as TryInto<String>>::Error: Error + Send + Sync + 'static,
    {
        let data = value.try_into().with_context(|| {
            format!("failed to convert cache value for key '{}' to string", key.as_ref())
        })?;

        let tmp_file_path = self.path_for(key.as_ref(), CACHE_FILE_TMP_EXT);
        write(&tmp_file_path, data).with_context(|| {
            format!(
                "failed to write cache file for key '{}' to '{}",
                key.as_ref(),
                tmp_file_path.display(),
            )
        })?;

        let pc_file_path = self.path_for(key.as_ref(), CACHE_FILE_PC_EXT);
        rename(&tmp_file_path, &pc_file_path).with_context(|| {
            format!(
                "failed to rename temp cache file for key '{}' from '{}' to '{}",
                key.as_ref(),
                tmp_file_path.display(),
                pc_file_path.display(),
            )
        })?;

        let file_path = self.path_for(key.as_ref(), CACHE_FILE_EXT);
        rename(&pc_file_path, &file_path).with_context(|| {
            format!(
                "failed to rename pre-commit cache file for key '{}' from '{}' to '{}",
                key.as_ref(),
                tmp_file_path.display(),
                file_path.display(),
            )
        })?;

        Ok(())
    }

    fn path_for(&self, key: &str, ext: &str) -> PathBuf {
        self.path.join(key).with_added_extension(ext)
    }
}
