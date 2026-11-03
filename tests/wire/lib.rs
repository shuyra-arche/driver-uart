//! 控制台那一面的 wire 边界：raw 字节的**编码即内容**。
//!
//! 这一组只念 `uart_api::frame::Bytes` 的公开形：一段非空字节、无表头、无条数、无终止符。
//! 零字节是**数据**（非空即可），`&[]` 才是"没有这一条"。

#[cfg(test)]
mod tests {
    use uart_api::frame::{Bytes, MAX};
    use wire::Message;

    #[test]
    fn raw_bytes_are_stored_without_header_or_terminator() {
        let sample = b"\x00\x1babc\x00\x7f\x00";
        let value = Bytes::of(sample).expect("nonempty, within MAX");
        let mut out = [0xff; MAX];
        assert_eq!(value.store(&mut out), Some(sample.len()));
        assert_eq!(&out[..sample.len()], &sample[..]);
        assert!(
            out[sample.len()..].iter().all(|byte| *byte == 0xff),
            "store must not add a header, length, padding or terminator"
        );
        assert_eq!(
            Bytes::fetch(&out[..sample.len()]).map(|bytes| bytes.bytes().to_vec()),
            Some(sample.to_vec())
        );
    }

    #[test]
    fn all_zero_nonempty_bytes_are_valid_data() {
        let value = Bytes::of(&[0; 5]).expect("zeros are still data");
        let mut out = [0xff; 5];
        assert_eq!(value.store(&mut out), Some(5));
        assert_eq!(out, [0; 5]);
        assert_eq!(
            Bytes::fetch(&[0; 5]).map(|bytes| bytes.bytes().to_vec()),
            Some(vec![0; 5])
        );
        assert_eq!(
            Bytes::fetch(&[0; MAX]).map(|bytes| bytes.bytes().len()),
            Some(MAX),
            "a full zero cell is not empty"
        );
    }

    #[test]
    fn empty_is_not_a_value() {
        assert!(Bytes::of(&[]).is_none());
        assert!(Bytes::fetch(&[]).is_none());
    }

    #[test]
    fn over_max_is_rejected() {
        assert!(Bytes::of(&[1; MAX + 1]).is_none());
        assert!(Bytes::fetch(&[1; MAX + 1]).is_none());
        assert!(Bytes::fetch(&[0; MAX + 1]).is_none());
    }

    #[test]
    fn max_boundary_round_trips() {
        let mut sample = [0; MAX];
        sample[0] = 0x7f;
        sample[MAX - 1] = 0x80;
        let value = Bytes::of(&sample).expect("MAX is accepted");
        let mut out = [0xaa; MAX];
        assert_eq!(value.store(&mut out), Some(MAX));
        assert_eq!(out, sample);
        assert_eq!(
            Bytes::fetch(&out).map(|bytes| bytes.bytes().to_vec()),
            Some(sample.to_vec())
        );
    }

    #[test]
    fn short_output_buffer_is_rejected() {
        let value = Bytes::of(b"hello").unwrap();
        let mut out = [0; 4];
        assert!(value.store(&mut out).is_none());
    }
}
