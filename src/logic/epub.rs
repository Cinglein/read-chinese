use crate::{components::character::Props, logic::segment::segment_sentences};
use unicode_segmentation::UnicodeSegmentation;

pub fn parse_epub(buffer: &[u8], words_page: usize) -> Option<Vec<Vec<Vec<Props>>>> {
    let cursor = std::io::Cursor::new(buffer);
    if let Ok(mut doc) = epub::doc::EpubDoc::from_reader(cursor) {
        let mut pages = Vec::new();
        while let Some((html, _)) = doc.get_current() {
            let chapter = html2text::from_read(html.as_slice(), usize::MAX);
            if !chapter.is_empty() {
                let sentences = chapter.split_sentence_bounds();
                pages.append(&mut segment_sentences(sentences, words_page));
            }
            let next = doc.go_next();
            if !next {
                break;
            }
        }
        Some(pages)
    } else {
        None
    }
}
