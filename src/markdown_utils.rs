// Converts pandoc inline footnotes to standard foot notes
pub fn preprocess_inline_footnotes(input: &str) -> String {
    let mut footnote_counter = 1;
    let mut output = String::new();
    let mut footnotes = Vec::new();

    for line in input.lines() {
        line.find("^[")
        .and_then(|start| { 
            line[start..].find("]").map(|end| {
                let footnote_text = &line[start + 2..start + end];
                let footnote_label = format!("[^{}]", footnote_counter);
                footnotes.push(format!("[^{}]: {}", footnote_counter, footnote_text));
                output.push_str(&line.replace(&line[start..start + end + 1], &footnote_label));
                footnote_counter += 1;
            })
        })
        .or_else(||{
             output.push_str(line);
             Some(())
        });

        output.push('\n');
    }

    output.push_str("\n");
    for footnote in footnotes {
        output.push_str(&footnote);
        output.push('\n');
    }

    output = output.trim().to_string();

    output
}

pub fn remove_headers(input : &str) -> String{
    let mut output = String::new();
    for line in input.lines() {
        line.starts_with("#")
        .then(|| {
            output.push_str(line.trim_start_matches(|c| c == '#').trim());
        })
        .or_else(||{
            output.push_str(line);
            Some(())
        });

        output.push('\n');
    }

    output = output.trim().to_string();

    output
}

#[cfg(target_arch = "wasm32")]
#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::wasm_bindgen_test;
    use wasm_bindgen_test::wasm_bindgen_test_configure;
    wasm_bindgen_test_configure!(run_in_browser); 

    #[wasm_bindgen_test]
    fn test_preprocess_inline_footnotes_no_footnotes() {
        let original_txt = "some **markup** [text](http://to.txt)";
        let output_text = preprocess_inline_footnotes(original_txt);
        assert_eq!(original_txt,output_text);
    }

    #[wasm_bindgen_test]
    fn test_preprocess_inline_footnotes_with_footnote() {
        let original_txt = "some **markup** [text](http://to.txt) and let add a foot note [^1foot note]";
        let output_text = preprocess_inline_footnotes(original_txt);
        assert_eq!(original_txt,output_text);
    }

    #[wasm_bindgen_test]
    fn test_preprocess_inline_footnotes_with_pandoc_footnote() {
        let original_txt = "some **markup** [text](http://to.txt) and let add a foot note ^[foot note]";
        let expected_txt = "some **markup** [text](http://to.txt) and let add a foot note [^1]\n\n[^1]: foot note";

        let output_text = preprocess_inline_footnotes(original_txt);
        assert_eq!(expected_txt, output_text);
    }

    #[wasm_bindgen_test]
    fn test_preprocess_inline_footnotes_with_pandoc_footnote_he() {
        let original_txt = "# מימון מדינה ללא מסים

תחשבו מה היה קורה אם ראש ממשלת ישראל היה מוציא צו: מהיום והלאה החל מהראשון לינואר עד לראשון למאי כל אזרח במדינה יעבוד למעני. המדינה תחליט במה הוא יעבוד. הכסף שהוא יכניס ילך לקופת האוצר. מהראשון למאי עד לראשון לינואר, כל עובד יוכל לעבוד באיזה עבודה שבה הוא יחפץ ולהשתמש בכסף שהוא מכניס כפי ראות עיניו^[הרעיון הזה אולי נשמע הזוי, אבל הוא בכלל לא מופרך. שיטת המיסוי הזו הייתה מקובלת מאוד בעולם העתיק.]



רוב הסיכויים שזה לא היה עובר ללא מהומה. אנשים היו מוחים על החודשים שבהם הם צריכים לעבוד למען הממשלה.";

        let expected_txt = "# מימון מדינה ללא מסים\n\nתחשבו מה היה קורה אם ראש ממשלת ישראל היה מוציא צו: מהיום והלאה החל מהראשון לינואר עד לראשון למאי כל אזרח במדינה יעבוד למעני. המדינה תחליט במה הוא יעבוד. הכסף שהוא יכניס ילך לקופת האוצר. מהראשון למאי עד לראשון לינואר, כל עובד יוכל לעבוד באיזה עבודה שבה הוא יחפץ ולהשתמש בכסף שהוא מכניס כפי ראות עיניו[^1]\n\n\n\nרוב הסיכויים שזה לא היה עובר ללא מהומה. אנשים היו מוחים על החודשים שבהם הם צריכים לעבוד למען הממשלה.\n\n[^1]: הרעיון הזה אולי נשמע הזוי, אבל הוא בכלל לא מופרך. שיטת המיסוי הזו הייתה מקובלת מאוד בעולם העתיק.";
        let output_text = preprocess_inline_footnotes(original_txt);
        assert_eq!(expected_txt, output_text);
    }

    #[wasm_bindgen_test]
    fn test_remove_headers() {
        let original_txt = "# First header\nSome line\n## second header.\nAnother line";
        let expected_txt = "First header\nSome line\nsecond header.\nAnother line";

        let output_text = remove_headers(original_txt);
        assert_eq!(expected_txt, output_text);
    }
}
