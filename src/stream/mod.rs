//! Types which operate over [`Stream`](futures::stream::Stream)`<Item =
//! `[`io::Result`](std::io::Result)`<`[`Bytes`](bytes::Bytes)`>>` streams, both encoders and
//! decoders for various formats.
//!
//! The `Stream` is treated as a single byte-stream to be compressed/decompressed, each item is a
//! chunk of data from this byte-stream. There is not guaranteed to be a one-to-one relationship
//! between chunks of data from the underlying stream and the resulting compressed/decompressed
//! stream, the encoders and decoders will buffer the incoming data and choose their own boundaries
//! at which to yield a new item.

mod brotli;
mod deflate;
mod flate;
mod gzip;
mod zlib;
mod zstd;

pub use self::brotli::{BrotliDecoder, BrotliEncoder};
pub use self::deflate::{DeflateDecoder, DeflateEncoder};
pub use self::gzip::{GzipDecoder, GzipEncoder};
pub use self::zlib::{ZlibDecoder, ZlibEncoder};
pub use self::zstd::{ZstdDecoder, ZstdEncoder};
