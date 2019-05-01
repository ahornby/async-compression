use bytes::Bytes;
use flate2::bufread::ZlibDecoder;
use futures::{
    executor::block_on,
    io::AsyncReadExt,
    stream::{self, StreamExt},
};
use std::io::{self, Read};

#[test]
fn zlib_stream() {
    use async_compression::stream::zlib;

    let stream = stream::iter(vec![
        Bytes::from_static(&[1, 2, 3]),
        Bytes::from_static(&[4, 5, 6]),
    ]);
    let compressed = zlib::ZlibStream::new(stream.map(Ok), zlib::Compression::default());
    let data: Vec<_> = block_on(compressed.collect());
    let data: io::Result<Vec<_>> = data.into_iter().collect();
    let data: Vec<u8> = data.unwrap().into_iter().flatten().collect();
    let mut output = vec![];
    ZlibDecoder::new(&data[..])
        .read_to_end(&mut output)
        .unwrap();
    assert_eq!(output, vec![1, 2, 3, 4, 5, 6]);
}

#[test]
fn zlib_read() {
    use async_compression::read::zlib;

    let input = &[1, 2, 3, 4, 5, 6];
    let mut compressed = zlib::ZlibRead::new(&input[..], zlib::Compression::default());
    let mut data = vec![];
    block_on(compressed.read_to_end(&mut data)).unwrap();
    let mut output = vec![];
    ZlibDecoder::new(&data[..])
        .read_to_end(&mut output)
        .unwrap();
    assert_eq!(output, input);
}