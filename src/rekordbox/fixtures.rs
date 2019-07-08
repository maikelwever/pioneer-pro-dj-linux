use bytes::Bytes;
use super::packets::DBMessage;
use nom::IResult;

type DBMessageParseResult<'a> = IResult<&'a [u8], DBMessage>;
type DBMessageParse<'a> = DBMessage;

/// (TypeRequest, TypeResponse, RenderRequest, RenderResponse)
type IntegrationTestDialog = (Bytes, Bytes, Bytes, Bytes);


pub fn title_by_artist_album_dialog() -> IntegrationTestDialog {
    (
        Bytes::from(vec![
            0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0x05, 0x80,
            0x00, 0x16, 0x10, 0x12, 0x02, 0x0f, 0x04, 0x14,
            0x00, 0x00, 0x00, 0x0c, 0x06, 0x06, 0x06, 0x06,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,

            0x11, 0x02, 0x02, 0x04, 0x01, // player metadata
            0x11, 0x00, 0x00, 0x00, 0x00, // sort_id
            0x11, 0x00, 0x00, 0x00, 0x01, // item_id
            0x11, 0x00, 0x00, 0x00, 0x00  // item_id
        ]),
        Bytes::from(vec![
            0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0x05, 0x80,
            0x00, 0x16, 0x10, 0x40, 0x00, 0x0f, 0x02, 0x14,
            0x00, 0x00, 0x00, 0x0c, 0x06, 0x06, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x11, 0x00, 0x00, 0x12, 0x02, 0x11, 0x00, 0x00,
            0x00, 0x01 // TODO: This seems to be non-fixed value, change to 0x02 later
        ]),
        Bytes::from(vec![
            0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0x05, 0x80,
            0x00, 0x17, 0x10, 0x30, 0x00, 0x0f, 0x06, 0x14,
            0x00, 0x00, 0x00, 0x0c, 0x06, 0x06, 0x06, 0x06,
            0x06, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x11, 0x02, 0x02, 0x04, 0x01, 0x11, 0x00, 0x00,
            0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x02, 0x11,
            0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00,
            0x02, 0x11, 0x00, 0x00, 0x00, 0x00
        ]),
        Bytes::from(vec![
            0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0x05, 0x80,
            0x00, 0x17, 0x10, 0x40, 0x01, 0x0f, 0x02, 0x14,
            0x00, 0x00, 0x00, 0x0c, 0x06, 0x06, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x11, 0x00, 0x00, 0x00, 0x01, 0x11, 0x00, 0x00,
            0x00, 0x00, 0x11, 0x87, 0x23, 0x49, 0xae, 0x11,
            0x05, 0x80, 0x00, 0x17, 0x10, 0x41, 0x01, 0x0f,
            0x0c, 0x14, 0x00, 0x00, 0x00, 0x0c, 0x06, 0x06,
            0x06, 0x02, 0x06, 0x02, 0x06, 0x06, 0x06, 0x06,
            0x06, 0x06, 0x11, 0x00, 0x00, 0x00, 0x00, 0x11,
            0x00, 0x00, 0x00, 0x05, 0x11, 0x00, 0x00, 0x00,
            0x1a, 0x26, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x44,
            0x00, 0x65, 0x00, 0x6d, 0x00, 0x6f, 0x00, 0x20,
            0x00, 0x54, 0x00, 0x72, 0x00, 0x61, 0x00, 0x63,
            0x00, 0x6b, 0x00, 0x20, 0x00, 0x31, 0x00, 0x00,
            0x11, 0x00, 0x00, 0x00, 0x02, 0x26, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00,
            0x04, 0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00,
            0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00,
            0x11, 0x00, 0x00, 0x01, 0x00, 0x11, 0x00, 0x00,
            0x00, 0x00, 0x11, 0x87, 0x23, 0x49, 0xae, 0x11,
            0x05, 0x80, 0x00, 0x17, 0x10, 0x41, 0x01, 0x0f,
            0x0c, 0x14, 0x00, 0x00, 0x00, 0x0c, 0x06, 0x06,
            0x06, 0x02, 0x06, 0x02, 0x06, 0x06, 0x06, 0x06,
            0x06, 0x06, 0x11, 0x00, 0x00, 0x00, 0x00, 0x11,
            0x00, 0x00, 0x00, 0x06, 0x11, 0x00, 0x00, 0x00,
            0x1a, 0x26, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x44,
            0x00, 0x65, 0x00, 0x6d, 0x00, 0x6f, 0x00, 0x20,
            0x00, 0x54, 0x00, 0x72, 0x00, 0x61, 0x00, 0x63,
            0x00, 0x6b, 0x00, 0x20, 0x00, 0x32, 0x00, 0x00,
            0x11, 0x00, 0x00, 0x00, 0x02, 0x26, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00,
            0x04, 0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00,
            0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00,
            0x11, 0x00, 0x00, 0x01, 0x00, 0x11, 0x00, 0x00,
            0x00, 0x00, 0x11, 0x87, 0x23, 0x49, 0xae, 0x11,
            0x05, 0x80, 0x00, 0x17, 0x10, 0x42, 0x01, 0x0f,
            0x00, 0x14, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00
        ]),
    )
}

pub fn album_by_artist_dialog() -> IntegrationTestDialog {
    (
        Bytes::from(vec![
            0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0x05, 0x80,
            0x00, 0x14, 0x10, 0x11, 0x02, 0x0f, 0x03, 0x14,
            0x00, 0x00, 0x00, 0x0c, 0x06, 0x06, 0x06, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,

            0x11, 0x02, 0x02, 0x04, 0x01, // player metadata
            0x11, 0x00, 0x00, 0x00, 0x00, // sort_id
            0x11, 0x00, 0x00, 0x00, 0x01, // item_id
        ]),
        Bytes::from(vec![
            0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0x05, 0x80,
            0x00, 0x14, 0x10, 0x40, 0x00, 0x0f, 0x02, 0x14,
            0x00, 0x00, 0x00, 0x0c, 0x06, 0x06, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x11, 0x00, 0x00, 0x11, 0x02, 0x11, 0x00, 0x00,
            0x00, 0x01
        ]),
        Bytes::from(vec![
            0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0x05, 0x80,
            0x00, 0x15, 0x10, 0x30, 0x00, 0x0f, 0x06, 0x14,
            0x00, 0x00, 0x00, 0x0c, 0x06, 0x06, 0x06, 0x06,
            0x06, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x11, 0x02, 0x02, 0x04, 0x01, 0x11, 0x00, 0x00,
            0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x01, 0x11,
            0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00,
            0x01, 0x11, 0x00, 0x00, 0x00, 0x00
        ]),
        Bytes::from(vec![
            0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0x05, 0x80,
            0x00, 0x15, 0x10, 0x40, 0x01, 0x0f, 0x02, 0x14,
            0x00, 0x00, 0x00, 0x0c, 0x06, 0x06, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x11, 0x00, 0x00, 0x00, 0x01, 0x11, 0x00, 0x00,
            0x00, 0x00, 0x11, 0x87, 0x23, 0x49, 0xae, 0x11,
            0x05, 0x80, 0x00, 0x15, 0x10, 0x41, 0x01, 0x0f,
            0x0c, 0x14, 0x00, 0x00, 0x00, 0x0c, 0x06, 0x06,
            0x06, 0x02, 0x06, 0x02, 0x06, 0x06, 0x06, 0x06,
            0x06, 0x06, 0x11, 0x00, 0x00, 0x00, 0x00, 0x11,
            0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00,
            0x10, 0x26, 0x00, 0x00, 0x00, 0x08, 0x00, 0x55,
            0x00, 0x6e, 0x00, 0x6b, 0x00, 0x6e, 0x00, 0x6f,
            0x00, 0x77, 0x00, 0x6e, 0x00, 0x00, 0x11, 0x00,
            0x00, 0x00, 0x02, 0x26, 0x00, 0x00, 0x00, 0x01,
            0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x02, 0x11,
            0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00,
            0x00, 0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00,
            0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00,
            0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0x05, 0x80,
            0x00, 0x15, 0x10, 0x42, 0x01, 0x0f, 0x00, 0x14,
            0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
        ]),
    )
}

pub fn setup_request_packet<'a>() -> DBMessageParseResult<'a> {
    DBMessage::parse(&[
        0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0xff, 0xff,
        0xff, 0xfe, 0x10, 0x00, 0x00, 0x0f, 0x01, 0x14,
        0x00, 0x00, 0x00, 0x0c, 0x06, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x11, 0x00, 0x00, 0x00, 0x02,
    ])
}

pub fn setup_response_packet() -> Bytes {
    Bytes::from(vec![
        0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0xff, 0xff,
        0xff, 0xfe, 0x10, 0x40, 0x00, 0x0f, 0x02, 0x14,
        0x00, 0x00, 0x00, 0x0c, 0x06, 0x06, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00,
        0x00, 0x11,
    ])
}

pub fn root_menu_request<'a>() -> DBMessageParseResult<'a> {
    DBMessage::parse(&[
        0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0x05, 0x80,
        0x00, 0x32, 0x10, 0x10, 0x00, 0x0f, 0x03, 0x14,
        0x00, 0x00, 0x00, 0x0c, 0x06, 0x06, 0x06, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x11, 0x02, 0x01, 0x04, 0x01, 0x11, 0x00, 0x00,
        0x00, 0x00, 0x11, 0x00, 0xff, 0xff, 0xff,
    ])
}

pub fn root_menu_response_packet() -> Bytes {
    Bytes::from(vec![
        0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0x05, 0x80,
        0x00, 0x32, 0x10, 0x40, 0x00, 0x0f, 0x02, 0x14,
        0x00, 0x00, 0x00, 0x0c, 0x06, 0x06, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x11, 0x00, 0x00, 0x10, 0x00, 0x11, 0x00, 0x00,
        0x00, 0x08,
    ])
}

pub fn artist_request_type<'a>() -> DBMessageParse<'a> {
    DBMessage::parse(&[
        0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0x05, 0x80, 0x00, 0x10, 0x10, 0x10, 0x02, 0x0f, 0x02, 0x14,
        0x00, 0x00, 0x00, 0x0c, 0x06, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x11, 0x02, 0x02, 0x04, 0x01, 0x11, 0x00, 0x00, 0x00, 0x00
    ]).unwrap().1
}

pub fn render_root_menu_request<'a>() -> Vec<u8> {
    vec![
        0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0x05, 0x80, 0x00, 0x0f, 0x10, 0x30, 0x00, 0x0f, 0x06, 0x14,
        0x00, 0x00, 0x00, 0x0c, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x11, 0x02, 0x01, 0x04, 0x01, 0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x07, 0x11,
        0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x08, 0x11, 0x00, 0x00, 0x00, 0x00,
    ]
}

pub fn artist_dialog() -> (Bytes, Bytes, Bytes, Bytes) {
    (
        artist_request(),
        artist_response(),
        render_artist_request(),
        render_artist_response(),
    )
}

pub fn artist_request() -> Bytes {
    Bytes::from(vec![
        0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0x05, 0x80, 0x00, 0x10, 0x10, 0x10, 0x02, 0x0f, 0x02, 0x14,
        0x00, 0x00, 0x00, 0x0c, 0x06, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,

        0x11, 0x02, 0x02, 0x04, 0x01,
        0x11, 0x00, 0x00, 0x00, 0x00
    ])
}

pub fn artist_response() -> Bytes {
    Bytes::from(vec![
        0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0x05, 0x80, 0x00, 0x10, 0x10, 0x40, 0x00, 0x0f, 0x02, 0x14,
        0x00, 0x00, 0x00, 0x0c, 0x06, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x11, 0x00, 0x00, 0x10, 0x02, 0x11, 0x00, 0x00, 0x00, 0x01,
    ])
}

pub fn render_artist_request() -> Bytes {
    Bytes::from(vec![
        0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0x05, 0x80, 0x00, 0x41, 0x10, 0x30, 0x00, 0x0f, 0x06, 0x14,
        0x00, 0x00, 0x00, 0x0c, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x11, 0x02, 0x02, 0x04, 0x01, 0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x01, 0x11,
        0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x01, 0x11, 0x00, 0x00, 0x00, 0x00,
    ])
}

pub fn render_artist_response() -> Bytes {
    Bytes::from(vec![
        0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0x05, 0x80, 0x00, 0x41, 0x10, 0x40, 0x01, 0x0f, 0x02, 0x14,
        0x00, 0x00, 0x00, 0x0c, 0x06, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x11, 0x00, 0x00, 0x00, 0x01, 0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x87, 0x23, 0x49, 0xae, 0x11,
        0x05, 0x80, 0x00, 0x41, 0x10, 0x41, 0x01, 0x0f, 0x0c, 0x14, 0x00, 0x00, 0x00, 0x0c, 0x06, 0x06,
        0x06, 0x02, 0x06, 0x02, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06, 0x11, 0x00, 0x00, 0x00, 0x00, 0x11,
        0x00, 0x00, 0x00, 0x01, 0x11, 0x00, 0x00, 0x00, 0x18, 0x26, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x4c,
        0x00, 0x6f, 0x00, 0x6f, 0x00, 0x70, 0x00, 0x6d, 0x00, 0x61, 0x00, 0x73, 0x00, 0x74, 0x00, 0x65,
        0x00, 0x72, 0x00, 0x73, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x02, 0x26, 0x00, 0x00, 0x00, 0x01,
        0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x07, 0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00,
        0x00, 0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00,
        0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0x05, 0x80, 0x00, 0x41, 0x10, 0x42, 0x01, 0x0f, 0x00, 0x14,
        0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00
    ])
}

pub fn raw_menu_footer_request() -> Bytes {
    Bytes::from(vec![
        0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0x05, 0x80, 0x00, 0x0f, 0x10, 0x42, 0x01, 0x0f,
        0x00, 0x14, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
    ])
}

pub fn metadata_dialog() -> IntegrationTestDialog {
    (
        Bytes::from(vec![
            0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0x05, 0x80,
            0x00, 0x1f, 0x10, 0x20, 0x02, 0x0f, 0x02, 0x14,
            0x00, 0x00, 0x00, 0x0c, 0x06, 0x06, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x11, 0x02, 0x01, 0x04, 0x01, 0x11, 0x00, 0x00,
            0x00, 0x05
        ]),
        Bytes::from(vec![
            0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0x05, 0x80,
            0x00, 0x1f, 0x10, 0x40, 0x00, 0x0f, 0x02, 0x14,
            0x00, 0x00, 0x00, 0x0c, 0x06, 0x06, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x11, 0x00, 0x00, 0x20, 0x02, 0x11, 0x00, 0x00,
            0x00, 0x01
        ]),
        Bytes::from(vec![
            0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0x05, 0x80,
            0x00, 0x20, 0x10, 0x30, 0x00, 0x0f, 0x06, 0x14,
            0x00, 0x00, 0x00, 0x0c, 0x06, 0x06, 0x06, 0x06,
            0x06, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x11, 0x02, 0x01, 0x04, 0x01, 0x11, 0x00, 0x00,
            0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x01, 0x11,
            0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00,
            0x01, 0x11, 0x00, 0x00, 0x00, 0x00
        ]),
        Bytes::from(vec![
            0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0x05, 0x80,
            0x00, 0x20, 0x10, 0x40, 0x01, 0x0f, 0x02, 0x14,
            0x00, 0x00, 0x00, 0x0c, 0x06, 0x06, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x11, 0x00, 0x00, 0x00, 0x01, 0x11, 0x00, 0x00,
            0x00, 0x00, 0x11, 0x87, 0x23, 0x49, 0xae, 0x11,
            0x05, 0x80, 0x00, 0x20, 0x10, 0x41, 0x01, 0x0f,
            0x0c, 0x14, 0x00, 0x00, 0x00, 0x0c, 0x06, 0x06,
            0x06, 0x02, 0x06, 0x02, 0x06, 0x06, 0x06, 0x06,
            0x06, 0x06, 0x11, 0x00, 0x00, 0x00, 0x01, 0x11,
            0x00, 0x00, 0x00, 0x05, 0x11, 0x00, 0x00, 0x00,
            0x1a, 0x26, 0x00, 0x00, 0x00, 0x0d, 0x00, 0x44,
            0x00, 0x65, 0x00, 0x6d, 0x00, 0x6f, 0x00, 0x20,
            0x00, 0x54, 0x00, 0x72, 0x00, 0x61, 0x00, 0x63,
            0x00, 0x6b, 0x00, 0x20, 0x00, 0x31, 0x00, 0x00,
            0x11, 0x00, 0x00, 0x00, 0x02, 0x26, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00,
            0x04, 0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00,
            0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00,
            0x11, 0x00, 0x00, 0x01, 0x00, 0x11, 0x00, 0x00,
            0x00, 0x00, 0x11, 0x87, 0x23, 0x49, 0xae, 0x11,
            0x05, 0x80, 0x00, 0x20, 0x10, 0x41, 0x01, 0x0f,
            0x0c, 0x14, 0x00, 0x00, 0x00, 0x0c, 0x06, 0x06,
            0x06, 0x02, 0x06, 0x02, 0x06, 0x06, 0x06, 0x06,
            0x06, 0x06, 0x11, 0x00, 0x00, 0x00, 0x01, 0x11,
            0x00, 0x00, 0x00, 0x01, 0x11, 0x00, 0x00, 0x00,
            0x18, 0x26, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x4c,
            0x00, 0x6f, 0x00, 0x6f, 0x00, 0x70, 0x00, 0x6d,
            0x00, 0x61, 0x00, 0x73, 0x00, 0x74, 0x00, 0x65,
            0x00, 0x72, 0x00, 0x73, 0x00, 0x00, 0x11, 0x00,
            0x00, 0x00, 0x02, 0x26, 0x00, 0x00, 0x00, 0x01,
            0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x07, 0x11,
            0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00,
            0x00, 0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00,
            0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00,
            0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0x05, 0x80,
            0x00, 0x20, 0x10, 0x41, 0x01, 0x0f, 0x0c, 0x14,
            0x00, 0x00, 0x00, 0x0c, 0x06, 0x06, 0x06, 0x02,
            0x06, 0x02, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06,
            0x11, 0x00, 0x00, 0x00, 0x01, 0x11, 0x00, 0x00,
            0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x02, 0x26,
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x11, 0x00,
            0x00, 0x00, 0x02, 0x26, 0x00, 0x00, 0x00, 0x01,
            0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x02, 0x11,
            0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00,
            0x00, 0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00,
            0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00,
            0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0x05, 0x80,
            0x00, 0x20, 0x10, 0x41, 0x01, 0x0f, 0x0c, 0x14,
            0x00, 0x00, 0x00, 0x0c, 0x06, 0x06, 0x06, 0x02,
            0x06, 0x02, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06,
            0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00,
            0x00, 0xac, 0x11, 0x00, 0x00, 0x00, 0x02, 0x26,
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x11, 0x00,
            0x00, 0x00, 0x02, 0x26, 0x00, 0x00, 0x00, 0x01,
            0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x0b, 0x11,
            0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00,
            0x00, 0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00,
            0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00,
            0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0x05, 0x80,
            0x00, 0x20, 0x10, 0x41, 0x01, 0x0f, 0x0c, 0x14,
            0x00, 0x00, 0x00, 0x0c, 0x06, 0x06, 0x06, 0x02,
            0x06, 0x02, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06,
            0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00,
            0x32, 0x00, 0x11, 0x00, 0x00, 0x00, 0x02, 0x26,
            0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x11, 0x00,
            0x00, 0x00, 0x02, 0x26, 0x00, 0x00, 0x00, 0x01,
            0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x0d, 0x11,
            0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00,
            0x00, 0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00,
            0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00,
            0x11, 0x87, 0x23, 0x49, 0xae, 0x11, 0x05, 0x80,
            0x00, 0x20, 0x10, 0x41, 0x01, 0x0f, 0x0c, 0x14,
            0x00, 0x00, 0x00, 0x0c, 0x06, 0x06, 0x06, 0x02,
            0x06, 0x02, 0x06, 0x06, 0x06, 0x06, 0x06, 0x06,
            0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00,
            0x00, 0x05, 0x11, 0x00, 0x00, 0x00, 0x3c, 0x26,
            0x00, 0x00, 0x00, 0x1e, 0x00, 0x54, 0x00, 0x72,
            0x00, 0x61, 0x00, 0x63, 0x00, 0x6b, 0x00, 0x73,
            0x00, 0x20, 0x00, 0x62, 0x00, 0x79, 0x00, 0x20,
            0x00, 0x77, 0x00, 0x77, 0x00, 0x77, 0x00, 0x2e,
            0x00, 0x6c, 0x00, 0x6f, 0x00, 0x6f, 0x00, 0x70,
            0x00, 0x6d, 0x00, 0x61, 0x00, 0x73, 0x00, 0x74,
            0x00, 0x65, 0x00, 0x72, 0x00, 0x73, 0x00, 0x2e,
            0x00, 0x63, 0x00, 0x6f, 0x00, 0x6d, 0x00, 0x00,
            0x11, 0x00, 0x00, 0x00, 0x02, 0x26, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00,
            0x23, 0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00,
            0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00,
            0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00,
            0x00, 0x00, 0x11, 0x87, 0x23, 0x49, 0xae, 0x11,
            0x05, 0x80, 0x00, 0x20, 0x10, 0x41, 0x01, 0x0f,
            0x0c, 0x14, 0x00, 0x00, 0x00, 0x0c, 0x06, 0x06,
            0x06, 0x02, 0x06, 0x02, 0x06, 0x06, 0x06, 0x06,
            0x06, 0x06, 0x11, 0x00, 0x00, 0x00, 0x01, 0x11,
            0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00,
            0x02, 0x26, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
            0x11, 0x00, 0x00, 0x00, 0x02, 0x26, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00,
            0x0f, 0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00,
            0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00,
            0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00,
            0x00, 0x00, 0x11, 0x87, 0x23, 0x49, 0xae, 0x11,
            0x05, 0x80, 0x00, 0x20, 0x10, 0x41, 0x01, 0x0f,
            0x0c, 0x14, 0x00, 0x00, 0x00, 0x0c, 0x06, 0x06,
            0x06, 0x02, 0x06, 0x02, 0x06, 0x06, 0x06, 0x06,
            0x06, 0x06, 0x11, 0x00, 0x00, 0x00, 0x00, 0x11,
            0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00,
            0x02, 0x26, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
            0x11, 0x00, 0x00, 0x00, 0x02, 0x26, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00,
            0x0a, 0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00,
            0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00,
            0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00,
            0x00, 0x00, 0x11, 0x87, 0x23, 0x49, 0xae, 0x11,
            0x05, 0x80, 0x00, 0x20, 0x10, 0x41, 0x01, 0x0f,
            0x0c, 0x14, 0x00, 0x00, 0x00, 0x0c, 0x06, 0x06,
            0x06, 0x02, 0x06, 0x02, 0x06, 0x06, 0x06, 0x06,
            0x06, 0x06, 0x11, 0x00, 0x00, 0x00, 0x00, 0x11,
            0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00,
            0x02, 0x26, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
            0x11, 0x00, 0x00, 0x00, 0x02, 0x26, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00,
            0x13, 0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00,
            0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00,
            0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00,
            0x00, 0x00, 0x11, 0x87, 0x23, 0x49, 0xae, 0x11,
            0x05, 0x80, 0x00, 0x20, 0x10, 0x41, 0x01, 0x0f,
            0x0c, 0x14, 0x00, 0x00, 0x00, 0x0c, 0x06, 0x06,
            0x06, 0x02, 0x06, 0x02, 0x06, 0x06, 0x06, 0x06,
            0x06, 0x06, 0x11, 0x00, 0x00, 0x00, 0x00, 0x11,
            0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00,
            0x02, 0x26, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
            0x11, 0x00, 0x00, 0x00, 0x02, 0x26, 0x00, 0x00,
            0x00, 0x01, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00,
            0x06, 0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00,
            0x00, 0x00, 0x00, 0x11, 0x00, 0x00, 0x00, 0x00,
            0x11, 0x00, 0x00, 0x00, 0x00, 0x11, 0x00, 0x00,
            0x00, 0x00, 0x11, 0x87, 0x23, 0x49, 0xae, 0x11,
            0x05, 0x80, 0x00, 0x20, 0x10, 0x42, 0x01, 0x0f,
            0x00, 0x14, 0x00, 0x00, 0x00, 0x0c, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00
        ]),
    )
}
