mod agency_repository;
mod album_repository;
mod album_version_repository;
mod artist_alias_repository;
mod artist_repository;
mod collection_repository;
mod digipack_repository;
mod fanclub_kit_repository;
mod group_member_repository;
mod group_repository;
mod lightstick_repository;
mod photocard_repository;
mod user_favorite_repository;

pub use album_repository::AlbumRepository;
pub use album_version_repository::AlbumVersionRepository;
pub use artist_alias_repository::ArtistAliasRepository;
pub use artist_repository::{ArtistRepository, ArtistSummaryRow};
pub use collection_repository::CollectionRepository;
pub use digipack_repository::DigipackRepository;
pub use fanclub_kit_repository::FanclubKitRepository;
pub use group_member_repository::GroupMemberRepository;
pub use group_repository::{GroupRepository, GroupSummaryRow};
pub use lightstick_repository::LightstickRepository;
pub use photocard_repository::PhotocardRepository;
pub use user_favorite_repository::UserFavoriteRepository;

// ─── Erreur ───────────────────────────────────────────────────────────────────

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Erreur Diesel : {0}")]
    Diesel(#[from] diesel::result::Error),

    #[error("Enregistrement introuvable : {0}")]
    NotFound(String),
}

pub type RepoResult<T> = Result<T, RepositoryError>;

// ─── Pagination ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct Page {
    pub page: u32,
    pub per_page: u32,
}

impl Page {
    pub fn new(page: u32, per_page: u32) -> Self {
        Self {
            page: page.max(1),
            per_page,
        }
    }

    pub fn limit(&self) -> i64 {
        self.per_page as i64
    }

    pub fn offset(&self) -> i64 {
        ((self.page - 1) * self.per_page) as i64
    }
}

impl Default for Page {
    fn default() -> Self {
        Self::new(1, 20)
    }
}

// ─── PageMeta ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageMeta {
    pub per_page: u32,
    pub current_page: u32,
    pub is_first: bool,
    pub is_last: bool,
    pub is_empty: bool,
    pub total: i64,
    pub has_total: bool,
    pub last_page: u32,
    pub has_more_pages: bool,
    pub has_pages: bool,
}

impl PageMeta {
    pub fn new(page: Page, total: i64) -> Self {
        let last_page = if page.per_page == 0 {
            1
        } else {
            ((total as f64) / (page.per_page as f64)).ceil() as u32
        }
        .max(1);

        let has_more_pages = page.page < last_page;
        let is_last = page.page >= last_page;
        let has_pages = total > 0;

        Self {
            per_page: page.per_page,
            current_page: page.page,
            is_first: page.page == 1,
            is_last,
            is_empty: total == 0,
            total,
            has_total: true,
            last_page,
            has_more_pages,
            has_pages,
        }
    }
}

// ─── PaginatedResult ──────────────────────────────────────────────────────────

#[derive(Debug, serde::Serialize)]
pub struct PaginatedResult<T: serde::Serialize> {
    pub data: Vec<T>,
    pub meta: PageMeta,
}

impl<T: serde::Serialize> PaginatedResult<T> {
    pub fn new(data: Vec<T>, page: Page, total: i64) -> Self {
        Self {
            data,
            meta: PageMeta::new(page, total),
        }
    }
}

// ─── Trait Repository ─────────────────────────────────────────────────────────

pub trait Repository<T: serde::Serialize> {
    fn insert(&mut self, item: T) -> RepoResult<T>;
    fn find_by_id(&mut self, id: &str) -> RepoResult<Option<T>>;
    fn find_all(&mut self, page: Page) -> RepoResult<PaginatedResult<T>>;
    fn update(&mut self, item: T) -> RepoResult<T>;
    fn soft_delete(&mut self, id: &str) -> RepoResult<()>;
}
