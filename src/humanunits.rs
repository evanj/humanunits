use std::time::Duration;

struct ByteUnit {
    unit_label: &'static str,
    bytes: usize,
}

impl ByteUnit {
    const UNIT_LABELS: [&'static str; 3] = ["KiB", "MiB", "GiB"];

    fn all() -> Vec<Self> {
        let mut result = Vec::with_capacity(Self::UNIT_LABELS.len());
        for (i, unit_label) in Self::UNIT_LABELS.iter().enumerate() {
            let bytes = 1 << ((i + 1) * 10);
            result.push(Self { unit_label, bytes });
        }
        result
    }
}

/// Returns a human-readable rate in the format MiB/sec by dividing bytes by duration.
#[must_use]
pub fn byte_rate_string(bytes: usize, duration: Duration) -> String {
    let bytes_per_sec = (bytes as f64 / duration.as_secs_f64() + 0.5) as usize;
    let mut result = bytes_string(bytes_per_sec);
    result.push_str("/sec");
    result
}

#[must_use]
pub fn bytes_string(bytes: usize) -> String {
    for unit in ByteUnit::all().iter().rev() {
        if bytes >= unit.bytes {
            let unit_v = bytes as f64 / unit.bytes as f64;
            return format!("{unit_v:.1} {}", unit.unit_label);
        }
    }
    format!("{bytes} bytes")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_bytes_string() {
        assert_eq!("1.0 GiB", bytes_string(1 << 30));
    }

    #[test]
    fn test_byte_rate_string() {
        assert_eq!(
            "1.0 GiB/sec",
            byte_rate_string(1 << 30, Duration::from_secs(1))
        );
    }
}
