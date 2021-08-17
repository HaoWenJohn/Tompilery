use std::convert::{TryFrom, TryInto};
use std::fmt::{Display, Formatter};

///Cache for accelerating lexical parsing,
/// it provides word to lexical  analyzer at a time,
/// and maintains inner pointer,moves data and reads new
/// data when 'next' pointer  crosses threshold.
///           next
///            ⬇ max_lookahead_size
///            |   ⬇
///            |-----|
/// buffer  f  s     |    threshold
/// ⬇       ⬇  ⬇     |           |
/// |-------he is a good man-----|--------|
/// \-----------buffer_size---------------/
pub struct InputBuffer{

    buffer:Vec<u8>,
    buffer_size:usize,

    first:usize,
    second:usize,
    next:usize,
    max_lookahead_size:usize,

    threshold:usize,

}
impl InputBuffer{
    pub fn new(buffer_size:u32,max_lookahead_size:usize,threshold:f32)->Self{
        let us_buffer_size= usize::try_from(buffer_size).unwrap_or(1<<20);
        let us_threshold = f32::round(threshold*buffer_size as f32) as usize;
        InputBuffer{
            buffer: Vec::with_capacity(us_buffer_size),
            buffer_size:us_buffer_size,
            first:0,
            second:0,
            next:us_threshold,
            max_lookahead_size,
            threshold:us_threshold
        }
    }

}
impl Display for InputBuffer{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
       write!(f,"buffer_size:{},max_lookahead_size:{},threshold:{}",self.buffer_size,self.max_lookahead_size,self.threshold)
    }
}

use spectral::prelude::*;
#[test]
fn create_buffer(){
    let buffer_size = 1<<30;
    let us_buffer_size = buffer_size as usize;
    let max_lookahead_size = 1<<7;
    let threshold = 0.8;
    let buffer = InputBuffer::new(buffer_size,max_lookahead_size,threshold);

    assert_that(&buffer.buffer_size).is_equal_to(&us_buffer_size);
    assert_that(&buffer.buffer.capacity()).is_equal_to(us_buffer_size);
    assert_that(&buffer.threshold).is_equal_to((threshold * buffer_size as f32)as usize)
}