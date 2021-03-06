/// The message used by the client to register interest in an entity.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    /// The `target` value **must** be a valid URL path pointing to an entity
    /// to watch. Note that the service name **must** be
    /// removed from the target field (e.g., the target field must say
    /// "/foo/bar", not "myservice.googleapis.com/foo/bar"). A client is
    /// also allowed to pass system-specific parameters in the URL that
    /// are only obeyed by some implementations. Some parameters will be
    /// implementation-specific. However, some have predefined meaning
    /// and are listed here:
    ///
    ///  * recursive = true|false [default=false]
    ///    If set to true, indicates that the client wants to watch all elements
    ///    of entities in the subtree rooted at the entity's name in `target`. For
    ///    descendants that are not the immediate children of the target, the
    ///    `Change.element` will contain slashes.
    ///
    ///    Note that some namespaces and entities will not support recursive
    ///    watching. When watching such an entity, a client must not set recursive
    ///    to true. Otherwise, it will receive an `UNIMPLEMENTED` error.
    ///
    /// Normal URL encoding must be used inside `target`.  For example, if a query
    /// parameter name or value, or the non-query parameter portion of `target`
    /// contains a special character, it must be %-encoded.  We recommend that
    /// clients and servers use their runtime's URL library to produce and consume
    /// target values.
    #[prost(string, tag = "1")]
    pub target: std::string::String,
    /// The `resume_marker` specifies how much of the existing underlying state is
    /// delivered to the client when the watch request is received by the
    /// system. The client can set this marker in one of the following ways to get
    /// different semantics:
    ///
    /// *   Parameter is not specified or has the value "".
    ///     Semantics: Fetch initial state.
    ///     The client wants the entity's initial state to be delivered. See the
    ///     description in "Initial State".
    ///
    /// *   Parameter is set to the string "now" (UTF-8 encoding).
    ///     Semantics: Fetch new changes only.
    ///     The client just wants to get the changes received by the system after
    ///     the watch point. The system may deliver changes from before the watch
    ///     point as well.
    ///
    /// *   Parameter is set to a value received in an earlier
    ///     `Change.resume_marker` field while watching the same entity.
    ///     Semantics: Resume from a specific point.
    ///     The client wants to receive the changes from a specific point; this
    ///     value must correspond to a value received in the `Change.resume_marker`
    ///     field. The system may deliver changes from before the `resume_marker`
    ///     as well. If the system cannot resume the stream from this point (e.g.,
    ///     if it is too far behind in the stream), it can raise the
    ///     `FAILED_PRECONDITION` error.
    ///
    /// An implementation MUST support an unspecified parameter and the
    /// empty string "" marker (initial state fetching) and the "now" marker.
    /// It need not support resuming from a specific point.
    #[prost(bytes, tag = "2")]
    pub resume_marker: std::vec::Vec<u8>,
}
/// A batch of Change messages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeBatch {
    /// A list of Change messages.
    #[prost(message, repeated, tag = "1")]
    pub changes: ::std::vec::Vec<Change>,
}
/// A Change indicates the most recent state of an element.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Change {
    /// Name of the element, interpreted relative to the entity's actual
    /// name. "" refers to the entity itself. The element name is a valid
    /// UTF-8 string.
    #[prost(string, tag = "1")]
    pub element: std::string::String,
    /// The state of the `element`.
    #[prost(enumeration = "change::State", tag = "2")]
    pub state: i32,
    /// The actual change data. This field is present only when `state() == EXISTS`
    /// or `state() == ERROR`. Please see
    /// [google.protobuf.Any][google.protobuf.Any] about how to use the Any type.
    #[prost(message, optional, tag = "6")]
    pub data: ::std::option::Option<::prost_types::Any>,
    /// If present, provides a compact representation of all the messages that have
    /// been received by the caller for the given entity, e.g., it could be a
    /// sequence number or a multi-part timestamp/version vector. This marker can
    /// be provided in the Request message, allowing the caller to resume the
    /// stream watching at a specific point without fetching the initial state.
    #[prost(bytes, tag = "4")]
    pub resume_marker: std::vec::Vec<u8>,
    /// If true, this Change is followed by more Changes that are in the same group
    /// as this Change.
    #[prost(bool, tag = "5")]
    pub continued: bool,
}
pub mod change {
    /// A reported value can be in one of the following states:
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// The element exists and its full value is included in data.
        Exists = 0,
        /// The element does not exist.
        DoesNotExist = 1,
        /// Element may or may not exist. Used only for initial state delivery when
        /// the client is not interested in fetching the initial state. See the
        /// "Initial State" section above.
        InitialStateSkipped = 2,
        /// The element may exist, but some error has occurred. More information is
        /// available in the data field - the value is a serialized Status
        /// proto (from [google.rpc.Status][])
        Error = 3,
    }
}
#[doc = r" Generated client implementations."]
pub mod watcher_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The service that a client uses to connect to the watcher system."]
    #[doc = " The errors returned by the service are in the canonical error space,"]
    #[doc = " see [google.rpc.Code][]."]
    pub struct WatcherClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl WatcherClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> WatcherClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        #[doc = " Start a streaming RPC to get watch information from the server."]
        pub async fn watch(
            &mut self,
            request: impl tonic::IntoRequest<super::Request>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::ChangeBatch>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/google.watcher.v1.Watcher/Watch");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for WatcherClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for WatcherClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "WatcherClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod watcher_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with WatcherServer."]
    #[async_trait]
    pub trait Watcher: Send + Sync + 'static {
        #[doc = "Server streaming response type for the Watch method."]
        type WatchStream: Stream<Item = Result<super::ChangeBatch, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " Start a streaming RPC to get watch information from the server."]
        async fn watch(
            &self,
            request: tonic::Request<super::Request>,
        ) -> Result<tonic::Response<Self::WatchStream>, tonic::Status>;
    }
    #[doc = " The service that a client uses to connect to the watcher system."]
    #[doc = " The errors returned by the service are in the canonical error space,"]
    #[doc = " see [google.rpc.Code][]."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct WatcherServer<T: Watcher> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Watcher> WatcherServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for WatcherServer<T>
    where
        T: Watcher,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/google.watcher.v1.Watcher/Watch" => {
                    #[allow(non_camel_case_types)]
                    struct WatchSvc<T: Watcher>(pub Arc<T>);
                    impl<T: Watcher> tonic::server::ServerStreamingService<super::Request> for WatchSvc<T> {
                        type Response = super::ChangeBatch;
                        type ResponseStream = T::WatchStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::Request>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.watch(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = WatchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.server_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Watcher> Clone for WatcherServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Watcher> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Watcher> tonic::transport::NamedService for WatcherServer<T> {
        const NAME: &'static str = "google.watcher.v1.Watcher";
    }
}
