use std::io::{Write, Result};

pub struct EncodingPart<'a> {
    pub word: &'a [u8],
    pub prev: Option<&'a EncodingPart<'a>>,
}

impl<'a> EncodingPart<'a> {
    pub fn write_into(&self, writer: &mut impl Write) -> Result<()> {
        if let Some(prev) = self.prev {
            prev.write_into(writer)?;
            writer.write_all(b" ")?;
        }
        writer.write_all(self.word)?;
        Ok(())
    }
}