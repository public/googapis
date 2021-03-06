/// Request object for ByteStream.Read.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadRequest {
    /// The name of the resource to read.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
    /// The offset for the first byte to return in the read, relative to the start
    /// of the resource.
    ///
    /// A `read_offset` that is negative or greater than the size of the resource
    /// will cause an `OUT_OF_RANGE` error.
    #[prost(int64, tag = "2")]
    pub read_offset: i64,
    /// The maximum number of `data` bytes the server is allowed to return in the
    /// sum of all `ReadResponse` messages. A `read_limit` of zero indicates that
    /// there is no limit, and a negative `read_limit` will cause an error.
    ///
    /// If the stream returns fewer bytes than allowed by the `read_limit` and no
    /// error occurred, the stream includes all data from the `read_offset` to the
    /// end of the resource.
    #[prost(int64, tag = "3")]
    pub read_limit: i64,
}
/// Response object for ByteStream.Read.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadResponse {
    /// A portion of the data for the resource. The service **may** leave `data`
    /// empty for any given `ReadResponse`. This enables the service to inform the
    /// client that the request is still live while it is running an operation to
    /// generate more data.
    #[prost(bytes, tag = "10")]
    pub data: std::vec::Vec<u8>,
}
/// Request object for ByteStream.Write.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteRequest {
    /// The name of the resource to write. This **must** be set on the first
    /// `WriteRequest` of each `Write()` action. If it is set on subsequent calls,
    /// it **must** match the value of the first request.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
    /// The offset from the beginning of the resource at which the data should be
    /// written. It is required on all `WriteRequest`s.
    ///
    /// In the first `WriteRequest` of a `Write()` action, it indicates
    /// the initial offset for the `Write()` call. The value **must** be equal to
    /// the `committed_size` that a call to `QueryWriteStatus()` would return.
    ///
    /// On subsequent calls, this value **must** be set and **must** be equal to
    /// the sum of the first `write_offset` and the sizes of all `data` bundles
    /// sent previously on this stream.
    ///
    /// An incorrect value will cause an error.
    #[prost(int64, tag = "2")]
    pub write_offset: i64,
    /// If `true`, this indicates that the write is complete. Sending any
    /// `WriteRequest`s subsequent to one in which `finish_write` is `true` will
    /// cause an error.
    #[prost(bool, tag = "3")]
    pub finish_write: bool,
    /// A portion of the data for the resource. The client **may** leave `data`
    /// empty for any given `WriteRequest`. This enables the client to inform the
    /// service that the request is still live while it is running an operation to
    /// generate more data.
    #[prost(bytes, tag = "10")]
    pub data: std::vec::Vec<u8>,
}
/// Response object for ByteStream.Write.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WriteResponse {
    /// The number of bytes that have been processed for the given resource.
    #[prost(int64, tag = "1")]
    pub committed_size: i64,
}
/// Request object for ByteStream.QueryWriteStatus.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWriteStatusRequest {
    /// The name of the resource whose write status is being requested.
    #[prost(string, tag = "1")]
    pub resource_name: std::string::String,
}
/// Response object for ByteStream.QueryWriteStatus.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWriteStatusResponse {
    /// The number of bytes that have been processed for the given resource.
    #[prost(int64, tag = "1")]
    pub committed_size: i64,
    /// `complete` is `true` only if the client has sent a `WriteRequest` with
    /// `finish_write` set to true, and the server has processed that request.
    #[prost(bool, tag = "2")]
    pub complete: bool,
}
#[doc = r" Generated client implementations."]
pub mod byte_stream_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " #### Introduction"]
    #[doc = ""]
    #[doc = " The Byte Stream API enables a client to read and write a stream of bytes to"]
    #[doc = " and from a resource. Resources have names, and these names are supplied in"]
    #[doc = " the API calls below to identify the resource that is being read from or"]
    #[doc = " written to."]
    #[doc = ""]
    #[doc = " All implementations of the Byte Stream API export the interface defined here:"]
    #[doc = ""]
    #[doc = " * `Read()`: Reads the contents of a resource."]
    #[doc = ""]
    #[doc = " * `Write()`: Writes the contents of a resource. The client can call `Write()`"]
    #[doc = "   multiple times with the same resource and can check the status of the write"]
    #[doc = "   by calling `QueryWriteStatus()`."]
    #[doc = ""]
    #[doc = " #### Service parameters and metadata"]
    #[doc = ""]
    #[doc = " The ByteStream API provides no direct way to access/modify any metadata"]
    #[doc = " associated with the resource."]
    #[doc = ""]
    #[doc = " #### Errors"]
    #[doc = ""]
    #[doc = " The errors returned by the service are in the Google canonical error space."]
    pub struct ByteStreamClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ByteStreamClient<tonic::transport::Channel> {
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
    impl<T> ByteStreamClient<T>
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
        #[doc = " `Read()` is used to retrieve the contents of a resource as a sequence"]
        #[doc = " of bytes. The bytes are returned in a sequence of responses, and the"]
        #[doc = " responses are delivered as the results of a server-side streaming RPC."]
        pub async fn read(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadRequest>,
        ) -> Result<tonic::Response<tonic::codec::Streaming<super::ReadResponse>>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/google.bytestream.ByteStream/Read");
            self.inner
                .server_streaming(request.into_request(), path, codec)
                .await
        }
        #[doc = " `Write()` is used to send the contents of a resource as a sequence of"]
        #[doc = " bytes. The bytes are sent in a sequence of request protos of a client-side"]
        #[doc = " streaming RPC."]
        #[doc = ""]
        #[doc = " A `Write()` action is resumable. If there is an error or the connection is"]
        #[doc = " broken during the `Write()`, the client should check the status of the"]
        #[doc = " `Write()` by calling `QueryWriteStatus()` and continue writing from the"]
        #[doc = " returned `committed_size`. This may be less than the amount of data the"]
        #[doc = " client previously sent."]
        #[doc = ""]
        #[doc = " Calling `Write()` on a resource name that was previously written and"]
        #[doc = " finalized could cause an error, depending on whether the underlying service"]
        #[doc = " allows over-writing of previously written resources."]
        #[doc = ""]
        #[doc = " When the client closes the request channel, the service will respond with"]
        #[doc = " a `WriteResponse`. The service will not view the resource as `complete`"]
        #[doc = " until the client has sent a `WriteRequest` with `finish_write` set to"]
        #[doc = " `true`. Sending any requests on a stream after sending a request with"]
        #[doc = " `finish_write` set to `true` will cause an error. The client **should**"]
        #[doc = " check the `WriteResponse` it receives to determine how much data the"]
        #[doc = " service was able to commit and whether the service views the resource as"]
        #[doc = " `complete` or not."]
        pub async fn write(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::WriteRequest>,
        ) -> Result<tonic::Response<super::WriteResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/google.bytestream.ByteStream/Write");
            self.inner
                .client_streaming(request.into_streaming_request(), path, codec)
                .await
        }
        #[doc = " `QueryWriteStatus()` is used to find the `committed_size` for a resource"]
        #[doc = " that is being written, which can then be used as the `write_offset` for"]
        #[doc = " the next `Write()` call."]
        #[doc = ""]
        #[doc = " If the resource does not exist (i.e., the resource has been deleted, or the"]
        #[doc = " first `Write()` has not yet reached the service), this method returns the"]
        #[doc = " error `NOT_FOUND`."]
        #[doc = ""]
        #[doc = " The client **may** call `QueryWriteStatus()` at any time to determine how"]
        #[doc = " much data has been processed for this resource. This is useful if the"]
        #[doc = " client is buffering data and needs to know which data can be safely"]
        #[doc = " evicted. For any sequence of `QueryWriteStatus()` calls for a given"]
        #[doc = " resource name, the sequence of returned `committed_size` values will be"]
        #[doc = " non-decreasing."]
        pub async fn query_write_status(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryWriteStatusRequest>,
        ) -> Result<tonic::Response<super::QueryWriteStatusResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.bytestream.ByteStream/QueryWriteStatus",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ByteStreamClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ByteStreamClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ByteStreamClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod byte_stream_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ByteStreamServer."]
    #[async_trait]
    pub trait ByteStream: Send + Sync + 'static {
        #[doc = "Server streaming response type for the Read method."]
        type ReadStream: Stream<Item = Result<super::ReadResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " `Read()` is used to retrieve the contents of a resource as a sequence"]
        #[doc = " of bytes. The bytes are returned in a sequence of responses, and the"]
        #[doc = " responses are delivered as the results of a server-side streaming RPC."]
        async fn read(
            &self,
            request: tonic::Request<super::ReadRequest>,
        ) -> Result<tonic::Response<Self::ReadStream>, tonic::Status>;
        #[doc = " `Write()` is used to send the contents of a resource as a sequence of"]
        #[doc = " bytes. The bytes are sent in a sequence of request protos of a client-side"]
        #[doc = " streaming RPC."]
        #[doc = ""]
        #[doc = " A `Write()` action is resumable. If there is an error or the connection is"]
        #[doc = " broken during the `Write()`, the client should check the status of the"]
        #[doc = " `Write()` by calling `QueryWriteStatus()` and continue writing from the"]
        #[doc = " returned `committed_size`. This may be less than the amount of data the"]
        #[doc = " client previously sent."]
        #[doc = ""]
        #[doc = " Calling `Write()` on a resource name that was previously written and"]
        #[doc = " finalized could cause an error, depending on whether the underlying service"]
        #[doc = " allows over-writing of previously written resources."]
        #[doc = ""]
        #[doc = " When the client closes the request channel, the service will respond with"]
        #[doc = " a `WriteResponse`. The service will not view the resource as `complete`"]
        #[doc = " until the client has sent a `WriteRequest` with `finish_write` set to"]
        #[doc = " `true`. Sending any requests on a stream after sending a request with"]
        #[doc = " `finish_write` set to `true` will cause an error. The client **should**"]
        #[doc = " check the `WriteResponse` it receives to determine how much data the"]
        #[doc = " service was able to commit and whether the service views the resource as"]
        #[doc = " `complete` or not."]
        async fn write(
            &self,
            request: tonic::Request<tonic::Streaming<super::WriteRequest>>,
        ) -> Result<tonic::Response<super::WriteResponse>, tonic::Status>;
        #[doc = " `QueryWriteStatus()` is used to find the `committed_size` for a resource"]
        #[doc = " that is being written, which can then be used as the `write_offset` for"]
        #[doc = " the next `Write()` call."]
        #[doc = ""]
        #[doc = " If the resource does not exist (i.e., the resource has been deleted, or the"]
        #[doc = " first `Write()` has not yet reached the service), this method returns the"]
        #[doc = " error `NOT_FOUND`."]
        #[doc = ""]
        #[doc = " The client **may** call `QueryWriteStatus()` at any time to determine how"]
        #[doc = " much data has been processed for this resource. This is useful if the"]
        #[doc = " client is buffering data and needs to know which data can be safely"]
        #[doc = " evicted. For any sequence of `QueryWriteStatus()` calls for a given"]
        #[doc = " resource name, the sequence of returned `committed_size` values will be"]
        #[doc = " non-decreasing."]
        async fn query_write_status(
            &self,
            request: tonic::Request<super::QueryWriteStatusRequest>,
        ) -> Result<tonic::Response<super::QueryWriteStatusResponse>, tonic::Status>;
    }
    #[doc = " #### Introduction"]
    #[doc = ""]
    #[doc = " The Byte Stream API enables a client to read and write a stream of bytes to"]
    #[doc = " and from a resource. Resources have names, and these names are supplied in"]
    #[doc = " the API calls below to identify the resource that is being read from or"]
    #[doc = " written to."]
    #[doc = ""]
    #[doc = " All implementations of the Byte Stream API export the interface defined here:"]
    #[doc = ""]
    #[doc = " * `Read()`: Reads the contents of a resource."]
    #[doc = ""]
    #[doc = " * `Write()`: Writes the contents of a resource. The client can call `Write()`"]
    #[doc = "   multiple times with the same resource and can check the status of the write"]
    #[doc = "   by calling `QueryWriteStatus()`."]
    #[doc = ""]
    #[doc = " #### Service parameters and metadata"]
    #[doc = ""]
    #[doc = " The ByteStream API provides no direct way to access/modify any metadata"]
    #[doc = " associated with the resource."]
    #[doc = ""]
    #[doc = " #### Errors"]
    #[doc = ""]
    #[doc = " The errors returned by the service are in the Google canonical error space."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct ByteStreamServer<T: ByteStream> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ByteStream> ByteStreamServer<T> {
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
    impl<T, B> Service<http::Request<B>> for ByteStreamServer<T>
    where
        T: ByteStream,
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
                "/google.bytestream.ByteStream/Read" => {
                    #[allow(non_camel_case_types)]
                    struct ReadSvc<T: ByteStream>(pub Arc<T>);
                    impl<T: ByteStream> tonic::server::ServerStreamingService<super::ReadRequest> for ReadSvc<T> {
                        type Response = super::ReadResponse;
                        type ResponseStream = T::ReadStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReadRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.read(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = ReadSvc(inner);
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
                "/google.bytestream.ByteStream/Write" => {
                    #[allow(non_camel_case_types)]
                    struct WriteSvc<T: ByteStream>(pub Arc<T>);
                    impl<T: ByteStream> tonic::server::ClientStreamingService<super::WriteRequest> for WriteSvc<T> {
                        type Response = super::WriteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<tonic::Streaming<super::WriteRequest>>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.write(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = WriteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.client_streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/google.bytestream.ByteStream/QueryWriteStatus" => {
                    #[allow(non_camel_case_types)]
                    struct QueryWriteStatusSvc<T: ByteStream>(pub Arc<T>);
                    impl<T: ByteStream> tonic::server::UnaryService<super::QueryWriteStatusRequest>
                        for QueryWriteStatusSvc<T>
                    {
                        type Response = super::QueryWriteStatusResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryWriteStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.query_write_status(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = QueryWriteStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
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
    impl<T: ByteStream> Clone for ByteStreamServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ByteStream> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ByteStream> tonic::transport::NamedService for ByteStreamServer<T> {
        const NAME: &'static str = "google.bytestream.ByteStream";
    }
}
