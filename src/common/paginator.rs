use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy)]
pub struct Paginator {
    #[serde(default = "Paginator::default_page_size")]
    pub page_size: i32,
    #[serde(default = "Paginator::default_page")]
    pub page: i32,
}

impl Paginator {
    fn default_page_size() -> i32 {
        10
    }

    fn default_page() -> i32 {
        1
    }

    pub fn offset(self) -> i32 {
        let index = self.page.max(1) - 1;
        index * self.page_size
    }
}
