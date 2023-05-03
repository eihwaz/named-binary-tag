/// Module with helper functions for compressing NBT format (deflating).
pub mod deflate {
    use crate::CompoundTag;
    use crate::encode::write_compound_tag;
    use std::io::{Write, Error};
    use flate::write::{GzEncoder, ZlibEncoder};

    /// Write a compound tag to writer using gzip compression.
    pub fn write_gzip_compound_tag<W: Write>(
        writer: &mut W,
        compound_tag: &CompoundTag,
    ) -> Result<(), Error> {
        write_compound_tag(
            &mut GzEncoder::new(writer, Default::default()),
            compound_tag,
        )
    }

    /// Write a compound tag to writer using zlib compression.
    pub fn write_zlib_compound_tag<W: Write>(
        writer: &mut W,
        compound_tag: &CompoundTag,
    ) -> Result<(), Error> {
        write_compound_tag(
            &mut ZlibEncoder::new(writer, Default::default()),
            compound_tag,
        )
    }
}

/// Module with helper functions for decompressing NBT format (enflating).
pub mod enflate {
    use crate::CompoundTag;
    use crate::decode::{TagDecodeError, read_compound_tag};
    use std::io::Read;
    use flate::read::{GzDecoder, ZlibDecoder};

    /// Read a compound tag from a reader compressed with gzip.
    pub fn read_gzip_compound_tag<R: Read>(reader: &mut R) -> Result<CompoundTag, TagDecodeError> {
        read_compound_tag(&mut GzDecoder::new(reader))
    }

    /// Read a compound tag from a reader compressed with zlib.
    pub fn read_zlib_compound_tag<R: Read>(reader: &mut R) -> Result<CompoundTag, TagDecodeError> {
        read_compound_tag(&mut ZlibDecoder::new(reader))
    }
}
