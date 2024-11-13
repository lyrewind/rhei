use axum::{extract::Query, http::StatusCode, Json};
use rhei_server::library::{
    filter_items, get_library_dir, get_static_dir, LibraryError, LibraryItem,
};
use rhei_server::page::Page;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct FetchData<T> {
    items: Vec<T>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimpleQuery {
    #[serde(default = "default_page_number")]
    pub page: usize,
    #[serde(default = "default_page_size")]
    pub page_size: usize,
    #[serde(default)]
    pub all: bool,
    #[serde(default)]
    pub query: String, // maybe not the best name choice
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DepthQuery {
    #[serde(default = "default_page_number")]
    pub page: usize,
    #[serde(default = "default_page_size")]
    pub page_size: usize,
    pub path: String,
    #[serde(default)]
    pub all: bool,
    #[serde(default)]
    pub query: String,
}

pub async fn fetch_library(
    query: Query<DepthQuery>,
) -> Result<Json<FetchData<LibraryItem>>, StatusCode> {
    let path = &format!("{}/{}", get_library_dir(), query.path);

    let items = match get_static_dir(path).await {
        Ok(items) => items,
        Err(err) => match err {
            LibraryError::NotFound => return Err(StatusCode::NOT_FOUND),
            LibraryError::Other => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    };
    let items = filter_items(items, &query.query);

    let items = if query.all {
        items
    } else {
        Page::create(items, query.page_size).nth(query.page)
    };

    Ok(Json(FetchData { items }))
}

pub async fn fetch_library_root(
    query: Query<SimpleQuery>,
) -> Result<Json<FetchData<LibraryItem>>, StatusCode> {
    let items = match get_static_dir(&get_library_dir()).await {
        Ok(items) => items,
        Err(err) => match err {
            LibraryError::NotFound => return Err(StatusCode::NOT_FOUND),
            LibraryError::Other => return Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    };

    let items = filter_items(items, &query.query);
    let page = Page::create(items, query.page_size).nth(query.page);
    Ok(Json(FetchData { items: page }))
}

fn default_page_size() -> usize {
    10
}

fn default_page_number() -> usize {
    0
}
