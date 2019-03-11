extern crate loqui;

use criterion::Criterion;
use bytes::BytesMut;
use tokio_codec::{Decoder, Encoder};

use loqui::protocol::codec::{LoquiCodec, LoquiFrame};
use loqui::protocol::frames;



fn encode_hello(c: &mut Criterion) {
    c.bench_function("encode_hello", |b| b.iter(|| {
        let mut codec = LoquiCodec::new(500);
        let frame = LoquiFrame::Hello(frames::Hello{
            flags: 15,
            version: 1,
            encodings: vec!["msgpack".into(), "json".into()],
            compressions: vec!["gzip".into(), "lzma".into()],
        });
        let dst = &mut BytesMut::with_capacity(1024);
        criterion::black_box(codec.encode(frame, dst));
    }));
}

fn decode_hello(c: &mut Criterion) {
    c.bench_function("decode_hello", |b| b.iter(|| {
        let mut codec = LoquiCodec::new(500);
        let frame_bytes = &b"\x01\x0f\x01\x00\x00\x00\x16msgpack,json|gzip,lzma"[..];
        let src = &mut BytesMut::with_capacity(1024);
        criterion::black_box(codec.decode(src));
    }));
}

fn encode_helloack(c: &mut Criterion) {
    c.bench_function("encode_helloack", |b| b.iter(|| {
        let mut codec = LoquiCodec::new(500);
        let frame = LoquiFrame::HelloAck(frames::HelloAck{
            flags: 15,
            ping_interval_ms: 32000,
            encoding: "msgpack".into(),
            compression: "gzip".into(),
        });
        let dst = &mut BytesMut::with_capacity(1024);
        criterion::black_box(codec.encode(frame, dst));
    }));
}

fn decode_helloack(c: &mut Criterion) {
    c.bench_function("decode_helloack", |b| b.iter(|| {
        let mut codec = LoquiCodec::new(500);
        let frame_bytes = &b"\x02\x0f\x00\x00}\x00\x00\x00\x00\x0cmsgpack|gzip"[..];
        let src = &mut BytesMut::with_capacity(1024);
        criterion::black_box(codec.decode(src));
    }));
}

criterion_group!(
    encoder_benches,
    encode_hello,
    encode_helloack,
);

criterion_group!(
    decoder_benches,
    decode_hello,
    decode_helloack,
);

criterion_main!(encoder_benches, decoder_benches);
