use html5ever::local_name;
use html5ever::tokenizer::{
    BufferQueue, Tag,
    Token::{self, *},
    TokenSink, TokenSinkResult, Tokenizer, TokenizerResult,
};
use std::fmt;

pub struct CleanHtml<'a>(pub &'a str);

impl<'a> fmt::Display for CleanHtml<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sink = MarkupTagCleaner { w: f };
        let mut tokenizer = Tokenizer::new(sink, Default::default());
        let mut buffer_queue = BufferQueue::new();
        buffer_queue.push_back(self.0.into());
        assert!(matches!(
            tokenizer.feed(&mut buffer_queue),
            TokenizerResult::Done
        ));
        tokenizer.end();
        Ok(())
    }
}

struct MarkupTagCleaner<'a, 'b: 'a> {
    w: &'a mut fmt::Formatter<'b>,
}

impl<'a, 'b> TokenSink for MarkupTagCleaner<'a, 'b> {
    type Handle = ();

    fn process_token(&mut self, token: Token, _line: u64) -> TokenSinkResult<()> {
        match token {
            CharacterTokens(b) => {
                self.w.write_str(&b[..]).unwrap();
            }
            TagToken(Tag { name, .. }) if name == local_name!("br") => {
                self.w.write_str("\n").unwrap();
            }
            NullCharacterToken => self.w.write_str("\0").unwrap(),
            _ => {}
        }
        TokenSinkResult::Continue
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn simple() {
        assert_eq!(
            super::CleanHtml("<asdf>asdf</asdf>, asdf").to_string(),
            "asdf, asdf"
        );
    }

    #[test]
    fn br() {
        assert_eq!(
            super::CleanHtml("<asdf>asdf</asdf>,<br>asdf").to_string(),
            "asdf,\nasdf"
        );
    }
}
