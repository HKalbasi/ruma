//! `GET /_matrix/media/*/download/{serverName}/{mediaId}/{fileName}`
//!
//! Retrieve content from the media store, specifying a filename to return.

pub mod v3 {
    //! `/v3/` ([spec])
    //!
    //! [spec]: https://spec.matrix.org/latest/client-server-api/#get_matrixmediav3downloadservernamemediaidfilename

    use http::header::{CONTENT_DISPOSITION, CONTENT_TYPE};
    use ruma_common::{
        api::{request, response, Metadata},
        metadata, IdParseError, MxcUri, OwnedServerName,
    };

    use crate::http_headers::CROSS_ORIGIN_RESOURCE_POLICY;

    const METADATA: Metadata = metadata! {
        method: GET,
        rate_limited: false,
        authentication: None,
        history: {
            1.0 => "/_matrix/media/r0/download/:server_name/:media_id/:filename",
            1.1 => "/_matrix/media/v3/download/:server_name/:media_id/:filename",
        }
    };

    /// Request type for the `get_media_content_as_filename` endpoint.
    #[request(error = crate::Error)]
    pub struct Request {
        /// The server name from the mxc:// URI (the authoritory component).
        #[ruma_api(path)]
        pub server_name: OwnedServerName,

        /// The media ID from the mxc:// URI (the path component).
        #[ruma_api(path)]
        pub media_id: String,

        /// The filename to return in the `Content-Disposition` header.
        #[ruma_api(path)]
        pub filename: String,

        /// Whether to fetch media deemed remote.
        ///
        /// Used to prevent routing loops. Defaults to `true`.
        #[ruma_api(query)]
        #[serde(
            default = "ruma_common::serde::default_true",
            skip_serializing_if = "ruma_common::serde::is_true"
        )]
        pub allow_remote: bool,
    }

    /// Response type for the `get_media_content_as_filename` endpoint.
    #[response(error = crate::Error)]
    pub struct Response {
        /// The content that was previously uploaded.
        #[ruma_api(raw_body)]
        pub file: Vec<u8>,

        /// The content type of the file that was previously uploaded.
        #[ruma_api(header = CONTENT_TYPE)]
        pub content_type: Option<String>,

        /// The value of the `Content-Disposition` HTTP header, possibly containing the name of the
        /// file that was previously uploaded.
        ///
        /// See [MDN] for the syntax.
        ///
        /// [MDN]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Content-Disposition#Syntax
        #[ruma_api(header = CONTENT_DISPOSITION)]
        pub content_disposition: Option<String>,

        /// The value of the `Cross-Origin-Resource-Policy` HTTP header.
        ///
        /// See [MDN] for the syntax.
        ///
        /// [MDN]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Headers/Cross-Origin-Resource-Policy#syntax
        #[ruma_api(header = CROSS_ORIGIN_RESOURCE_POLICY)]
        pub cross_origin_resource_policy: Option<String>,
    }

    impl Request {
        /// Creates a new `Request` with the given media ID, server name and filename.
        pub fn new(media_id: String, server_name: OwnedServerName, filename: String) -> Self {
            Self { media_id, server_name, filename, allow_remote: true }
        }

        /// Creates a new `Request` with the given url and filename.
        pub fn from_url(url: &MxcUri, filename: String) -> Result<Self, IdParseError> {
            let (server_name, media_id) = url.parts()?;

            Ok(Self {
                media_id: media_id.to_owned(),
                server_name: server_name.to_owned(),
                filename,
                allow_remote: true,
            })
        }
    }

    impl Response {
        /// Creates a new `Response` with the given file.
        ///
        /// The Cross-Origin Resource Policy defaults to `cross-origin`.
        pub fn new(file: Vec<u8>) -> Self {
            Self {
                file,
                content_type: None,
                content_disposition: None,
                cross_origin_resource_policy: Some("cross-origin".to_owned()),
            }
        }
    }
}
