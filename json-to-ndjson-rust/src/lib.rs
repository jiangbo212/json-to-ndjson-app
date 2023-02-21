use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn copy_text(text: &str) -> String {
    let new_text = format!("Copied: {}", text);
    new_text
}

pub fn add(left: usize, right: usize) -> usize {
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
