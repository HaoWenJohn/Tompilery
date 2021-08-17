use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Formatter, format};

///Cache for accelerating lexical parsing,
/// it provides word to lexical  analyzer at a time,
/// and maintains inner pointer,moves data and reads new
/// data when 'word_tail' pointer  crosses threshold.

///             max_lookahead_size
///                ⬇
///             |------|
/// buffer  h t next   |    threshold
/// ⬇       ⬇⬇  ⬇      |          |
/// |-------he  is a good man-----|--------|
/// \-----------buffer_size---------------/
pub struct InputBuffer {
    buffer: Vec<u8>,
    buffer_size: usize,

    word_head: usize,
    word_tail: usize,
    next: usize,
    max_lookahead_size: usize,

    threshold: usize,

}

impl InputBuffer {
    pub fn new(buffer_size: u32, max_lookahead_size: usize, threshold: f32) -> Self {
        let us_buffer_size = usize::try_from(buffer_size).unwrap_or(1 << 20);
        let us_threshold = f32::round(threshold * buffer_size as f32) as usize;
        InputBuffer {
            buffer: Vec::with_capacity(us_buffer_size),
            buffer_size: us_buffer_size,
            word_head: 0,
            word_tail: 0,
            next: 0,
            max_lookahead_size,
            threshold: us_threshold,
        }
    }

    pub fn advance(&mut self) {
        self.word_head = self.next;
        loop {
            let next_char = self.lookahead(1)[0];
            if next_char == b' ' {
                if self.word_tail <= self.word_head {
                    self.word_tail = self.next - 1;
                }
                break;
            }
        }
        loop{
            if  self.lookahead(1)[0] != b' '{
                self.back(1);
                return;
            }
        }

    }
    pub fn back(&mut self,step:usize){
        self.next-=step;
    }
    pub fn lookahead(&mut self, step: usize) -> &[u8] {

        self.next += step;
        if self.next >= self.threshold {
            self.flush();
        }
        &self.buffer[self.next - step..self.next]
    }

    pub fn flush(&mut self) {
        //todo load new data

        let gap = self.word_head;
        self.word_head = 0;
        self.word_tail = self.word_tail - gap;
        self.next = self.next - gap;
    }
}

impl Display for InputBuffer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "buffer_size:{},max_lookahead_size:{},threshold:{}", self.buffer_size, self.max_lookahead_size, self.threshold)
    }
}

use spectral::prelude::*;

fn default_test_case() -> InputBuffer {
    InputBuffer::new(1 << 20, 1 << 7, 0.8)
}

#[test]
fn test_create_buffer() {
    let buffer_size = 1 << 30;
    let us_buffer_size = buffer_size as usize;
    let max_lookahead_size = 1 << 7;
    let threshold = 0.8;
    let buffer = InputBuffer::new(
        buffer_size,
        max_lookahead_size,
        threshold,
    );
    assert_that(&buffer.buffer_size).is_equal_to(&us_buffer_size);
    assert_that(&buffer.buffer.capacity()).is_equal_to(us_buffer_size);
    assert_that(&buffer.threshold).is_equal_to((threshold * buffer_size as f32) as usize)
}
#[test]
fn test_lookahead(){
    let mut buffer = default_test_case();
    let contents = "test for lookahead function".as_bytes();
    contents.into_iter().cloned()
        .for_each(|c| buffer.buffer.push(c));

    assert_eq!(buffer.lookahead(4), "test".as_bytes());
    assert_eq!(buffer.next,4);

    assert_eq!(buffer.lookahead(5), " for ".as_bytes());
    assert_eq!(buffer.next,9);
}
#[test]
fn test_advance() {
    let mut buffer = default_test_case();
    let contents = "test for advance function".as_bytes();
    contents.into_iter().cloned()
        .for_each(|c| buffer.buffer.push(c));


    buffer.advance();


    let test =&buffer.buffer[buffer.word_head..buffer.word_tail];
    let expect_test = "test".as_bytes();
    assert_eq!(test,expect_test);
    buffer.advance();
    let for_ =&buffer.buffer[buffer.word_head..buffer.word_tail];
    let expect_for = "for".as_bytes();
    assert_eq!(for_,expect_for);
    buffer.advance();
    let advance =&buffer.buffer[buffer.word_head..buffer.word_tail];
    let expect_advance = "advance".as_bytes();
    assert_eq!(advance,expect_advance);
    // buffer.advance();
    // let function =&buffer.buffer[buffer.word_head..buffer.word_tail];
    // let expect_function = "function".as_bytes();
    // assert_eq!(function,expect_function);
    // buffer.advance();
    // let none =&buffer.buffer[buffer.word_head..buffer.word_tail];
    // let expect_none = "".as_bytes();
    // assert_eq!(none,expect_none);
}