use crate::components::character::Props;
use unicode_segmentation::UnicodeSegmentation;

pub fn segment(text: &str, words_page: usize) -> Vec<Vec<Vec<Props>>> {
    let sentences = text.split_sentence_bounds();
    segment_sentences(sentences, words_page)
}

pub fn segment_sentences<'a, S: Iterator<Item = &'a str>>(
    sentences: S,
    words_page: usize,
) -> Vec<Vec<Vec<Props>>> {
    let mut count = std::collections::HashMap::new();
    let (mut pages, last_page, _words) = sentences
        .map(|sentence| {
            let words = sentence
                .split_word_bounds()
                .filter(|w| !w.contains(char::is_whitespace));
            let words: Vec<Props> = words
                .map(|word| {
                    let c = if let Some(n) = count.get_mut(word) {
                        *n += 1;
                        *n
                    } else {
                        count.insert(word, 1);
                        1
                    };
                    if c < 4 || (c < 10 && c % 2 == 1) || (c < 19 && c % 3 == 0) {
                        Props::new(word, true)
                    } else {
                        Props::new(word, false)
                    }
                })
                .collect();
            let word_count = words.len();
            (word_count, words)
        })
        .fold(
            (Vec::new(), Vec::new(), 0),
            |(mut pages, mut page, mut words), (word_count, sentence)| {
                if words < words_page {
                    page.push(sentence);
                    words += word_count;
                    (pages, page, words)
                } else {
                    pages.push(page);
                    (pages, Vec::new(), 0)
                }
            },
        );
    pages.push(last_page);
    pages
}
