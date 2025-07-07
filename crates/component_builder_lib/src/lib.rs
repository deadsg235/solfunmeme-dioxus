// Re-export main components
pub use crate::builder::ComponentBuilderEmojiApp;
pub use crate::components::{ComponentConfigPanel, ComponentEmojiDialog};
pub use crate::renderer::RenderComponent;

mod builder;
mod components;
mod renderer;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
