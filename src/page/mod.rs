
mod header;
mod index;

mod space_info;
mod ty;
mod util;

use derive_more::{Display, From};
use rkyv::{Archive, Deserialize, Serialize};

pub use header::GeneralHeader;
pub use index::{map_tree_index, map_unique_tree_index, IndexPage};
pub use space_info::{SpaceInfo, Interval};
pub use ty::PageType;
pub use util::map_index_pages_to_general;

// TODO: Move to config
/// The size of a page. Header size and other parts are _included_ in this size.
/// That's exact page size.
pub const PAGE_SIZE: usize = 4096 * 4;

/// Length of [`GeneralHeader`].
///
/// ## Rkyv representation
///
/// Length of the values are:
///
/// * `page_id` - 4 bytes,
/// * `previous_id` - 4 bytes,
/// * `next_id` - 4 bytes,
/// * `page_type` - 2 bytes,
/// * `space_id` - 4 bytes,
///
/// **2 bytes are added by rkyv implicitly.**
pub const HEADER_LENGTH: usize = 20;

/// Length of the inner part of [`General`] page. It's counted as [`PAGE_SIZE`]
/// without [`General`] page [`HEADER_LENGTH`].
pub const INNER_PAGE_LENGTH: usize = PAGE_SIZE - HEADER_LENGTH;

/// Represents page's identifier. Is unique within the table bounds
#[derive(
    Archive,
    Copy,
    Clone,
    Deserialize,
    Debug,
    Display,
    Eq,
    From,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
pub struct PageId(u32);

impl PageId {
    pub fn next(self) -> Self {
        PageId(self.0 + 1)
    }
}

impl From<PageId> for usize {
    fn from(value: PageId) -> Self {
        value.0 as usize
    }
}

/// General page representation.
#[derive(
    Archive, Copy, Clone, Deserialize, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct General<Inner> {
    pub header: GeneralHeader,
    pub inner: Inner,
}

#[cfg(test)]
mod tests {
    use crate::page::ty::PageType;
    use crate::page::{GeneralHeader, HEADER_LENGTH};

    fn get_general_header() -> GeneralHeader {
        GeneralHeader {
            page_id: 1.into(),
            previous_id: 2.into(),
            next_id: 4.into(),
            page_type: PageType::Index,
            space_id: 5.into(),
        }
    }

    #[test]
    fn general_header_length_valid() {
        let header = get_general_header();
        let bytes = rkyv::to_bytes::<_, 32>(&header).unwrap();

        assert_eq!(bytes.len(), HEADER_LENGTH)
    }
}
