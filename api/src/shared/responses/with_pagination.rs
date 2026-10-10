use crate::shared::params;

#[derive(Debug, Clone, serde::Serialize, utoipa::ToSchema)]
pub struct WithPagination<T> {
    pub current_page: params::CurrentPage,
    pub page_size: params::PageSize,
    pub total: u32,
    pub pages: u32,
    pub items: Vec<T>,
}
