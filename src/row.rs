use crate::{
    constant::{
        EMAIL_OFFSET, EMAIL_SIZE, ID_OFFSET, ID_SIZE, ROW_SIZE, USERNAME_OFFSET, USERNAME_SIZE,
    },
    utils::pad_or_truncate,
};

#[derive(Debug, PartialEq)]
pub struct Row {
    pub id: u32,
    pub username: [u8; USERNAME_SIZE],
    pub email: [u8; EMAIL_SIZE],
}

fn serialize_row(source: &Row, destination: &mut [u8]) {
    assert!(destination.len() >= ROW_SIZE, "Buffer too small",);

    destination[ID_OFFSET..ID_OFFSET + ID_SIZE].copy_from_slice(&source.id.to_le_bytes());

    destination[USERNAME_OFFSET..USERNAME_OFFSET + USERNAME_SIZE].copy_from_slice(&source.username);

    destination[EMAIL_OFFSET..EMAIL_OFFSET + EMAIL_SIZE].copy_from_slice(&source.email);
}

fn deserialize_row(source: &[u8]) -> Row {
    assert!(
        source.len() >= ROW_SIZE,
        "Source buffer too small: {:?}, {:?}",
        source.len(),
        ROW_SIZE
    );

    let id_bytes = &source[ID_OFFSET..ID_OFFSET + ID_SIZE];
    return Row {
        id: u32::from_le_bytes(id_bytes.try_into().unwrap()),
        username: pad_or_truncate(
            &source[USERNAME_OFFSET..USERNAME_OFFSET + USERNAME_SIZE],
            true,
        ),
        email: pad_or_truncate(&source[EMAIL_OFFSET..EMAIL_OFFSET + EMAIL_SIZE], true),
    };
}

#[cfg(test)]
mod tests {
    use crate::utils::pad_or_truncate;

    use super::*;

    #[test]
    fn test_serialize_row() {
        // Create a sample row
        let row = Row {
            id: 1,
            username: pad_or_truncate::<USERNAME_SIZE>("test".as_bytes(), true),
            email: pad_or_truncate::<EMAIL_SIZE>("test@example.com".as_bytes(), true),
        };

        // Spot check null termination
        assert_eq!(row.username[4], 0); // After "test"
        assert_eq!(row.email[13], 0); // After "test@example.com"

        // Create a buffer with the exact size needed
        let mut buffer = vec![0; ROW_SIZE];

        // Serialize the row into the buffer
        serialize_row(&row, &mut buffer);

        // Verify the serialized data
        // This depends on your serialization format, but for example:
        let mut expected = Vec::new();
        expected.extend(&row.id.to_ne_bytes());
        expected.extend(row.username);
        expected.extend(&[0]); // null terminator if using C-style strings
        expected.extend(row.email);
        expected.extend(&[0]); // null terminator

        assert_eq!(buffer, expected.as_slice());
    }

    #[test]
    fn test_serialize_empty_strings() {
        let row = Row {
            id: 1,
            username: pad_or_truncate("".as_bytes(), true),
            email: pad_or_truncate("".as_bytes(), true),
        };
        let mut buffer = vec![0; ROW_SIZE];
        serialize_row(&row, &mut buffer);
        // Verify empty strings are handled correctly
        let mut expected = Vec::new();
        expected.extend(&row.id.to_ne_bytes());
        expected.extend(row.username);
        expected.extend(&[0]); // null terminator if using C-style strings
        expected.extend(row.email);
        expected.extend(&[0]); // null terminator

        assert_eq!(buffer, expected.as_slice());
    }

    #[test]
    #[should_panic(expected = "Buffer too small")]
    fn test_buffer_too_small() {
        let row = Row {
            id: 1,
            username: pad_or_truncate("test".as_bytes(), true),
            email: pad_or_truncate("test@example.com".as_bytes(), true),
        };
        let mut buffer = vec![0; ROW_SIZE - 1];
        serialize_row(&row, &mut buffer);
    }

    #[test]
    fn test_serialize_deserialize_roundtrip() {
        let original = Row {
            id: 1,
            username: pad_or_truncate("test".as_bytes(), true),
            email: pad_or_truncate("test@example.com".as_bytes(), true),
        };
        let mut buffer = vec![0; ROW_SIZE];
        serialize_row(&original, &mut buffer);

        let deserialized = deserialize_row(&buffer);
        assert_eq!(original, deserialized);
    }
}
