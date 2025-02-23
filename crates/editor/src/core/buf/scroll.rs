use algebra::vec2::usize::USizeVec2;

#[derive(Debug, Clone, Default)]
pub struct EditorScroll {
    pub offset: USizeVec2,
}

impl EditorScroll {
    pub fn get(&self) -> USizeVec2 {
        self.offset
    }
}
