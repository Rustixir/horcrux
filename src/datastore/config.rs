


#[derive(Clone)]
pub enum StorageType {
    // Store to memory
    RamCopies,

    // Store to memory and persist to disk
    DiskCopies,
}

impl Default for StorageType {
    fn default() -> Self {
        Self::RamCopies
    }
}



#[derive(Clone, Default)]
pub struct Config {
    path: String,
    name: String,
    total_page_size: usize,
    stype: StorageType,
}

impl Config {

    /// default is RamCopies
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        return self
    }

    pub fn with_path(mut self, path: impl Into<String>) -> Self {
        self.path = path.into();
        return self
    }

    pub fn with_total_page_size(mut self, total_page_size: usize) -> Self {
        self.total_page_size = total_page_size;
        return self
    }

    pub fn with_type(mut self, stype: StorageType) -> Self {
        self.stype = stype;
        return self
    }

}
