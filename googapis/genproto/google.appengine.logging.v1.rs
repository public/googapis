/// Application log line emitted while processing a request.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LogLine {
    /// Approximate time when this log entry was made.
    #[prost(message, optional, tag = "1")]
    pub time: ::std::option::Option<::prost_types::Timestamp>,
    /// Severity of this log entry.
    #[prost(
        enumeration = "super::super::super::logging::r#type::LogSeverity",
        tag = "2"
    )]
    pub severity: i32,
    /// App-provided log message.
    #[prost(string, tag = "3")]
    pub log_message: std::string::String,
    /// Where in the source code this log message was written.
    #[prost(message, optional, tag = "4")]
    pub source_location: ::std::option::Option<SourceLocation>,
}
/// Specifies a location in a source code file.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceLocation {
    /// Source file name. Depending on the runtime environment, this might be a
    /// simple name or a fully-qualified name.
    #[prost(string, tag = "1")]
    pub file: std::string::String,
    /// Line within the source file.
    #[prost(int64, tag = "2")]
    pub line: i64,
    /// Human-readable name of the function or method being invoked, with optional
    /// context such as the class or package name. This information is used in
    /// contexts such as the logs viewer, where a file and line number are less
    /// meaningful. The format can vary by language. For example:
    /// `qual.if.ied.Class.method` (Java), `dir/package.func` (Go), `function`
    /// (Python).
    #[prost(string, tag = "3")]
    pub function_name: std::string::String,
}
/// A reference to a particular snapshot of the source tree used to build and
/// deploy an application.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceReference {
    /// Optional. A URI string identifying the repository.
    /// Example: "https://github.com/GoogleCloudPlatform/kubernetes.git"
    #[prost(string, tag = "1")]
    pub repository: std::string::String,
    /// The canonical and persistent identifier of the deployed revision.
    /// Example (git): "0035781c50ec7aa23385dc841529ce8a4b70db1b"
    #[prost(string, tag = "2")]
    pub revision_id: std::string::String,
}
/// Complete log information about a single HTTP request to an App Engine
/// application.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestLog {
    /// Application that handled this request.
    #[prost(string, tag = "1")]
    pub app_id: std::string::String,
    /// Module of the application that handled this request.
    #[prost(string, tag = "37")]
    pub module_id: std::string::String,
    /// Version of the application that handled this request.
    #[prost(string, tag = "2")]
    pub version_id: std::string::String,
    /// Globally unique identifier for a request, which is based on the request
    /// start time.  Request IDs for requests which started later will compare
    /// greater as strings than those for requests which started earlier.
    #[prost(string, tag = "3")]
    pub request_id: std::string::String,
    /// Origin IP address.
    #[prost(string, tag = "4")]
    pub ip: std::string::String,
    /// Time when the request started.
    #[prost(message, optional, tag = "6")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Time when the request finished.
    #[prost(message, optional, tag = "7")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Latency of the request.
    #[prost(message, optional, tag = "8")]
    pub latency: ::std::option::Option<::prost_types::Duration>,
    /// Number of CPU megacycles used to process request.
    #[prost(int64, tag = "9")]
    pub mega_cycles: i64,
    /// Request method. Example: `"GET"`, `"HEAD"`, `"PUT"`, `"POST"`, `"DELETE"`.
    #[prost(string, tag = "10")]
    pub method: std::string::String,
    /// Contains the path and query portion of the URL that was requested. For
    /// example, if the URL was "http://example.com/app?name=val", the resource
    /// would be "/app?name=val".  The fragment identifier, which is identified by
    /// the `#` character, is not included.
    #[prost(string, tag = "11")]
    pub resource: std::string::String,
    /// HTTP version of request. Example: `"HTTP/1.1"`.
    #[prost(string, tag = "12")]
    pub http_version: std::string::String,
    /// HTTP response status code. Example: 200, 404.
    #[prost(int32, tag = "13")]
    pub status: i32,
    /// Size in bytes sent back to client by request.
    #[prost(int64, tag = "14")]
    pub response_size: i64,
    /// Referrer URL of request.
    #[prost(string, tag = "15")]
    pub referrer: std::string::String,
    /// User agent that made the request.
    #[prost(string, tag = "16")]
    pub user_agent: std::string::String,
    /// The logged-in user who made the request.
    ///
    /// Most likely, this is the part of the user's email before the `@` sign.  The
    /// field value is the same for different requests from the same user, but
    /// different users can have similar names.  This information is also
    /// available to the application via the App Engine Users API.
    ///
    /// This field will be populated starting with App Engine 1.9.21.
    #[prost(string, tag = "40")]
    pub nickname: std::string::String,
    /// File or class that handled the request.
    #[prost(string, tag = "17")]
    pub url_map_entry: std::string::String,
    /// Internet host and port number of the resource being requested.
    #[prost(string, tag = "20")]
    pub host: std::string::String,
    /// An indication of the relative cost of serving this request.
    #[prost(double, tag = "21")]
    pub cost: f64,
    /// Queue name of the request, in the case of an offline request.
    #[prost(string, tag = "22")]
    pub task_queue_name: std::string::String,
    /// Task name of the request, in the case of an offline request.
    #[prost(string, tag = "23")]
    pub task_name: std::string::String,
    /// Whether this was a loading request for the instance.
    #[prost(bool, tag = "24")]
    pub was_loading_request: bool,
    /// Time this request spent in the pending request queue.
    #[prost(message, optional, tag = "25")]
    pub pending_time: ::std::option::Option<::prost_types::Duration>,
    /// If the instance processing this request belongs to a manually scaled
    /// module, then this is the 0-based index of the instance. Otherwise, this
    /// value is -1.
    #[prost(int32, tag = "26")]
    pub instance_index: i32,
    /// Whether this request is finished or active.
    #[prost(bool, tag = "27")]
    pub finished: bool,
    /// Whether this is the first `RequestLog` entry for this request.  If an
    /// active request has several `RequestLog` entries written to Stackdriver
    /// Logging, then this field will be set for one of them.
    #[prost(bool, tag = "42")]
    pub first: bool,
    /// An identifier for the instance that handled the request.
    #[prost(string, tag = "28")]
    pub instance_id: std::string::String,
    /// A list of log lines emitted by the application while serving this request.
    #[prost(message, repeated, tag = "29")]
    pub line: ::std::vec::Vec<LogLine>,
    /// App Engine release version.
    #[prost(string, tag = "38")]
    pub app_engine_release: std::string::String,
    /// Stackdriver Trace identifier for this request.
    #[prost(string, tag = "39")]
    pub trace_id: std::string::String,
    /// Source code for the application that handled this request. There can be
    /// more than one source reference per deployed application if source code is
    /// distributed among multiple repositories.
    #[prost(message, repeated, tag = "41")]
    pub source_reference: ::std::vec::Vec<SourceReference>,
}
