use crate::{
    block_data::BlockData,
    byte_data::{
        compare_bytes, find_bytes_idx, parse_string_to_read_delimiter, rfind_all_bytes_idx,
        rfind_bytes_idx, ByteDataError, BUFF_SIZE_BYTES,
    },
    stale_tracker::StaleTracker,
    traits::IsValid,
};
use alloc::vec::Vec;
use core::{cmp::min, str};
use embedded_io::Read;
use log::*;

pub struct SerialReceiveBlock {
    name: &'static str,
    start_delimiter: (Vec<u8>, Vec<usize>, usize),
    end_delimiter: (Vec<u8>, Vec<usize>, usize),

    // TODO this should be a VecDeque, but embedded_io
    // doesn't provide nice methods for writing to a VecDeque yet
    buffer: Vec<u8>,
    read_bytes: usize,
    pub data: BlockData,
    pub stale_check: StaleTracker,
}

impl SerialReceiveBlock {
    pub fn new(
        name: &'static str,
        start_delimiter: &str,
        end_delimiter: &str,
        read_bytes: f64,
        stale_age_ms: f64,
    ) -> Self {
        let start_delimiter = parse_string_to_read_delimiter(start_delimiter);
        let start_len = start_delimiter.0.len() + start_delimiter.1.len();
        let start_delimiter = (start_delimiter.0, start_delimiter.1, start_len);

        let end_delimiter = parse_string_to_read_delimiter(end_delimiter);
        let end_len = end_delimiter.0.len() + end_delimiter.1.len();
        let end_delimiter = (end_delimiter.0, end_delimiter.1, end_len);

        Self {
            name,
            data: BlockData::from_bytes(b""),
            start_delimiter,
            end_delimiter,
            stale_check: StaleTracker::from_ms(stale_age_ms),
            buffer: Vec::with_capacity(BUFF_SIZE_BYTES),
            read_bytes: read_bytes as usize,
        }
    }

    pub fn run(&mut self, protocol: &mut impl Read, app_time_s: f64) {
        let mut serial_buf = [0u8; BUFF_SIZE_BYTES];
        if let Ok(size) = protocol.read(serial_buf.as_mut_slice()) {
            self.buffer.extend_from_slice(&serial_buf[..size]);
        } else {
            return;
        }

        if let Ok((start_idx, end_idx)) = self.parse_data() {
            let val = &self.buffer[start_idx..end_idx];
            debug!("{}: Parsed value: {:?}", self.name, val);
            if start_idx != 0 {
                debug!("{}: Discarding {} bytes", self.name, start_idx);
            }
            self.data.set_bytes(val);
            self.buffer
                .drain(..(min(end_idx + self.end_delimiter.2, self.buffer.len())));
            self.stale_check.mark_updated(app_time_s);
        } else if self.buffer.len() > BUFF_SIZE_BYTES * 2 {
            self.buffer.clear();
            self.buffer.extend_from_slice(&serial_buf);
            debug!(
                "{}: Read too many bytes without a valid message. Clearing buffer",
                self.name
            );
        }
    }

    fn try_parse_fixed_length_data(
        &self,
        data_buff: &[u8],
        start_indices: &[usize],
    ) -> Result<(usize, usize), ByteDataError> {
        let chunk_end = data_buff.len();
        for chunk_start in start_indices {
            let offset_chunk_start = chunk_start + self.start_delimiter.2;
            if chunk_end < offset_chunk_start {
                continue;
            }

            if chunk_end - offset_chunk_start >= self.read_bytes {
                let end = offset_chunk_start + self.read_bytes;
                return Ok((offset_chunk_start, end));
            }
        }

        debug!("{}: No end delimiter found", self.name);
        Err(ByteDataError::EndDelimiterNotFound)
    }

    fn try_parse_end_delimited_data(
        &self,
        data_buff: &[u8],
        start_indices: &[usize],
    ) -> Result<(usize, usize), ByteDataError> {
        let mut chunk_end = data_buff.len();
        for chunk_start in start_indices {
            let offset_chunk_start = chunk_start + self.start_delimiter.2;
            if self.read_bytes > 0 {
                if offset_chunk_start + self.read_bytes + self.end_delimiter.2 > chunk_end {
                    continue;
                }

                let delim_start = offset_chunk_start + self.read_bytes;
                let delim_end = delim_start + self.end_delimiter.2;
                if compare_bytes(
                    &data_buff[delim_start..delim_end],
                    &self.end_delimiter.0,
                    &self.end_delimiter.1,
                ) {
                    return Ok((offset_chunk_start, delim_start));
                }
            } else if let Ok(v) = find_bytes_idx(
                &data_buff[offset_chunk_start..chunk_end],
                &self.end_delimiter.0,
                &self.end_delimiter.1,
            ) {
                return Ok((offset_chunk_start, offset_chunk_start + v));
            }

            chunk_end = *chunk_start;
        }

        debug!("{}: No end delimiter found", self.name);
        Err(ByteDataError::EndDelimiterNotFound)
    }

    fn parse_data(&self) -> Result<(usize, usize), ByteDataError> {
        let start_idx;
        let end_idx;

        // Use anything in the overflow buffer plus new data
        debug!("{}: Received value: {:?}", self.name, &self.buffer);
        debug!(
            "{}: Start delimiter: {:?}, End delimiter: {:?}",
            self.name, &self.start_delimiter, &self.end_delimiter
        );

        if !self.start_delimiter.0.is_empty() {
            let start_indices = rfind_all_bytes_idx(
                &self.buffer,
                &self.start_delimiter.0,
                &self.start_delimiter.1,
            );

            if start_indices.is_empty() {
                debug!("{}: No start delimiter found", self.name);
                return Err(ByteDataError::StartDelimiterNotFound);
            }

            if !self.end_delimiter.0.is_empty() {
                (start_idx, end_idx) =
                    self.try_parse_end_delimited_data(&self.buffer, &start_indices)?;
            } else if self.read_bytes > 0 {
                (start_idx, end_idx) =
                    self.try_parse_fixed_length_data(&self.buffer, &start_indices)?;
            } else {
                end_idx = self.buffer.len();
                start_idx = start_indices[0] + self.start_delimiter.2;
            }
        } else if !self.end_delimiter.0.is_empty() {
            end_idx = rfind_bytes_idx(&self.buffer, &self.end_delimiter.0, &self.end_delimiter.1)?;
            if self.read_bytes > 0 {
                if end_idx < self.read_bytes {
                    debug!("{}: Not enough bytes to read", self.name);
                    return Err(ByteDataError::InsufficientData);
                }

                start_idx = end_idx - self.read_bytes;
            } else {
                start_idx = rfind_bytes_idx(
                    &self.buffer[..end_idx],
                    &self.end_delimiter.0,
                    &self.end_delimiter.1,
                )
                .map(|idx| idx + self.end_delimiter.2)
                .unwrap_or(0);
            }
        } else {
            start_idx = 0;
            end_idx = self.buffer.len();
        }

        Ok((start_idx, end_idx))
    }
}

impl IsValid for SerialReceiveBlock {
    fn is_valid(&self, app_time_s: f64) -> BlockData {
        self.stale_check.is_valid(app_time_s)
    }
}

#[cfg(all(test, feature = "std"))]
mod tests {
    use protocols::MockRead;

    use super::*;

    struct TestBlockConfig {
        start_delimiter: String,
        end_delimiter: String,
        read_bytes: f64,
    }

    fn test_serial_read_single_chunk(config: &TestBlockConfig, data: &[u8], expected: &[u8]) {
        test_serial_read(config, &[data], expected)
    }

    fn run_serial_read(config: &TestBlockConfig, data: &[&[u8]]) -> SerialReceiveBlock {
        let mut block = SerialReceiveBlock::new(
            "foo",
            &config.start_delimiter,
            &config.end_delimiter,
            config.read_bytes,
            1000.0,
        );
        let mut mock_proto = MockRead::new();
        let mut chunk = 0;
        let num_chunks = data.len();
        let data: Vec<Vec<u8>> = data.iter().map(|d| d.to_vec()).collect();
        mock_proto.expect_read().returning(move |buf| {
            let res = &data[chunk];
            for (i, c) in res.iter().enumerate() {
                buf[i] = *c;
            }

            chunk += 1;
            Ok(res.len())
        });
        for _ in 0..num_chunks {
            block.run(&mut mock_proto, 1.0);
        }
        block
    }

    fn test_serial_read(config: &TestBlockConfig, data: &[&[u8]], expected: &[u8]) {
        let block = run_serial_read(config, data);
        assert_eq!(block.data, BlockData::from_bytes(expected));

        let expect_valid = !expected.is_empty();
        assert_eq!(block.stale_check.is_valid(1.0).all(), expect_valid);
    }

    #[test]
    fn test_multi_end_delimited_string_no_start_delimiter() {
        test_serial_read_single_chunk(
            &TestBlockConfig {
                start_delimiter: String::from(""),
                end_delimiter: String::from("\r\n"),
                read_bytes: 0.,
            },
            b"AT+GMR\r\nAT version:1.6.0.0(Feb  3 2018 12:00:06)\r\nSDK version:2.2.1(f42c330)\r\ncompile time:Feb 12 2018 16:31:26\r\nBin version(Wroom 02):1.6.1\r\nOK\r\n",
            b"OK",
        );
    }

    #[test]
    fn test_hex_multi_end_delimited_string_no_start_delimiter() {
        test_serial_read_single_chunk(
            &TestBlockConfig {
                start_delimiter: String::from(""),
                end_delimiter: String::from(r"\x55\x55"),
                read_bytes: 0.,
            },
            b"\x55\x55\xAA\xBB\x55\x55\xCC\xDD\x55\x55\xEE\xFF\x55\x55\x00\x11\x55\x55",
            b"\x00\x11",
        );
    }

    #[test]
    fn test_read_simple_scalar() {
        test_serial_read_single_chunk(
            &TestBlockConfig {
                start_delimiter: String::from(""),
                end_delimiter: String::from(""),
                read_bytes: 0.,
            },
            b"42",
            b"42",
        );
    }

    #[test]
    fn test_read_end_delimited_scalar() {
        test_serial_read_single_chunk(
            &TestBlockConfig {
                start_delimiter: String::from(""),
                end_delimiter: String::from(";"),
                read_bytes: 0.,
            },
            b"54;55;somegarbage",
            b"55",
        );
    }

    #[test]
    fn test_read_start_delimited_scalar() {
        test_serial_read_single_chunk(
            &TestBlockConfig {
                start_delimiter: String::from("$"),
                end_delimiter: String::from(""),
                read_bytes: 0.,
            },
            b"stuff$42$9001",
            b"9001",
        );
    }

    #[test]
    fn test_read_start_and_end_delimited_scalar() {
        test_serial_read_single_chunk(
            &TestBlockConfig {
                start_delimiter: String::from("$"),
                end_delimiter: String::from("\r\n"),
                read_bytes: 0.,
            },
            b"$123\r\n$456\r\n$789",
            b"456",
        );
    }

    #[test]
    fn test_read_simple_vector() {
        test_serial_read_single_chunk(
            &TestBlockConfig {
                start_delimiter: String::from(""),
                end_delimiter: String::from(""),
                read_bytes: 0.,
            },
            b"42,43.0,44.4,45.5,46",
            b"42,43.0,44.4,45.5,46",
        );
    }

    #[test]
    fn test_read_end_delimited_vector() {
        test_serial_read_single_chunk(
            &TestBlockConfig {
                start_delimiter: String::from(""),
                end_delimiter: String::from("\n"),
                read_bytes: 0.,
            },
            b"1/2.0\n3.1/4.0/5\n6.6",
            b"3.1/4.0/5",
        );
    }

    #[test]
    fn test_read_start_delimited_vector() {
        test_serial_read_single_chunk(
            &TestBlockConfig {
                start_delimiter: String::from("?FOO:"),
                end_delimiter: String::from(""),
                read_bytes: 0.,
            },
            b"5+6?FOO:1+2?FOO:-3.0+4.0",
            b"-3.0+4.0",
        );
    }

    #[test]
    fn test_read_start_and_end_delimited_vector() {
        test_serial_read_single_chunk(
            &TestBlockConfig {
                start_delimiter: String::from("$GPGSA"),
                end_delimiter: String::from("\r\n"),
                read_bytes: 0.,
            },
            b"$GPGSA,A,3,10,07,05,02,29,04,08,13,,,,,1.72,1.03,1.38*0A\r\n$GPGGA,092750.000,5321.6802,N,00630.3372,W,1,8,1.03,61.7,M,55.2,M,,*76\r\n",
            b",A,3,10,07,05,02,29,04,08,13,,,,,1.72,1.03,1.38*0A",
        );
    }

    #[test]
    fn test_read_start_and_end_delimited_vector_missing_end() {
        test_serial_read_single_chunk(
            &TestBlockConfig {
                start_delimiter: String::from("$"),
                end_delimiter: String::from("?"),
                read_bytes: 0.,
            },
            b"$1$2$3",
            b"",
        );
    }

    #[test]
    fn test_read_start_and_end_delimited_vector_missing_start() {
        test_serial_read_single_chunk(
            &TestBlockConfig {
                start_delimiter: String::from("$"),
                end_delimiter: String::from("?"),
                read_bytes: 0.,
            },
            b"1?2?3?",
            b"",
        );
    }

    #[test]
    fn test_read_message_in_2_chunks() {
        test_serial_read(
            &TestBlockConfig {
                start_delimiter: String::from("$"),
                end_delimiter: String::from("\r\n"),
                read_bytes: 0.,
            },
            &[b"$hello", b"world\r\n"],
            b"helloworld",
        );
    }

    #[test]
    fn test_read_message_in_several_chunks() {
        test_serial_read(
            &TestBlockConfig {
                start_delimiter: String::from("$"),
                end_delimiter: String::from("\r\n"),
                read_bytes: 0.,
            },
            &[b"$", b"h", b"e", b"l", b"l", b"o", b"\r", b"\n"],
            b"hello",
        );
    }

    #[test]
    fn test_read_message_with_partial_message() {
        test_serial_read(
            &TestBlockConfig {
                start_delimiter: String::from("$"),
                end_delimiter: String::from("\r\n"),
                read_bytes: 0.,
            },
            &[b"$foo\r\n$ba", b"r\r\n"],
            b"bar",
        );
    }

    #[test]
    fn test_read_fixed_size_message_single_chunk() {
        test_serial_read_single_chunk(
            &TestBlockConfig {
                start_delimiter: String::from("$"),
                end_delimiter: String::new(),
                read_bytes: 5.,
            },
            b"$123456",
            b"12345",
        );
    }

    #[test]
    fn test_read_fixed_size_message_multi_chunk() {
        test_serial_read(
            &TestBlockConfig {
                start_delimiter: String::from("$"),
                end_delimiter: String::new(),
                read_bytes: 5.,
            },
            &[b"$12", b"3456"],
            b"12345",
        );
    }

    #[test]
    fn test_read_fixed_size_message_data_too_short() {
        test_serial_read_single_chunk(
            &TestBlockConfig {
                start_delimiter: String::from("$"),
                end_delimiter: String::new(),
                read_bytes: 5.,
            },
            b"$123",
            b"",
        );
    }

    #[test]
    fn test_read_fixed_size_message_start_delimiter_in_data() {
        test_serial_read_single_chunk(
            &TestBlockConfig {
                start_delimiter: String::from(r"\xAA\xAA"),
                end_delimiter: String::new(),
                read_bytes: 3.,
            },
            b"\xAA\xAA\xAB\xAC\xAA\xAA\xAA",
            b"\xAB\xAC\xAA",
        );
    }

    #[test]
    fn test_read_multiple_messages() {
        let mut block = run_serial_read(
            &TestBlockConfig {
                start_delimiter: String::from("$"),
                end_delimiter: String::from("\r\n"),
                read_bytes: 0.,
            },
            &[b"$123\r\n$456\r\n"],
        );
        assert_eq!(block.data, BlockData::from_bytes(b"456"));

        let mut mock = MockRead::new();
        mock.expect_read().returning(|_| Ok(0));
        block.run(&mut mock, 0.1);

        assert_eq!(block.data, BlockData::from_bytes(b"456"));
    }

    #[test]
    fn test_read_start_and_end_delimiter_and_size() {
        test_serial_read_single_chunk(
            &TestBlockConfig {
                start_delimiter: String::from(r"$"),
                end_delimiter: String::from("\r\n"),
                read_bytes: 3.,
            },
            b"$123",
            b"",
        );

        test_serial_read_single_chunk(
            &TestBlockConfig {
                start_delimiter: String::from(r"$"),
                end_delimiter: String::from("\r\n"),
                read_bytes: 8.,
            },
            b"$123\r\n456\r\n",
            b"123\r\n456",
        );

        test_serial_read_single_chunk(
            &TestBlockConfig {
                start_delimiter: String::from(r"$"),
                end_delimiter: String::from("\r\n"),
                read_bytes: 7.,
            },
            b"$123$456\r\n",
            b"123$456",
        );
    }

    #[test]
    fn test_read_data_with_wildcard_bytes() {
        test_serial_read_single_chunk(
            &TestBlockConfig {
                start_delimiter: String::from(r"\xAA\x**\xAB"),
                end_delimiter: String::from(r"\xAC\x**\xAD"),
                read_bytes: 0.,
            },
            b"\xFF\xFF\xAA\xFF\xAB\xF0\xF1\xF2\xAC\xFF\xAD\xFF\xFF",
            b"\xF0\xF1\xF2",
        );
    }
}
