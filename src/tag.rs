
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct Tag(i64);

impl Tag {
    /// Create tag of any value.
    pub(crate) fn from(id: i64) -> Tag {
        Tag(id)
    }


    pub fn new() -> Tag {
        unsafe {
            COUNTER += 1;
            Tag(COUNTER)
        }
    }

    pub fn none() -> Tag {
        Tag(0)
    }


    pub fn special(id: i64) -> Option<Tag> {
        if (64..=128).contains(&id) {
            Some(Tag(id))
        } else {
            None
        }
    }

    pub fn id(&self) -> i64 {
        self.0
    }
}


static mut COUNTER: i64 = 128;

impl Tag {}

impl Default for Tag {
    fn default() -> Self {
        Self::new()
    }
}
