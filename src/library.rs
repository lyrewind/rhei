use serde::{Deserialize, Serialize};
use tokio::{
    fs::{self, DirEntry},
    io,
};
use tracing::log::{error, info, warn};

/// Reads the static directory to serve from env.
/// If it's missing, a default is used.
pub fn get_library_dir() -> String {
    use std::env;

    match env::var("RHEI_LIBRARY") {
        Ok(library_path) => library_path,
        Err(_) => String::from("library"),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct LibraryItem {
    pub name: String,
    #[serde(rename = "isDir")]
    pub is_dir: bool,
    #[serde(rename = "hasChildDir")]
    pub has_child_dir: bool,
    #[serde(rename = "hasChildFile")]
    pub has_child_file: bool,
}

impl LibraryItem {
    async fn from_dir_entry(entry: &DirEntry) -> Result<Self, ()> {
        let name = match entry.file_name().into_string() {
            Ok(name) => name,
            Err(_) => return Err(()),
        };

        let metadata = match entry.metadata().await {
            Ok(metadata) => metadata,
            Err(_) => return Err(()),
        };

        let mut has_child_dir = false;
        let mut has_child_file = false;

        if metadata.is_dir() {
            match fs::read_dir(entry.path()).await {
                Ok(mut child_entries) => {
                    while let Ok(Some(child)) = child_entries.next_entry().await {
                        if has_child_dir && has_child_file {
                            break;
                        }

                        let metadata = match child.metadata().await {
                            Ok(metadata) => metadata,
                            Err(_) => return Err(()),
                        };

                        if metadata.is_dir() {
                            has_child_dir = true;
                        }

                        if metadata.is_file() {
                            has_child_file = true;
                        }
                    }
                }
                Err(_) => {
                    error!("failed to read contents at {:?}", entry.path());
                    return Err(());
                }
            };
        }

        Ok(LibraryItem {
            name,
            is_dir: metadata.is_dir(),
            has_child_file,
            has_child_dir,
        })
    }
}

pub enum LibraryError {
    NotFound,
    Other,
}

pub async fn get_static_dir<'a>(path: &'a str) -> Result<Vec<LibraryItem>, LibraryError> {
    match fs::read_dir(path).await {
        Ok(mut files) => {
            let mut items: Vec<LibraryItem> = vec![];

            while let Ok(Some(entry)) = files.next_entry().await {
                let item = match LibraryItem::from_dir_entry(&entry).await {
                    Ok(item) => item,
                    Err(_) => {
                        warn!(
                            "failed to create library entry from {:?}",
                            entry.file_name()
                        );
                        continue;
                    }
                };

                let is_empty_dir = item.is_dir && (!item.has_child_file && !item.has_child_dir);

                if !is_empty_dir {
                    items.push(item);
                }
            }

            items.sort();
            Ok(items)
        }
        Err(err) => match err.kind() {
            io::ErrorKind::NotFound => Err(LibraryError::NotFound),
            _ => Err(LibraryError::Other),
        },
    }
}

pub fn filter_items(items: Vec<LibraryItem>, query: &str) -> Vec<LibraryItem> {
    if query.len() == 0 {
        items
    } else {
        items
            .into_iter()
            .filter(|item| (*item).name.contains(query))
            .collect()
    }
}

#[derive(Debug)]
pub struct ValidationError(String);

pub async fn validate_library() -> Result<(), ValidationError> {
    let lib_dir = get_library_dir();
    match fs::read_dir(&lib_dir).await {
        Ok(_) => info!("found library directory: '{}'", lib_dir),
        Err(err) => {
            warn!(
                "'{}' directory not found, attempting to initialise it.",
                lib_dir
            );

            match err.kind() {
                io::ErrorKind::NotFound => match fs::create_dir(&lib_dir).await {
                    Ok(_) => info!("'{}' directory initialised.", lib_dir),
                    Err(err) => {
                        return Err(ValidationError(format!(
                            "'{}' directory couldn't be initialised.\n{:?}",
                            lib_dir, err
                        )))
                    }
                },
                _err => return Err(ValidationError(format!("{:?}", _err))),
            }
        }
    }

    Ok(())
}
