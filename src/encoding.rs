use core::str;

use crate::size;

const BASE64_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

const BASE64_GROUP_BYTES: u64 = 3;
const BASE64_GROUP_CHARACTERS: u64 = 4;
const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
const PERCENT_ESCAPE_BYTES: u64 = 3;
const SEXTET_MASK: u32 = 0x3f;
const _: () = assert!(BASE64_ALPHABET.len() == 64, "base64 maps six bits onto 64 characters");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Sextet {
    First,
    Fourth,
    Second,
    Third,
}

impl Sextet {
    const fn shift(self) -> u32 {
        match self {
            Self::First => 18,
            Self::Fourth => 0,
            Self::Second => 12,
            Self::Third => 6,
        }
    }
}

#[must_use]
pub(crate) fn base64(input: &[u8]) -> String {
    assert!(!input.is_empty(), "a credential pair is never empty");

    let group_count = size::of_slice(input).div_ceil(BASE64_GROUP_BYTES);
    let capacity = group_count.strict_mul(BASE64_GROUP_CHARACTERS);
    let mut encoded = String::with_capacity(size::offset(capacity));

    for chunk in input.chunks(3) {
        base64_group_push(&mut encoded, chunk);
    }

    assert_eq!(size::of_text(&encoded), capacity, "each group of three bytes is four characters");
    assert!(size::of_text(&encoded).is_multiple_of(4), "base64 is padded to whole groups");

    encoded
}

fn base64_character(group: u32, sextet_position: Sextet) -> char {
    assert!(group < 1_u32 << 24_u32, "a group packs three bytes");

    let shift = sextet_position.shift();

    assert!(shift.is_multiple_of(6), "a sextet starts on a multiple of six bits");

    let sextet = group.wrapping_shr(shift) & SEXTET_MASK;
    let position = size::offset(u64::from(sextet));

    BASE64_ALPHABET.get(position).map_or('A', |byte| char::from(*byte))
}

fn base64_group_push(encoded: &mut String, chunk: &[u8]) {
    assert!(!chunk.is_empty(), "a chunk carries at least one byte");
    assert!(size::of_slice(chunk) <= BASE64_GROUP_BYTES, "a chunk carries at most three bytes");

    let first = chunk.first().copied().unwrap_or(0);
    let second = chunk.get(1).copied().unwrap_or(0);
    let third = chunk.get(2).copied().unwrap_or(0);

    let group =
        u32::from(first).wrapping_shl(16) | u32::from(second).wrapping_shl(8) | u32::from(third);

    encoded.push(base64_character(group, Sextet::First));
    encoded.push(base64_character(group, Sextet::Second));

    if chunk.len() < 2 {
        encoded.push('=');
    } else {
        encoded.push(base64_character(group, Sextet::Third));
    }

    if chunk.len() < 3 {
        encoded.push('=');
    } else {
        encoded.push(base64_character(group, Sextet::Fourth));
    }
}

const fn byte_is_query(byte: u8) -> bool {
    if byte_is_unreserved(byte) {
        return true;
    }

    matches!(
        byte,
        b'!' | b'$'
            | b'%'
            | b'&'
            | b'\''
            | b'('
            | b')'
            | b'*'
            | b'+'
            | b','
            | b'/'
            | b':'
            | b';'
            | b'='
            | b'?'
            | b'@',
    )
}

const fn byte_is_unreserved(byte: u8) -> bool {
    if byte.is_ascii_alphanumeric() {
        return true;
    }

    matches!(byte, b'-' | b'.' | b'_' | b'~')
}

#[must_use]
pub(crate) fn digest(chunks: &[&[u8]]) -> u64 {
    assert!(size::of_slice(chunks) < u64::MAX, "a digest reads a countable run of chunks");

    let mut held = FNV_OFFSET;

    for byte in chunks.iter().copied().flatten() {
        held = (held ^ u64::from(*byte)).wrapping_mul(FNV_PRIME);
    }

    held
}

fn escape(value: &str, is_kept: fn(u8) -> bool) -> String {
    assert!(!value.is_empty(), "an empty query term is never encoded");

    let bytes_max = size::of_text(value).strict_mul(PERCENT_ESCAPE_BYTES);
    let mut encoded = String::with_capacity(size::offset(bytes_max));

    for byte in value.bytes() {
        if is_kept(byte) {
            encoded.push(char::from(byte));

            continue;
        }

        encoded.push('%');
        encoded.push(hex_digit(byte.wrapping_shr(4)));
        encoded.push(hex_digit(byte & 0x0f));
    }

    assert!(encoded.len() >= value.len(), "escaping never shortens a value");
    assert!(size::of_text(&encoded) <= bytes_max, "escaping at most triples a value");

    encoded
}

fn hex_digit(nibble: u8) -> char {
    assert!(nibble <= 0x0f, "a hex digit encodes four bits");

    char::from_digit(u32::from(nibble), 16).unwrap_or('0').to_ascii_uppercase()
}

#[must_use]
pub(crate) fn percent(value: &str) -> String {
    escape(value, byte_is_unreserved)
}

#[must_use]
pub(crate) fn percent_decode(text: &str) -> Vec<u8> {
    let bytes = text.as_bytes();
    let mut decoded: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut position: u64 = 0;

    for _byte_index in 0..bytes.len() {
        let start = size::offset(position);

        let Some(byte) = bytes.get(start) else {
            break;
        };

        let digits = bytes.get(start.strict_add(1)..start.strict_add(3));
        let hex = digits.filter(|found| found.iter().all(u8::is_ascii_hexdigit));
        let pair = hex.and_then(|found| str::from_utf8(found).ok());
        let escape = pair.filter(|_found| *byte == b'%');

        if let Some(found) = escape.and_then(|pair_text| u8::from_str_radix(pair_text, 16).ok()) {
            decoded.push(found);
            position = position.strict_add(3);
        } else {
            decoded.push(*byte);
            position = position.strict_add(1);
        }
    }

    assert!(decoded.len() <= bytes.len(), "decoding never lengthens a value");
    assert!(size::offset(position) >= decoded.len(), "each decoded byte spends an input byte");

    decoded
}

#[must_use]
pub(crate) fn query_escape(raw: &str) -> String {
    let escaped = escape(raw, byte_is_query);

    assert!(escaped.bytes().all(byte_is_query), "an escaped query holds only what a query may");

    escaped
}

#[cfg(test)]
mod tests {
    use super::{base64, digest, percent, percent_decode, query_escape};

    #[test]
    fn a_digest_matches_the_published_fnv_vectors_and_folds_across_its_chunks() {
        assert_eq!(digest(&[]), 0xcbf2_9ce4_8422_2325, "no bytes leave the offset");
        assert_eq!(digest(&[b"a"]), 0xaf63_dc4c_8601_ec8c, "FNV-1a of a");
        assert_eq!(digest(&[b"foobar"]), 0x8594_4171_f739_67e8, "FNV-1a of foobar");
        assert_eq!(digest(&[b"foo", b"bar"]), 0x8594_4171_f739_67e8, "split in two");
    }

    #[test]
    fn base64_matches_known_vectors() {
        assert_eq!(base64(b"f"), "Zg==", "one byte pads twice");
        assert_eq!(base64(b"fo"), "Zm8=", "two bytes pad once");
        assert_eq!(base64(b"foo"), "Zm9v", "three bytes pad never");
        assert_eq!(base64(b"foob"), "Zm9vYg==", "a fourth byte starts a new group");
        assert_eq!(base64(b"\x00\xff\x80"), "AP+A", "every bit pattern maps");

        assert_eq!(
            base64(b"user@example.com:token123"),
            "dXNlckBleGFtcGxlLmNvbTp0b2tlbjEyMw==",
            "a credential pair encodes as the HTTP header expects",
        );
    }

    #[test]
    #[should_panic(expected = "a credential pair is never empty")]
    fn base64_refuses_an_empty_input() {
        let _encoded = base64(b"");
    }

    #[test]
    fn percent_decode_reverses_the_escapes_the_uri_standard_names() {
        assert_eq!(percent_decode("a%20b%2Fc"), b"a b/c", "two escapes");
        assert_eq!(percent_decode("%C3%A9"), "\u{e9}".as_bytes(), "two bytes of one character");
        assert_eq!(percent_decode("plain"), b"plain", "nothing to do");
        assert_eq!(percent_decode("100%"), b"100%", "a bare percent stays");
        assert_eq!(percent_decode("%zz"), b"%zz", "a bad escape stays");
        assert_eq!(percent_decode("%+A"), b"%+A", "a sign is no hex digit");
        assert_eq!(percent_decode("%-1%A"), b"%-1%A", "and neither is a minus");
    }

    #[test]
    fn percent_decode_undoes_percent_for_every_byte_a_text_can_hold() {
        let edges = [0x80, 0x7ff, 0x800, 0xffff, 0x1_0000, 0x10_ffff];
        let every_width: String = (0_u32..=0x7f).chain(edges).filter_map(char::from_u32).collect();

        assert_eq!(every_width.len(), 0x80 + 2 + 2 + 3 + 3 + 4 + 4, "each width of UTF-8");

        for value in ["a b", "x/y?z=1&w", "\u{e9}\u{4e16}\u{1f600}", "100%", "%2F", &every_width] {
            assert_eq!(percent_decode(&percent(value)), value.as_bytes(), "{value} round trips");
        }
    }

    #[test]
    fn percent_escapes_everything_outside_the_unreserved_set() {
        assert_eq!(percent("a-b.c_d~e9"), "a-b.c_d~e9", "the unreserved set passes");
        assert_eq!(percent(" "), "%20", "a space escapes");
        assert_eq!(percent("&"), "%26", "an ampersand escapes");
        assert_eq!(percent("="), "%3D", "an equals sign escapes");
        assert_eq!(percent("/"), "%2F", "a slash escapes");
        assert_eq!(percent("\""), "%22", "a quote escapes");
        assert_eq!(percent("\u{e9}"), "%C3%A9", "each UTF-8 byte escapes");

        assert_eq!(
            percent("assignee = \"5b10ac\""),
            "assignee%20%3D%20%225b10ac%22",
            "a JQL term escapes whole",
        );
    }

    #[test]
    #[should_panic(expected = "an empty query term is never encoded")]
    fn percent_refuses_an_empty_input() {
        let _encoded = percent("");
    }

    #[test]
    fn query_escape_keeps_what_a_query_may_hold_and_escapes_the_rest() {
        let kept = "a-b.c_d~e9!$&'()*+,;=:/?@%2F%";

        assert_eq!(query_escape(kept), kept, "the RFC 3986 query set passes, escapes included");
        assert_eq!(query_escape("a b"), "a%20b", "a space escapes");
        assert_eq!(query_escape("{\"<>}"), "%7B%22%3C%3E%7D", "the bytes a URI refuses escape");
        assert_eq!(query_escape("a\r\nb"), "a%0D%0Ab", "a line break cannot split a header");
        assert_eq!(query_escape("caf\u{e9}"), "caf%C3%A9", "each UTF-8 byte escapes");
        assert_eq!(query_escape("#"), "%23", "a fragment mark cannot end the query");
    }
}
