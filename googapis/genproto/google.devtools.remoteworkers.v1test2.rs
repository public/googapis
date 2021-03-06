/// Describes a worker, which is a list of one or more devices and the
/// connections between them. A device could be a computer, a phone, or even an
/// accelerator like a GPU; it's up to the farm administrator to decide how to
/// model their farm. For example, if a farm only has one type of GPU, the GPU
/// could be modelled as a "has_gpu" property on its host computer; if it has
/// many subproperties itself, it might be better to model it as a separate
/// device.
///
/// The first device in the worker is the "primary device" - that is, the device
/// running a bot and which is responsible for actually executing commands. All
/// other devices are considered to be attached devices, and must be controllable
/// by the primary device.
///
/// This message (and all its submessages) can be used in two contexts:
///
/// * Status: sent by the bot to report the current capabilities of the device to
/// allow reservation matching.
/// * Request: sent by a client to request a device with certain capabilities in
/// a reservation.
///
/// Several of the fields in this message have different semantics depending on
/// which of which of these contexts it is used. These semantics are described
/// below.
///
/// Several messages in Worker and its submessages have the concept of keys and
/// values, such as `Worker.Property` and `Device.Property`. All keys are simple
/// strings, but certain keys are "standard" keys and should be broadly supported
/// across farms and implementations; these are listed below each relevant
/// message. Bot implementations or farm admins may add *additional* keys, but
/// these SHOULD all begin with an underscore so they do not conflict with
/// standard keys that may be added in the future.
///
/// Keys are not context sensitive.
///
/// See http://goo.gl/NurY8g for more information on the Worker message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Worker {
    /// A list of devices; the first device is the primary device. See the `Device`
    /// message for more information.
    #[prost(message, repeated, tag = "1")]
    pub devices: ::std::vec::Vec<Device>,
    /// A worker may contain "global" properties. For example, certain machines
    /// might be reserved for certain types of jobs, like short-running compilation
    /// versus long-running integration tests. This property is known as a "pool"
    /// and is not related to any one device within the worker; rather, it applies
    /// to the worker as a whole.
    ///
    /// The behaviour of repeated keys is identical to that of Device.Property.
    #[prost(message, repeated, tag = "2")]
    pub properties: ::std::vec::Vec<worker::Property>,
    /// Bots can be configured in certain ways when accepting leases. For example,
    /// many leases are executed inside a Docker container. To support this, the
    /// bot needs to be able to report that it has Docker installed (and knows how
    /// to execute something inside a container), and the task submitter needs to
    /// specify which image should be used to start the container. Similarly, a
    /// lease may be able to run as one of several users on the worker; in such
    /// cases, the bot needs to report what users are available, and the submitter
    /// needs to choose one.
    ///
    /// Therefore, when this message is reported by the bot to the service, each
    /// key represents a *type* of configuration that the bot knows how to set,
    /// while each *value* represents a legal value for that configuration (the
    /// empty string is interpretted as a wildcard, such as for Docker images).
    /// When this message is sent by the server to the bot in the context of a
    /// lease, it represents a command to the bot to apply the setting. Keys may
    /// be repeated during reporting but not in a lease.
    #[prost(message, repeated, tag = "3")]
    pub configs: ::std::vec::Vec<worker::Config>,
}
pub mod worker {
    /// A global property; see the `properties` field for more information.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Property {
        /// For general information on keys, see the documentation to `Worker`.
        ///
        /// The current set of standard keys are:
        ///
        /// * pool: different workers can be reserved for different purposes. For
        /// example, an admin might want to segregate long-running integration tests
        /// from short-running unit tests, so unit tests will always get some
        /// throughput. To support this, the server can assign different values for
        /// `pool` (such as "itest" and "utest") to different workers, and then have
        /// jobs request workers from those pools.
        #[prost(string, tag = "1")]
        pub key: std::string::String,
        /// The property's value.
        #[prost(string, tag = "2")]
        pub value: std::string::String,
    }
    /// A configuration request or report; see the `configs` field for more
    /// information.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Config {
        /// For general information on keys, see the documentation to `Worker`.
        ///
        /// The current set of standard keys are:
        ///
        /// * DockerImage: the image of the container. When being reported by the
        /// bot, the empty value should always be included if the bot is able to pull
        /// its own images; the bot may optionally *also* report images that are
        /// present in its cache. When being requested in a lease, the value is the
        /// URI of the image (eg `gcr.io/user/image@sha256:hash`).
        #[prost(string, tag = "1")]
        pub key: std::string::String,
        /// The configuration's value.
        #[prost(string, tag = "2")]
        pub value: std::string::String,
    }
}
/// Any device, including computers, phones, accelerators (e.g. GPUs), etc. All
/// names must be unique.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Device {
    /// The handle can be thought of as the "name" of the device, and must be
    /// unique within a Worker.
    ///
    /// In the Status context, the handle should be some human-understandable name,
    /// perhaps corresponding to a label physically written on the device to make
    /// it easy to locate. In the Request context, the name should be the
    /// *logical* name expected by the task. The bot is responsible for mapping the
    /// logical name expected by the task to a machine-readable name that the task
    /// can actually use, such as a USB address. The method by which this mapping
    /// is communicated to the task is not covered in this API.
    #[prost(string, tag = "1")]
    pub handle: std::string::String,
    /// Properties of this device that don't change based on the tasks that are
    /// running on it, e.g. OS, CPU architecture, etc.
    ///
    /// Keys may be repeated, and have the following interpretation:
    ///
    ///    * Status context: the device can support *any* the listed values. For
    ///    example, an "ISA" property might include "x86", "x86-64" and "sse4".
    ///
    ///    * Request context: the device *must* support *all* of the listed values.
    #[prost(message, repeated, tag = "2")]
    pub properties: ::std::vec::Vec<device::Property>,
}
pub mod device {
    /// A device property; see `properties` for more information.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Property {
        /// For general information on keys, see the documentation to `Worker`.
        ///
        /// The current set of standard keys are:
        ///
        /// * os: a human-readable description of the OS. Examples include `linux`,
        /// `ubuntu` and `ubuntu 14.04` (note that a bot may advertise itself as more
        /// than one). This will be replaced in the future by more well-structured
        /// keys and values to represent OS variants.
        ///
        /// * has-docker: "true" if the bot has Docker installed. This will be
        /// replaced in the future by a more structured message for Docker support.
        #[prost(string, tag = "1")]
        pub key: std::string::String,
        /// The property's value.
        #[prost(string, tag = "2")]
        pub value: std::string::String,
    }
}
/// A bot session represents the state of a bot while in continuous contact with
/// the server for a period of time. The session includes information about the
/// worker - that is, the *worker* (the physical or virtual hardware) is
/// considered to be a property of the bot (the software agent running on that
/// hardware), which is the reverse of real life, but more natural from the point
/// of the view of this API, which communicates solely with the bot and not
/// directly with the underlying worker.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BotSession {
    /// The bot session name, as selected by the server. Output only during a call
    /// to CreateBotSession.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// A unique bot ID within the farm used to persistently identify this bot over
    /// time (i.e., over multiple sessions). This ID must be unique within a
    /// farm. Typically, the bot ID will be the same as the name of the primary
    /// device in the worker (e.g., what you'd get from typing `uname -n` on *nix),
    /// but this is not required since a single device may allow multiple bots to
    /// run on it, each with access to different resources. What is important is
    /// that this ID is meaningful to humans, who might need to hunt a physical
    /// machine down to fix it.
    ///
    /// When CreateBotSession is successfully called with a bot_id, all prior
    /// sessions with the same ID are invalidated. If a bot attempts to update an
    /// invalid session, the server must reject that request, and may also
    /// quarantine the other bot with the same bot IDs (ie, stop sending it new
    /// leases and alert an admin).
    #[prost(string, tag = "2")]
    pub bot_id: std::string::String,
    /// The status of the bot. This must be populated in every call to
    /// UpdateBotSession.
    #[prost(enumeration = "BotStatus", tag = "3")]
    pub status: i32,
    /// A description of the worker hosting this bot. The Worker message is used
    /// here in the Status context (see Worker for more information).  If multiple
    /// bots are running on the worker, this field should only describe the
    /// resources accessible from this bot.
    ///
    /// During the call to CreateBotSession, the server may make arbitrary changes
    /// to the worker's `server_properties` field (see that field for more
    /// information). Otherwise, this field is input-only.
    #[prost(message, optional, tag = "4")]
    pub worker: ::std::option::Option<Worker>,
    /// A list of all leases that are a part of this session. See the Lease message
    /// for details.
    #[prost(message, repeated, tag = "5")]
    pub leases: ::std::vec::Vec<Lease>,
    /// The time at which this bot session will expire, unless the bot calls
    /// UpdateBotSession again. Output only.
    #[prost(message, optional, tag = "6")]
    pub expire_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The version of the bot code currently running. The server may use this
    /// information to issue an admin action to tell the bot to update itself.
    #[prost(string, tag = "7")]
    pub version: std::string::String,
}
/// A Lease is a lease that the scheduler has assigned to this bot. If the bot
/// notices (by UpdateBotSession) that it has any leases in the PENDING state, it
/// should call UpdateBotSession to put the leases into the ACTIVE state and
/// start executing their assignments.
///
/// All fields in this message are output-only, *except* the `state` and `status`
/// fields. Note that repeated fields can only be updated as a unit, so on every
/// update the bot must provide an update for *all* the leases the server expects
/// it to report on.
///
/// The scheduler *should* ensure that all leases scheduled to a bot can actually
/// be accepted, but race conditions may occur. In such cases, the bot should
/// attempt to accept the leases in the order they are listed by the server, to
/// allow the server to control priorities.
///
/// The server will remove COMPLETED leases from time to time, after which the
/// bot shouldn't report on them any more (the server will ignore superfluous
/// COMPLETED records).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Lease {
    /// A short string uniquely identifing the lease within this bot session.
    #[prost(string, tag = "7")]
    pub id: std::string::String,
    /// The actual work to be performed, if any. May be omitted by the server if
    /// the lease is not in the `PENDING` state. The message must be meaningful to
    /// the bot. Output only (must only be set by the server).
    #[prost(message, optional, tag = "8")]
    pub payload: ::std::option::Option<::prost_types::Any>,
    /// Any result the bot wishes to provide about the lease. Must not be changed
    /// after the first call with the lease in the `COMPLETED` or `CANCELLED`
    /// state. Input only (must only be set by the bot, will not be echoed by the
    /// server).
    #[prost(message, optional, tag = "9")]
    pub result: ::std::option::Option<::prost_types::Any>,
    /// The state of the lease. See LeaseState for more information.
    #[prost(enumeration = "LeaseState", tag = "2")]
    pub state: i32,
    /// The final status of the lease (should be populated by the bot if the state
    /// is completed). This is the status of the lease, not of any task represented
    /// by the lease. For example, if the bot could not accept the lease because it
    /// asked for some resource the bot didn't have, this status will be
    /// FAILED_PRECONDITION. But if the assignment in the lease didn't execute
    /// correctly, this field will be `OK` while the failure of the assignment must
    /// communicated via the `result` field.
    #[prost(message, optional, tag = "3")]
    pub status: ::std::option::Option<super::super::super::rpc::Status>,
    /// The requirements that are being claimed by this lease. This field may be
    /// omitted by the server if the lease is not pending.
    #[prost(message, optional, tag = "4")]
    pub requirements: ::std::option::Option<Worker>,
    /// The time at which this lease expires. The server *may* extend this over
    /// time, but due to race conditions, the bot is not *required* to respect any
    /// expiry date except the first one.
    #[prost(message, optional, tag = "5")]
    pub expire_time: ::std::option::Option<::prost_types::Timestamp>,
    /// DEPRECATED. The assignment should be provided to the bot via the `payload`
    /// field. Clients that wish to use a simple name (such as a queue of work
    /// provided elsewhere) should define a custom message type and encode it into
    /// `payload`.
    #[prost(string, tag = "1")]
    pub assignment: std::string::String,
    /// DEPRECATED. Use `payload` instead.
    #[prost(message, optional, tag = "6")]
    pub inline_assignment: ::std::option::Option<::prost_types::Any>,
}
/// AdminTemp is a prelimiary set of administration tasks. It's called "Temp"
/// because we do not yet know the best way to represent admin tasks; it's
/// possible that this will be entirely replaced in later versions of this API.
/// If this message proves to be sufficient, it will be renamed in the alpha or
/// beta release of this API.
///
/// This message (suitably marshalled into a protobuf.Any) can be used as the
/// inline_assignment field in a lease; the lease assignment field should simply
/// be `"admin"` in these cases.
///
/// This message is heavily based on Swarming administration tasks from the LUCI
/// project (http://github.com/luci/luci-py/appengine/swarming).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdminTemp {
    /// The admin action; see `Command` for legal values.
    #[prost(enumeration = "admin_temp::Command", tag = "1")]
    pub command: i32,
    /// The argument to the admin action; see `Command` for semantics.
    #[prost(string, tag = "2")]
    pub arg: std::string::String,
}
pub mod admin_temp {
    /// Possible administration actions.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Command {
        /// Illegal value.
        Unspecified = 0,
        /// Download and run a new version of the bot. `arg` will be a resource
        /// accessible via `ByteStream.Read` to obtain the new bot code.
        BotUpdate = 1,
        /// Restart the bot without downloading a new version. `arg` will be a
        /// message to log.
        BotRestart = 2,
        /// Shut down the bot. `arg` will be a task resource name (similar to those
        /// in tasks.proto) that the bot can use to tell the server that it is
        /// terminating.
        BotTerminate = 3,
        /// Restart the host computer. `arg` will be a message to log.
        HostRestart = 4,
    }
}
/// Request message for CreateBotSession.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBotSessionRequest {
    /// Required. The farm resource.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The bot session to create. Server-assigned fields like name must be unset.
    #[prost(message, optional, tag = "2")]
    pub bot_session: ::std::option::Option<BotSession>,
}
/// Request message for UpdateBotSession.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBotSessionRequest {
    /// Required. The bot session name. Must match bot_session.name.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The bot session resource to update.
    #[prost(message, optional, tag = "2")]
    pub bot_session: ::std::option::Option<BotSession>,
    /// Required. The fields on the bot that should be updated. See the BotSession resource
    /// for which fields are updatable by which caller.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// A coarse description of the status of the bot that the server uses to
/// determine whether to assign the bot new leases.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BotStatus {
    /// Default value; do not use.
    Unspecified = 0,
    /// The bot is healthy, and will accept leases as normal.
    Ok = 1,
    /// The bot is unhealthy and will not accept new leases. For example, the bot
    /// may have detected that available disk space is too low. This situation may
    /// resolve itself, but will typically require human intervention.
    Unhealthy = 2,
    /// The bot has been asked to reboot the host. The bot will not accept new
    /// leases; once all leases are complete, this session will no longer be
    /// updated but the bot will be expected to establish a new session after the
    /// reboot completes.
    HostRebooting = 3,
    /// The bot has been asked to shut down. As with HOST_REBOOTING, once all
    /// leases are completed, the session will no longer be updated and the bot
    /// will not be expected to establish a new session.
    ///
    /// Bots are typically only asked to shut down if its host computer will be
    /// modified in some way, such as deleting a VM.
    BotTerminating = 4,
    /// The bot is initializing and is not ready to accept leases.
    Initializing = 5,
}
/// The state of the lease. All leases start in the PENDING state. A bot can
/// change PENDING to ACTIVE or (in the case of an error) COMPLETED, or from
/// ACTIVE to COMPLETED. The server can change PENDING or ACTIVE to CANCELLED if
/// it wants the bot to release its resources - for example, if the bot needs to
/// be quarantined (it's producing bad output) or a cell needs to be drained.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LeaseState {
    /// Default value; do not use.
    Unspecified = 0,
    /// Pending: the server expects the bot to accept this lease. This may only be
    /// set by the server.
    Pending = 1,
    /// Active: the bot has accepted this lease. This may only be set by the bot.
    Active = 2,
    /// Completed: the bot is no longer leased. This may only be set by the bot,
    /// and the status field must be populated iff the state is COMPLETED.
    Completed = 4,
    /// Cancelled: The bot should immediately release all resources associated with
    /// the lease. This may only be set by the server.
    Cancelled = 5,
}
#[doc = r" Generated client implementations."]
pub mod bots_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Design doc: https://goo.gl/oojM5H"]
    #[doc = ""]
    #[doc = " Loosely speaking, the Bots interface monitors a collection of workers (think"]
    #[doc = " of them as \"computers\" for a moment). This collection is known as a \"farm,\""]
    #[doc = " and its purpose is to perform work on behalf of a client."]
    #[doc = ""]
    #[doc = " Each worker runs a small program known as a \"bot\" that allows it to be"]
    #[doc = " controlled by the server. This interface contains only methods that are"]
    #[doc = " called by the bots themselves; admin functionality is out of scope for this"]
    #[doc = " interface."]
    #[doc = ""]
    #[doc = " More precisely, we use the term \"worker\" to refer to the physical \"thing\""]
    #[doc = " running the bot. We use the term \"worker,\" and not \"machine\" or \"computer,\""]
    #[doc = " since a worker may consist of more than one machine - e.g., a computer with"]
    #[doc = " multiple attached devices, or even a cluster of computers, with only one of"]
    #[doc = " them running the bot. Conversely, a single machine may host several bots, in"]
    #[doc = " which case each bot has a \"worker\" corresponding to the slice of the machine"]
    #[doc = " being managed by that bot."]
    #[doc = ""]
    #[doc = " The main resource in the Bots interface is not, surprisingly, a Bot - it is a"]
    #[doc = " BotSession, which represents a period of time in which a bot is in continuous"]
    #[doc = " contact with the server (see the BotSession message for more information)."]
    #[doc = " The parent of a bot session can be thought of as an instance of a farm. That"]
    #[doc = " is, one endpoint may be able to manage many farms for many users. For"]
    #[doc = " example, for a farm managed through GCP, the parent resource will typically"]
    #[doc = " take the form \"projects/{project_id}\". This is referred to below as \"the farm"]
    #[doc = " resource.\""]
    pub struct BotsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BotsClient<tonic::transport::Channel> {
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
    impl<T> BotsClient<T>
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
        #[doc = " CreateBotSession is called when the bot first joins the farm, and"]
        #[doc = " establishes a session ID to ensure that multiple machines do not register"]
        #[doc = " using the same name accidentally."]
        pub async fn create_bot_session(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBotSessionRequest>,
        ) -> Result<tonic::Response<super::BotSession>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.remoteworkers.v1test2.Bots/CreateBotSession",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " UpdateBotSession must be called periodically by the bot (on a schedule"]
        #[doc = " determined by the server) to let the server know about its status, and to"]
        #[doc = " pick up new lease requests from the server."]
        pub async fn update_bot_session(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBotSessionRequest>,
        ) -> Result<tonic::Response<super::BotSession>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.remoteworkers.v1test2.Bots/UpdateBotSession",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for BotsClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for BotsClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "BotsClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod bots_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with BotsServer."]
    #[async_trait]
    pub trait Bots: Send + Sync + 'static {
        #[doc = " CreateBotSession is called when the bot first joins the farm, and"]
        #[doc = " establishes a session ID to ensure that multiple machines do not register"]
        #[doc = " using the same name accidentally."]
        async fn create_bot_session(
            &self,
            request: tonic::Request<super::CreateBotSessionRequest>,
        ) -> Result<tonic::Response<super::BotSession>, tonic::Status>;
        #[doc = " UpdateBotSession must be called periodically by the bot (on a schedule"]
        #[doc = " determined by the server) to let the server know about its status, and to"]
        #[doc = " pick up new lease requests from the server."]
        async fn update_bot_session(
            &self,
            request: tonic::Request<super::UpdateBotSessionRequest>,
        ) -> Result<tonic::Response<super::BotSession>, tonic::Status>;
    }
    #[doc = " Design doc: https://goo.gl/oojM5H"]
    #[doc = ""]
    #[doc = " Loosely speaking, the Bots interface monitors a collection of workers (think"]
    #[doc = " of them as \"computers\" for a moment). This collection is known as a \"farm,\""]
    #[doc = " and its purpose is to perform work on behalf of a client."]
    #[doc = ""]
    #[doc = " Each worker runs a small program known as a \"bot\" that allows it to be"]
    #[doc = " controlled by the server. This interface contains only methods that are"]
    #[doc = " called by the bots themselves; admin functionality is out of scope for this"]
    #[doc = " interface."]
    #[doc = ""]
    #[doc = " More precisely, we use the term \"worker\" to refer to the physical \"thing\""]
    #[doc = " running the bot. We use the term \"worker,\" and not \"machine\" or \"computer,\""]
    #[doc = " since a worker may consist of more than one machine - e.g., a computer with"]
    #[doc = " multiple attached devices, or even a cluster of computers, with only one of"]
    #[doc = " them running the bot. Conversely, a single machine may host several bots, in"]
    #[doc = " which case each bot has a \"worker\" corresponding to the slice of the machine"]
    #[doc = " being managed by that bot."]
    #[doc = ""]
    #[doc = " The main resource in the Bots interface is not, surprisingly, a Bot - it is a"]
    #[doc = " BotSession, which represents a period of time in which a bot is in continuous"]
    #[doc = " contact with the server (see the BotSession message for more information)."]
    #[doc = " The parent of a bot session can be thought of as an instance of a farm. That"]
    #[doc = " is, one endpoint may be able to manage many farms for many users. For"]
    #[doc = " example, for a farm managed through GCP, the parent resource will typically"]
    #[doc = " take the form \"projects/{project_id}\". This is referred to below as \"the farm"]
    #[doc = " resource.\""]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct BotsServer<T: Bots> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Bots> BotsServer<T> {
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
    impl<T, B> Service<http::Request<B>> for BotsServer<T>
    where
        T: Bots,
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
                "/google.devtools.remoteworkers.v1test2.Bots/CreateBotSession" => {
                    #[allow(non_camel_case_types)]
                    struct CreateBotSessionSvc<T: Bots>(pub Arc<T>);
                    impl<T: Bots> tonic::server::UnaryService<super::CreateBotSessionRequest>
                        for CreateBotSessionSvc<T>
                    {
                        type Response = super::BotSession;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateBotSessionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_bot_session(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateBotSessionSvc(inner);
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
                "/google.devtools.remoteworkers.v1test2.Bots/UpdateBotSession" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateBotSessionSvc<T: Bots>(pub Arc<T>);
                    impl<T: Bots> tonic::server::UnaryService<super::UpdateBotSessionRequest>
                        for UpdateBotSessionSvc<T>
                    {
                        type Response = super::BotSession;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateBotSessionRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_bot_session(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateBotSessionSvc(inner);
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
    impl<T: Bots> Clone for BotsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Bots> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Bots> tonic::transport::NamedService for BotsServer<T> {
        const NAME: &'static str = "google.devtools.remoteworkers.v1test2.Bots";
    }
}
/// Describes a shell-style task to execute, suitable for providing as the Bots
/// interface's `Lease.payload` field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandTask {
    /// The inputs to the task.
    #[prost(message, optional, tag = "1")]
    pub inputs: ::std::option::Option<command_task::Inputs>,
    /// The expected outputs from the task.
    #[prost(message, optional, tag = "4")]
    pub expected_outputs: ::std::option::Option<command_task::Outputs>,
    /// The timeouts of this task.
    #[prost(message, optional, tag = "5")]
    pub timeouts: ::std::option::Option<command_task::Timeouts>,
}
pub mod command_task {
    /// Describes the inputs to a shell-style task.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Inputs {
        /// The command itself to run (e.g., argv).
        ///
        /// This field should be passed directly to the underlying operating system,
        /// and so it must be sensible to that operating system. For example, on
        /// Windows, the first argument might be "C:\Windows\System32\ping.exe" -
        /// that is, using drive letters and backslashes. A command for a *nix
        /// system, on the other hand, would use forward slashes.
        ///
        /// All other fields in the RWAPI must consistently use forward slashes,
        /// since those fields may be interpretted by both the service and the bot.
        #[prost(string, repeated, tag = "1")]
        pub arguments: ::std::vec::Vec<std::string::String>,
        /// The input filesystem to be set up prior to the task beginning. The
        /// contents should be a repeated set of FileMetadata messages though other
        /// formats are allowed if better for the implementation (eg, a LUCI-style
        /// .isolated file).
        ///
        /// This field is repeated since implementations might want to cache the
        /// metadata, in which case it may be useful to break up portions of the
        /// filesystem that change frequently (eg, specific input files) from those
        /// that don't (eg, standard header files).
        #[prost(message, repeated, tag = "2")]
        pub files: ::std::vec::Vec<super::Digest>,
        /// Inline contents for blobs expected to be needed by the bot to execute the
        /// task. For example, contents of entries in `files` or blobs that are
        /// indirectly referenced by an entry there.
        ///
        /// The bot should check against this list before downloading required task
        /// inputs to reduce the number of communications between itself and the
        /// remote CAS server.
        #[prost(message, repeated, tag = "4")]
        pub inline_blobs: ::std::vec::Vec<super::Blob>,
        /// All environment variables required by the task.
        #[prost(message, repeated, tag = "3")]
        pub environment_variables: ::std::vec::Vec<inputs::EnvironmentVariable>,
        /// Directory from which a command is executed. It is a relative directory
        /// with respect to the bot's working directory (i.e., "./"). If it is
        /// non-empty, then it must exist under "./". Otherwise, "./" will be used.
        #[prost(string, tag = "5")]
        pub working_directory: std::string::String,
    }
    pub mod inputs {
        /// An environment variable required by this task.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct EnvironmentVariable {
            /// The envvar name.
            #[prost(string, tag = "1")]
            pub name: std::string::String,
            /// The envvar value.
            #[prost(string, tag = "2")]
            pub value: std::string::String,
        }
    }
    /// Describes the expected outputs of the command.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Outputs {
        /// A list of expected files, relative to the execution root. All paths
        /// MUST be delimited by forward slashes.
        #[prost(string, repeated, tag = "1")]
        pub files: ::std::vec::Vec<std::string::String>,
        /// A list of expected directories, relative to the execution root. All paths
        /// MUST be delimited by forward slashes.
        #[prost(string, repeated, tag = "2")]
        pub directories: ::std::vec::Vec<std::string::String>,
        /// The destination to which any stdout should be sent. The method by which
        /// the bot should send the stream contents to that destination is not
        /// defined in this API. As examples, the destination could be a file
        /// referenced in the `files` field in this message, or it could be a URI
        /// that must be written via the ByteStream API.
        #[prost(string, tag = "3")]
        pub stdout_destination: std::string::String,
        /// The destination to which any stderr should be sent. The method by which
        /// the bot should send the stream contents to that destination is not
        /// defined in this API. As examples, the destination could be a file
        /// referenced in the `files` field in this message, or it could be a URI
        /// that must be written via the ByteStream API.
        #[prost(string, tag = "4")]
        pub stderr_destination: std::string::String,
    }
    /// Describes the timeouts associated with this task.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Timeouts {
        /// This specifies the maximum time that the task can run, excluding the
        /// time required to download inputs or upload outputs. That is, the worker
        /// will terminate the task if it runs longer than this.
        #[prost(message, optional, tag = "1")]
        pub execution: ::std::option::Option<::prost_types::Duration>,
        /// This specifies the maximum amount of time the task can be idle - that is,
        /// go without generating some output in either stdout or stderr. If the
        /// process is silent for more than the specified time, the worker will
        /// terminate the task.
        #[prost(message, optional, tag = "2")]
        pub idle: ::std::option::Option<::prost_types::Duration>,
        /// If the execution or IO timeouts are exceeded, the worker will try to
        /// gracefully terminate the task and return any existing logs. However,
        /// tasks may be hard-frozen in which case this process will fail. This
        /// timeout specifies how long to wait for a terminated task to shut down
        /// gracefully (e.g. via SIGTERM) before we bring down the hammer (e.g.
        /// SIGKILL on *nix, CTRL_BREAK_EVENT on Windows).
        #[prost(message, optional, tag = "3")]
        pub shutdown: ::std::option::Option<::prost_types::Duration>,
    }
}
/// DEPRECATED - use CommandResult instead.
/// Describes the actual outputs from the task.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandOutputs {
    /// exit_code is only fully reliable if the status' code is OK. If the task
    /// exceeded its deadline or was cancelled, the process may still produce an
    /// exit code as it is cancelled, and this will be populated, but a successful
    /// (zero) is unlikely to be correct unless the status code is OK.
    #[prost(int32, tag = "1")]
    pub exit_code: i32,
    /// The output files. The blob referenced by the digest should contain
    /// one of the following (implementation-dependent):
    ///    * A marshalled DirectoryMetadata of the returned filesystem
    ///    * A LUCI-style .isolated file
    #[prost(message, optional, tag = "2")]
    pub outputs: ::std::option::Option<Digest>,
}
/// DEPRECATED - use CommandResult instead.
/// Can be used as part of CompleteRequest.metadata, or are part of a more
/// sophisticated message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandOverhead {
    /// The elapsed time between calling Accept and Complete. The server will also
    /// have its own idea of what this should be, but this excludes the overhead of
    /// the RPCs and the bot response time.
    #[prost(message, optional, tag = "1")]
    pub duration: ::std::option::Option<::prost_types::Duration>,
    /// The amount of time *not* spent executing the command (ie
    /// uploading/downloading files).
    #[prost(message, optional, tag = "2")]
    pub overhead: ::std::option::Option<::prost_types::Duration>,
}
/// All information about the execution of a command, suitable for providing as
/// the Bots interface's `Lease.result` field.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommandResult {
    /// An overall status for the command. For example, if the command timed out,
    /// this might have a code of DEADLINE_EXCEEDED; if it was killed by the OS for
    /// memory exhaustion, it might have a code of RESOURCE_EXHAUSTED.
    #[prost(message, optional, tag = "1")]
    pub status: ::std::option::Option<super::super::super::rpc::Status>,
    /// The exit code of the process. An exit code of "0" should only be trusted if
    /// `status` has a code of OK (otherwise it may simply be unset).
    #[prost(int32, tag = "2")]
    pub exit_code: i32,
    /// The output files. The blob referenced by the digest should contain
    /// one of the following (implementation-dependent):
    ///    * A marshalled DirectoryMetadata of the returned filesystem
    ///    * A LUCI-style .isolated file
    #[prost(message, optional, tag = "3")]
    pub outputs: ::std::option::Option<Digest>,
    /// The elapsed time between calling Accept and Complete. The server will also
    /// have its own idea of what this should be, but this excludes the overhead of
    /// the RPCs and the bot response time.
    #[prost(message, optional, tag = "4")]
    pub duration: ::std::option::Option<::prost_types::Duration>,
    /// The amount of time *not* spent executing the command (ie
    /// uploading/downloading files).
    #[prost(message, optional, tag = "5")]
    pub overhead: ::std::option::Option<::prost_types::Duration>,
    /// Implementation-dependent metadata about the task. Both servers and bots
    /// may define messages which can be encoded here; bots are free to provide
    /// metadata in multiple formats, and servers are free to choose one or more
    /// of the values to process and ignore others. In particular, it is *not*
    /// considered an error for the bot to provide the server with a field that it
    /// doesn't know about.
    #[prost(message, repeated, tag = "6")]
    pub metadata: ::std::vec::Vec<::prost_types::Any>,
}
/// The metadata for a file. Similar to the equivalent message in the Remote
/// Execution API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileMetadata {
    /// The path of this file. If this message is part of the
    /// CommandOutputs.outputs fields, the path is relative to the execution root
    /// and must correspond to an entry in CommandTask.outputs.files. If this
    /// message is part of a Directory message, then the path is relative to the
    /// root of that directory. All paths MUST be delimited by forward slashes.
    #[prost(string, tag = "1")]
    pub path: std::string::String,
    /// A pointer to the contents of the file. The method by which a client
    /// retrieves the contents from a CAS system is not defined here.
    #[prost(message, optional, tag = "2")]
    pub digest: ::std::option::Option<Digest>,
    /// If the file is small enough, its contents may also or alternatively be
    /// listed here.
    #[prost(bytes, tag = "3")]
    pub contents: std::vec::Vec<u8>,
    /// Properties of the file
    #[prost(bool, tag = "4")]
    pub is_executable: bool,
}
/// The metadata for a directory. Similar to the equivalent message in the Remote
/// Execution API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DirectoryMetadata {
    /// The path of the directory, as in [FileMetadata.path][google.devtools.remoteworkers.v1test2.FileMetadata.path].
    #[prost(string, tag = "1")]
    pub path: std::string::String,
    /// A pointer to the contents of the directory, in the form of a marshalled
    /// Directory message.
    #[prost(message, optional, tag = "2")]
    pub digest: ::std::option::Option<Digest>,
}
/// The CommandTask and CommandResult messages assume the existence of a service
/// that can serve blobs of content, identified by a hash and size known as a
/// "digest." The method by which these blobs may be retrieved is not specified
/// here, but a model implementation is in the Remote Execution API's
/// "ContentAddressibleStorage" interface.
///
/// In the context of the RWAPI, a Digest will virtually always refer to the
/// contents of a file or a directory. The latter is represented by the
/// byte-encoded Directory message.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Digest {
    /// A string-encoded hash (eg "1a2b3c", not the byte array [0x1a, 0x2b, 0x3c])
    /// using an implementation-defined hash algorithm (eg SHA-256).
    #[prost(string, tag = "1")]
    pub hash: std::string::String,
    /// The size of the contents. While this is not strictly required as part of an
    /// identifier (after all, any given hash will have exactly one canonical
    /// size), it's useful in almost all cases when one might want to send or
    /// retrieve blobs of content and is included here for this reason.
    #[prost(int64, tag = "2")]
    pub size_bytes: i64,
}
/// Describes a blob of binary content with its digest.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Blob {
    /// The digest of the blob. This should be verified by the receiver.
    #[prost(message, optional, tag = "1")]
    pub digest: ::std::option::Option<Digest>,
    /// The contents of the blob.
    #[prost(bytes, tag = "2")]
    pub contents: std::vec::Vec<u8>,
}
/// The contents of a directory. Similar to the equivalent message in the Remote
/// Execution API.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Directory {
    /// The files in this directory
    #[prost(message, repeated, tag = "1")]
    pub files: ::std::vec::Vec<FileMetadata>,
    /// Any subdirectories
    #[prost(message, repeated, tag = "2")]
    pub directories: ::std::vec::Vec<DirectoryMetadata>,
}
/// DEPRECATED - use Lease.payload instead.
/// A Task represents a unit of work. Its result and logs are defined as
/// subresources.
///
/// If all the `Any` fields are populated, this can be a very large message, and
/// clients may not want the entire message returned on every call to every
/// method. Such clients should request partial responses
/// (https://cloud.google.com/apis/design/design_patterns#partial_response) and
/// servers should implement partial responses in order to reduce unnecessry
/// overhead.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Task {
    /// The name of this task. Output only.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The actual task to perform. For example, this could be CommandTask to run a
    /// command line.
    #[prost(message, optional, tag = "2")]
    pub description: ::std::option::Option<::prost_types::Any>,
    /// Handles to logs. The key is a human-readable name like `stdout`, and the
    /// handle is a resource name that can be passed to ByteStream or other
    /// accessors.
    ///
    /// An implementation may define some logs by default (like `stdout`), and may
    /// allow clients to add new logs via AddTaskLog.
    #[prost(map = "string, string", tag = "3")]
    pub logs: ::std::collections::HashMap<std::string::String, std::string::String>,
}
/// DEPRECATED - use Lease.assignment_result instead.
/// The result and metadata of the task.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskResult {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The result may be updated several times; the client must only set
    /// `complete` to true to indicate that no further updates are allowed.
    /// If this is not true, the `status` field must not be examined since its zero
    /// value is equivalent to `OK`.
    ///
    /// Once a task is completed, it must not be updated with further results,
    /// though the implementation may choose to continue to receive logs.
    #[prost(bool, tag = "2")]
    pub complete: bool,
    /// The final status of the task itself. For example, if task.description
    /// included a timeout which was violated, status.code may be
    /// DEADLINE_EXCEEDED. This field can only be read if `complete` is true.
    #[prost(message, optional, tag = "3")]
    pub status: ::std::option::Option<super::super::super::rpc::Status>,
    /// Any non-log output, such as output files and exit codes. See
    /// CommandResult as an example.
    #[prost(message, optional, tag = "4")]
    pub output: ::std::option::Option<::prost_types::Any>,
    /// Any information about how the command was executed, eg runtime. See
    /// CommandOverhead as an example.
    #[prost(message, optional, tag = "5")]
    pub meta: ::std::option::Option<::prost_types::Any>,
}
/// Request message for `GetTask`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTaskRequest {
    /// Required. The task name.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for `UpdateTaskResult`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateTaskResultRequest {
    /// Required. The task result name; must match `result.name`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The result being updated.
    #[prost(message, optional, tag = "2")]
    pub result: ::std::option::Option<TaskResult>,
    /// Required. The fields within `result` that are specified.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// Required. If this is being updated by a bot from BotManager, the source should be
    /// bot.session_id. That way, if two bots accidentally get the same name, we'll
    /// know to reject updates from the older one.
    #[prost(string, tag = "4")]
    pub source: std::string::String,
}
/// Request message for `AddTaskLog`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTaskLogRequest {
    /// Required. The name of the task that will own the new log.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The human-readable name of the log, like `stdout` or a relative file path.
    #[prost(string, tag = "2")]
    pub log_id: std::string::String,
}
/// Response message for `AddTaskLog`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddTaskLogResponse {
    /// The handle for the new log, as would be returned in Task.logs.
    #[prost(string, tag = "1")]
    pub handle: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod tasks_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " DEPRECATED. GetTask should be replaced by Lease.payload, UpdateTaskResult by"]
    #[doc = " Lease.result and logs should be precreated prior to sending to the bot (eg,"]
    #[doc = " via CommandTask.expected_outputs.stdout_destination)."]
    pub struct TasksClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl TasksClient<tonic::transport::Channel> {
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
    impl<T> TasksClient<T>
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
        #[doc = " DEPRECATED - use Lease.payload instead."]
        #[doc = " GetTask reads the current state of the task. Tasks must be created through"]
        #[doc = " some other interface, and should be immutable once created and exposed to"]
        #[doc = " the bots."]
        pub async fn get_task(
            &mut self,
            request: impl tonic::IntoRequest<super::GetTaskRequest>,
        ) -> Result<tonic::Response<super::Task>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.remoteworkers.v1test2.Tasks/GetTask",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " DEPRECATED - use Lease.result instead."]
        #[doc = " UpdateTaskResult updates the result."]
        pub async fn update_task_result(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateTaskResultRequest>,
        ) -> Result<tonic::Response<super::TaskResult>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.remoteworkers.v1test2.Tasks/UpdateTaskResult",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " DEPRECATED - precreate logs prior to sending to bot."]
        #[doc = " AddTaskLog creates a new streaming log. The log is streamed and marked as"]
        #[doc = " completed through other interfaces (i.e., ByteStream). This can be called"]
        #[doc = " by the bot if it wants to create a new log; the server can also predefine"]
        #[doc = " logs that do not need to be created (e.g. `stdout`)."]
        pub async fn add_task_log(
            &mut self,
            request: impl tonic::IntoRequest<super::AddTaskLogRequest>,
        ) -> Result<tonic::Response<super::AddTaskLogResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.devtools.remoteworkers.v1test2.Tasks/AddTaskLog",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for TasksClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for TasksClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TasksClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod tasks_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with TasksServer."]
    #[async_trait]
    pub trait Tasks: Send + Sync + 'static {
        #[doc = " DEPRECATED - use Lease.payload instead."]
        #[doc = " GetTask reads the current state of the task. Tasks must be created through"]
        #[doc = " some other interface, and should be immutable once created and exposed to"]
        #[doc = " the bots."]
        async fn get_task(
            &self,
            request: tonic::Request<super::GetTaskRequest>,
        ) -> Result<tonic::Response<super::Task>, tonic::Status>;
        #[doc = " DEPRECATED - use Lease.result instead."]
        #[doc = " UpdateTaskResult updates the result."]
        async fn update_task_result(
            &self,
            request: tonic::Request<super::UpdateTaskResultRequest>,
        ) -> Result<tonic::Response<super::TaskResult>, tonic::Status>;
        #[doc = " DEPRECATED - precreate logs prior to sending to bot."]
        #[doc = " AddTaskLog creates a new streaming log. The log is streamed and marked as"]
        #[doc = " completed through other interfaces (i.e., ByteStream). This can be called"]
        #[doc = " by the bot if it wants to create a new log; the server can also predefine"]
        #[doc = " logs that do not need to be created (e.g. `stdout`)."]
        async fn add_task_log(
            &self,
            request: tonic::Request<super::AddTaskLogRequest>,
        ) -> Result<tonic::Response<super::AddTaskLogResponse>, tonic::Status>;
    }
    #[doc = " DEPRECATED. GetTask should be replaced by Lease.payload, UpdateTaskResult by"]
    #[doc = " Lease.result and logs should be precreated prior to sending to the bot (eg,"]
    #[doc = " via CommandTask.expected_outputs.stdout_destination)."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct TasksServer<T: Tasks> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Tasks> TasksServer<T> {
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
    impl<T, B> Service<http::Request<B>> for TasksServer<T>
    where
        T: Tasks,
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
                "/google.devtools.remoteworkers.v1test2.Tasks/GetTask" => {
                    #[allow(non_camel_case_types)]
                    struct GetTaskSvc<T: Tasks>(pub Arc<T>);
                    impl<T: Tasks> tonic::server::UnaryService<super::GetTaskRequest> for GetTaskSvc<T> {
                        type Response = super::Task;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetTaskRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_task(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetTaskSvc(inner);
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
                "/google.devtools.remoteworkers.v1test2.Tasks/UpdateTaskResult" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateTaskResultSvc<T: Tasks>(pub Arc<T>);
                    impl<T: Tasks> tonic::server::UnaryService<super::UpdateTaskResultRequest>
                        for UpdateTaskResultSvc<T>
                    {
                        type Response = super::TaskResult;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateTaskResultRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_task_result(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateTaskResultSvc(inner);
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
                "/google.devtools.remoteworkers.v1test2.Tasks/AddTaskLog" => {
                    #[allow(non_camel_case_types)]
                    struct AddTaskLogSvc<T: Tasks>(pub Arc<T>);
                    impl<T: Tasks> tonic::server::UnaryService<super::AddTaskLogRequest> for AddTaskLogSvc<T> {
                        type Response = super::AddTaskLogResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddTaskLogRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.add_task_log(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = AddTaskLogSvc(inner);
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
    impl<T: Tasks> Clone for TasksServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Tasks> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Tasks> tonic::transport::NamedService for TasksServer<T> {
        const NAME: &'static str = "google.devtools.remoteworkers.v1test2.Tasks";
    }
}
