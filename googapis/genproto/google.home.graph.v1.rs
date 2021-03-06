/// Third-party device definition.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Device {
    /// Third-party device ID.
    #[prost(string, tag = "1")]
    pub id: std::string::String,
    /// Hardware type of the device.
    /// See [device
    /// types](https://developers.google.com/assistant/smarthome/guides).
    #[prost(string, tag = "2")]
    pub r#type: std::string::String,
    /// Traits supported by the device.
    /// See [device
    /// traits](https://developers.google.com/assistant/smarthome/traits).
    #[prost(string, repeated, tag = "3")]
    pub traits: ::std::vec::Vec<std::string::String>,
    /// Names given to this device by your smart home Action.
    #[prost(message, optional, tag = "4")]
    pub name: ::std::option::Option<DeviceNames>,
    /// Indicates whether your smart home Action will report state of this device
    /// to Google via
    /// [ReportStateAndNotification][google.home.graph.v1.HomeGraphApiService.ReportStateAndNotification].
    #[prost(bool, tag = "5")]
    pub will_report_state: bool,
    /// Suggested name for the room where this device is installed.
    /// Google attempts to use this value during user setup.
    #[prost(string, tag = "6")]
    pub room_hint: std::string::String,
    /// Suggested name for the structure where this device is installed.
    /// Google attempts to use this value during user setup.
    #[prost(string, tag = "7")]
    pub structure_hint: std::string::String,
    /// Device manufacturer, model, hardware version, and software version.
    #[prost(message, optional, tag = "8")]
    pub device_info: ::std::option::Option<DeviceInfo>,
    /// Attributes for the traits supported by the device.
    #[prost(message, optional, tag = "9")]
    pub attributes: ::std::option::Option<::prost_types::Struct>,
    /// Custom device attributes stored in Home Graph and provided to your
    /// smart home Action in each
    /// [QUERY](https://developers.google.com/assistant/smarthome/reference/intent/query)
    /// and
    /// [EXECUTE](https://developers.google.com/assistant/smarthome/reference/intent/execute)
    /// intent.
    #[prost(message, optional, tag = "10")]
    pub custom_data: ::std::option::Option<::prost_types::Struct>,
    /// Alternate IDs associated with this device.
    /// This is used to identify cloud synced devices enabled for [local
    /// fulfillment](https://developers.google.com/assistant/smarthome/concepts/local).
    #[prost(message, repeated, tag = "11")]
    pub other_device_ids: ::std::vec::Vec<AgentOtherDeviceId>,
    /// Indicates whether your smart home Action will report notifications
    /// to Google for this device via
    /// [ReportStateAndNotification][google.home.graph.v1.HomeGraphApiService.ReportStateAndNotification].
    ///
    /// If your smart home Action enables users to control device notifications,
    /// you should update this field and call
    /// [RequestSyncDevices][google.home.graph.v1.HomeGraphApiService.RequestSyncDevices].
    #[prost(bool, tag = "12")]
    pub notification_supported_by_agent: bool,
}
/// Identifiers used to describe the device.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceNames {
    /// Primary name of the device, generally provided by the user.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Additional names provided by the user for the device.
    #[prost(string, repeated, tag = "2")]
    pub nicknames: ::std::vec::Vec<std::string::String>,
    /// List of names provided by the manufacturer rather than the user, such as
    /// serial numbers, SKUs, etc.
    #[prost(string, repeated, tag = "3")]
    pub default_names: ::std::vec::Vec<std::string::String>,
}
/// Device information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeviceInfo {
    /// Device manufacturer.
    #[prost(string, tag = "1")]
    pub manufacturer: std::string::String,
    /// Device model.
    #[prost(string, tag = "2")]
    pub model: std::string::String,
    /// Device hardware version.
    #[prost(string, tag = "3")]
    pub hw_version: std::string::String,
    /// Device software version.
    #[prost(string, tag = "4")]
    pub sw_version: std::string::String,
}
/// Alternate third-party device ID.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgentOtherDeviceId {
    /// Project ID for your smart home Action.
    #[prost(string, tag = "1")]
    pub agent_id: std::string::String,
    /// Unique third-party device ID.
    #[prost(string, tag = "2")]
    pub device_id: std::string::String,
}
/// Request type for the
/// [`RequestSyncDevices`](#google.home.graph.v1.HomeGraphApiService.RequestSyncDevices)
/// call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestSyncDevicesRequest {
    /// Required. Third-party user ID.
    #[prost(string, tag = "1")]
    pub agent_user_id: std::string::String,
    /// Optional. If set, the request will be added to a queue and a response will
    /// be returned immediately. This enables concurrent requests for the given
    /// `agent_user_id`, but the caller will not receive any error responses.
    #[prost(bool, tag = "2")]
    pub r#async: bool,
}
/// Response type for the
/// [`RequestSyncDevices`](#google.home.graph.v1.HomeGraphApiService.RequestSyncDevices)
/// call.
///
/// Intentionally empty upon success. An HTTP response code is returned
/// with more details upon failure.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RequestSyncDevicesResponse {}
/// Request type for the
/// [`ReportStateAndNotification`](#google.home.graph.v1.HomeGraphApiService.ReportStateAndNotification)
/// call. It may include states, notifications, or both. States and notifications
/// are defined per `device_id` (for example, "123" and "456" in the following
/// example).
/// # Example
///
/// ```json
/// {
///   "requestId": "ff36a3cc-ec34-11e6-b1a0-64510650abcf",
///   "agentUserId": "1234",
///   "payload": {
///     "devices": {
///       "states": {
///         "123": {
///           "on": true
///         },
///         "456": {
///           "on": true,
///           "brightness": 10
///         }
///       },
///     }
///   }
/// }
/// ```
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportStateAndNotificationRequest {
    /// Request ID used for debugging.
    #[prost(string, tag = "1")]
    pub request_id: std::string::String,
    /// Unique identifier per event (for example, a doorbell press).
    #[prost(string, tag = "4")]
    pub event_id: std::string::String,
    /// Required. Third-party user ID.
    #[prost(string, tag = "2")]
    pub agent_user_id: std::string::String,
    /// Token to maintain state in the follow up notification response.
    #[prost(string, tag = "5")]
    pub follow_up_token: std::string::String,
    /// Required. State of devices to update and notification metadata for devices.
    #[prost(message, optional, tag = "3")]
    pub payload: ::std::option::Option<StateAndNotificationPayload>,
}
/// Response type for the
/// [`ReportStateAndNotification`](#google.home.graph.v1.HomeGraphApiService.ReportStateAndNotification)
/// call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportStateAndNotificationResponse {
    /// Request ID copied from [ReportStateAndNotificationRequest][google.home.graph.v1.ReportStateAndNotificationRequest].
    #[prost(string, tag = "1")]
    pub request_id: std::string::String,
}
/// Payload containing the state and notification information for devices.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateAndNotificationPayload {
    /// The devices for updating state and sending notifications.
    #[prost(message, optional, tag = "1")]
    pub devices: ::std::option::Option<ReportStateAndNotificationDevice>,
}
/// The states and notifications specific to a device.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReportStateAndNotificationDevice {
    /// States of devices to update. See the **Device STATES** section
    /// of the individual trait [reference
    /// guides](https://developers.google.com/assistant/smarthome/traits).
    #[prost(message, optional, tag = "1")]
    pub states: ::std::option::Option<::prost_types::Struct>,
    /// Notifications metadata for devices. See the **Device NOTIFICATIONS**
    /// section of the individual trait [reference
    /// guides](https://developers.google.com/assistant/smarthome/traits).
    #[prost(message, optional, tag = "2")]
    pub notifications: ::std::option::Option<::prost_types::Struct>,
}
/// Request type for the
/// [`DeleteAgentUser`](#google.home.graph.v1.HomeGraphApiService.DeleteAgentUser)
/// call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAgentUserRequest {
    /// Request ID used for debugging.
    #[prost(string, tag = "1")]
    pub request_id: std::string::String,
    /// Required. Third-party user ID.
    #[prost(string, tag = "2")]
    pub agent_user_id: std::string::String,
}
/// Request type for the
/// [`Query`](#google.home.graph.v1.HomeGraphApiService.Query) call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRequest {
    /// Request ID used for debugging.
    #[prost(string, tag = "1")]
    pub request_id: std::string::String,
    /// Required. Third-party user ID.
    #[prost(string, tag = "2")]
    pub agent_user_id: std::string::String,
    /// Required. Inputs containing third-party device IDs for which to
    /// get the device states.
    #[prost(message, repeated, tag = "3")]
    pub inputs: ::std::vec::Vec<QueryRequestInput>,
}
/// Device ID inputs to [QueryRequest][google.home.graph.v1.QueryRequest].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRequestInput {
    /// Payload containing third-party device IDs.
    #[prost(message, optional, tag = "1")]
    pub payload: ::std::option::Option<QueryRequestPayload>,
}
/// Payload containing device IDs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRequestPayload {
    /// Third-party device IDs for which to get the device states.
    #[prost(message, repeated, tag = "1")]
    pub devices: ::std::vec::Vec<AgentDeviceId>,
}
/// Third-party device ID for one device.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AgentDeviceId {
    /// Third-party device ID.
    #[prost(string, tag = "1")]
    pub id: std::string::String,
}
/// Response type for the
/// [`Query`](#google.home.graph.v1.HomeGraphApiService.Query) call.
/// This should follow the same format as the Google smart home
/// `action.devices.QUERY`
/// [response](https://developers.google.com/assistant/smarthome/reference/intent/query).
/// # Example
///
/// ```json
/// {
///   "requestId": "ff36a3cc-ec34-11e6-b1a0-64510650abcf",
///   "payload": {
///     "devices": {
///       "123": {
///         "on": true,
///         "online": true
///       },
///       "456": {
///         "on": true,
///         "online": true,
///         "brightness": 80,
///         "color": {
///           "name": "cerulean",
///           "spectrumRGB": 31655
///         }
///       }
///     }
///   }
/// }
/// ```
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResponse {
    /// Request ID used for debugging. Copied from the request.
    #[prost(string, tag = "1")]
    pub request_id: std::string::String,
    /// Device states for the devices given in the request.
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<QueryResponsePayload>,
}
/// Payload containing device states information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResponsePayload {
    /// States of the devices. Map of third-party device ID to struct of device
    /// states.
    #[prost(map = "string, message", tag = "1")]
    pub devices: ::std::collections::HashMap<std::string::String, ::prost_types::Struct>,
}
/// Request type for the [`Sync`](#google.home.graph.v1.HomeGraphApiService.Sync)
/// call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncRequest {
    /// Request ID used for debugging.
    #[prost(string, tag = "1")]
    pub request_id: std::string::String,
    /// Required. Third-party user ID.
    #[prost(string, tag = "2")]
    pub agent_user_id: std::string::String,
}
/// Response type for the
/// [`Sync`](#google.home.graph.v1.HomeGraphApiService.Sync) call.
/// This should follow the same format as the Google smart home
/// `action.devices.SYNC`
/// [response](https://developers.google.com/assistant/smarthome/reference/intent/sync).
/// # Example
///
/// ```json
/// {
///   "requestId": "ff36a3cc-ec34-11e6-b1a0-64510650abcf",
///   "payload": {
///     "agentUserId": "1836.15267389",
///     "devices": [{
///       "id": "123",
///       "type": "action.devices.types.OUTLET",
///       "traits": [
///         "action.devices.traits.OnOff"
///       ],
///       "name": {
///         "defaultNames": ["My Outlet 1234"],
///         "name": "Night light",
///         "nicknames": ["wall plug"]
///       },
///       "willReportState": false,
///       "deviceInfo": {
///         "manufacturer": "lights-out-inc",
///         "model": "hs1234",
///         "hwVersion": "3.2",
///         "swVersion": "11.4"
///       },
///       "customData": {
///         "fooValue": 74,
///         "barValue": true,
///         "bazValue": "foo"
///       }
///     }]
///   }
/// }
/// ```
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncResponse {
    /// Request ID used for debugging. Copied from the request.
    #[prost(string, tag = "1")]
    pub request_id: std::string::String,
    /// Devices associated with the third-party user.
    #[prost(message, optional, tag = "2")]
    pub payload: ::std::option::Option<SyncResponsePayload>,
}
/// Payload containing device information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SyncResponsePayload {
    /// Third-party user ID
    #[prost(string, tag = "1")]
    pub agent_user_id: std::string::String,
    /// Devices associated with the third-party user.
    #[prost(message, repeated, tag = "2")]
    pub devices: ::std::vec::Vec<Device>,
}
#[doc = r" Generated client implementations."]
pub mod home_graph_api_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Google Home Graph API service. The Home Graph service provides support for"]
    #[doc = " accessing first-party and third-party devices stored in Google's Home Graph."]
    #[doc = " The Home Graph database provides contextual data about the relationships"]
    #[doc = " between devices and the home."]
    #[doc = ""]
    #[doc = " For more details, see the [Home Graph developer"]
    #[doc = " guide](https://developers.google.com/assistant/smarthome/concepts/homegraph)."]
    pub struct HomeGraphApiServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl HomeGraphApiServiceClient<tonic::transport::Channel> {
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
    impl<T> HomeGraphApiServiceClient<T>
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
        #[doc = " Requests Google to send an `action.devices.SYNC`"]
        #[doc = " [intent](https://developers.google.com/assistant/smarthome/reference/intent/sync)"]
        #[doc = " to your smart home Action to update device metadata for the given user."]
        #[doc = ""]
        #[doc = ""]
        #[doc = " The third-party user's identity is passed via the `agent_user_id`"]
        #[doc = " (see [RequestSyncDevicesRequest][google.home.graph.v1.RequestSyncDevicesRequest])."]
        #[doc = " This request must be authorized using service account credentials from your"]
        #[doc = " Actions console project."]
        pub async fn request_sync_devices(
            &mut self,
            request: impl tonic::IntoRequest<super::RequestSyncDevicesRequest>,
        ) -> Result<tonic::Response<super::RequestSyncDevicesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.home.graph.v1.HomeGraphApiService/RequestSyncDevices",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Reports device state and optionally sends device notifications."]
        #[doc = " Called by your smart home Action when the state of a third-party device"]
        #[doc = " changes or you need to send a notification about the device."]
        #[doc = " See [Implement Report"]
        #[doc = " State](https://developers.google.com/assistant/smarthome/develop/report-state)"]
        #[doc = " for more information."]
        #[doc = ""]
        #[doc = " This method updates the device state according to its declared"]
        #[doc = " [traits](https://developers.google.com/assistant/smarthome/concepts/devices-traits)."]
        #[doc = " Publishing a new state value outside of these traits will result in an"]
        #[doc = " `INVALID_ARGUMENT` error response."]
        #[doc = ""]
        #[doc = " The third-party user's identity is passed in via the `agent_user_id`"]
        #[doc = " (see [ReportStateAndNotificationRequest][google.home.graph.v1.ReportStateAndNotificationRequest])."]
        #[doc = " This request must be authorized using service account credentials from your"]
        #[doc = " Actions console project."]
        pub async fn report_state_and_notification(
            &mut self,
            request: impl tonic::IntoRequest<super::ReportStateAndNotificationRequest>,
        ) -> Result<tonic::Response<super::ReportStateAndNotificationResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.home.graph.v1.HomeGraphApiService/ReportStateAndNotification",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Unlinks the given third-party user from your smart home Action."]
        #[doc = " All data related to this user will be deleted."]
        #[doc = ""]
        #[doc = " For more details on how users link their accounts, see"]
        #[doc = " [fulfillment and"]
        #[doc = " authentication](https://developers.google.com/assistant/smarthome/concepts/fulfillment-authentication)."]
        #[doc = ""]
        #[doc = " The third-party user's identity is passed in via the `agent_user_id`"]
        #[doc = " (see [DeleteAgentUserRequest][google.home.graph.v1.DeleteAgentUserRequest])."]
        #[doc = " This request must be authorized using service account credentials from your"]
        #[doc = " Actions console project."]
        pub async fn delete_agent_user(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAgentUserRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.home.graph.v1.HomeGraphApiService/DeleteAgentUser",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets the current states in Home Graph for the given set of the third-party"]
        #[doc = " user's devices."]
        #[doc = ""]
        #[doc = " The third-party user's identity is passed in via the `agent_user_id`"]
        #[doc = " (see [QueryRequest][google.home.graph.v1.QueryRequest])."]
        #[doc = " This request must be authorized using service account credentials from your"]
        #[doc = " Actions console project."]
        pub async fn query(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryRequest>,
        ) -> Result<tonic::Response<super::QueryResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.home.graph.v1.HomeGraphApiService/Query",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets all the devices associated with the given third-party user."]
        #[doc = ""]
        #[doc = " The third-party user's identity is passed in via the `agent_user_id`"]
        #[doc = " (see [SyncRequest][google.home.graph.v1.SyncRequest])."]
        #[doc = " This request must be authorized using service account credentials from your"]
        #[doc = " Actions console project."]
        pub async fn sync(
            &mut self,
            request: impl tonic::IntoRequest<super::SyncRequest>,
        ) -> Result<tonic::Response<super::SyncResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.home.graph.v1.HomeGraphApiService/Sync",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for HomeGraphApiServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for HomeGraphApiServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "HomeGraphApiServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod home_graph_api_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with HomeGraphApiServiceServer."]
    #[async_trait]
    pub trait HomeGraphApiService: Send + Sync + 'static {
        #[doc = " Requests Google to send an `action.devices.SYNC`"]
        #[doc = " [intent](https://developers.google.com/assistant/smarthome/reference/intent/sync)"]
        #[doc = " to your smart home Action to update device metadata for the given user."]
        #[doc = ""]
        #[doc = ""]
        #[doc = " The third-party user's identity is passed via the `agent_user_id`"]
        #[doc = " (see [RequestSyncDevicesRequest][google.home.graph.v1.RequestSyncDevicesRequest])."]
        #[doc = " This request must be authorized using service account credentials from your"]
        #[doc = " Actions console project."]
        async fn request_sync_devices(
            &self,
            request: tonic::Request<super::RequestSyncDevicesRequest>,
        ) -> Result<tonic::Response<super::RequestSyncDevicesResponse>, tonic::Status>;
        #[doc = " Reports device state and optionally sends device notifications."]
        #[doc = " Called by your smart home Action when the state of a third-party device"]
        #[doc = " changes or you need to send a notification about the device."]
        #[doc = " See [Implement Report"]
        #[doc = " State](https://developers.google.com/assistant/smarthome/develop/report-state)"]
        #[doc = " for more information."]
        #[doc = ""]
        #[doc = " This method updates the device state according to its declared"]
        #[doc = " [traits](https://developers.google.com/assistant/smarthome/concepts/devices-traits)."]
        #[doc = " Publishing a new state value outside of these traits will result in an"]
        #[doc = " `INVALID_ARGUMENT` error response."]
        #[doc = ""]
        #[doc = " The third-party user's identity is passed in via the `agent_user_id`"]
        #[doc = " (see [ReportStateAndNotificationRequest][google.home.graph.v1.ReportStateAndNotificationRequest])."]
        #[doc = " This request must be authorized using service account credentials from your"]
        #[doc = " Actions console project."]
        async fn report_state_and_notification(
            &self,
            request: tonic::Request<super::ReportStateAndNotificationRequest>,
        ) -> Result<tonic::Response<super::ReportStateAndNotificationResponse>, tonic::Status>;
        #[doc = " Unlinks the given third-party user from your smart home Action."]
        #[doc = " All data related to this user will be deleted."]
        #[doc = ""]
        #[doc = " For more details on how users link their accounts, see"]
        #[doc = " [fulfillment and"]
        #[doc = " authentication](https://developers.google.com/assistant/smarthome/concepts/fulfillment-authentication)."]
        #[doc = ""]
        #[doc = " The third-party user's identity is passed in via the `agent_user_id`"]
        #[doc = " (see [DeleteAgentUserRequest][google.home.graph.v1.DeleteAgentUserRequest])."]
        #[doc = " This request must be authorized using service account credentials from your"]
        #[doc = " Actions console project."]
        async fn delete_agent_user(
            &self,
            request: tonic::Request<super::DeleteAgentUserRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Gets the current states in Home Graph for the given set of the third-party"]
        #[doc = " user's devices."]
        #[doc = ""]
        #[doc = " The third-party user's identity is passed in via the `agent_user_id`"]
        #[doc = " (see [QueryRequest][google.home.graph.v1.QueryRequest])."]
        #[doc = " This request must be authorized using service account credentials from your"]
        #[doc = " Actions console project."]
        async fn query(
            &self,
            request: tonic::Request<super::QueryRequest>,
        ) -> Result<tonic::Response<super::QueryResponse>, tonic::Status>;
        #[doc = " Gets all the devices associated with the given third-party user."]
        #[doc = ""]
        #[doc = " The third-party user's identity is passed in via the `agent_user_id`"]
        #[doc = " (see [SyncRequest][google.home.graph.v1.SyncRequest])."]
        #[doc = " This request must be authorized using service account credentials from your"]
        #[doc = " Actions console project."]
        async fn sync(
            &self,
            request: tonic::Request<super::SyncRequest>,
        ) -> Result<tonic::Response<super::SyncResponse>, tonic::Status>;
    }
    #[doc = " Google Home Graph API service. The Home Graph service provides support for"]
    #[doc = " accessing first-party and third-party devices stored in Google's Home Graph."]
    #[doc = " The Home Graph database provides contextual data about the relationships"]
    #[doc = " between devices and the home."]
    #[doc = ""]
    #[doc = " For more details, see the [Home Graph developer"]
    #[doc = " guide](https://developers.google.com/assistant/smarthome/concepts/homegraph)."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct HomeGraphApiServiceServer<T: HomeGraphApiService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: HomeGraphApiService> HomeGraphApiServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for HomeGraphApiServiceServer<T>
    where
        T: HomeGraphApiService,
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
                "/google.home.graph.v1.HomeGraphApiService/RequestSyncDevices" => {
                    #[allow(non_camel_case_types)]
                    struct RequestSyncDevicesSvc<T: HomeGraphApiService>(pub Arc<T>);
                    impl<T: HomeGraphApiService>
                        tonic::server::UnaryService<super::RequestSyncDevicesRequest>
                        for RequestSyncDevicesSvc<T>
                    {
                        type Response = super::RequestSyncDevicesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RequestSyncDevicesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.request_sync_devices(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RequestSyncDevicesSvc(inner);
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
                "/google.home.graph.v1.HomeGraphApiService/ReportStateAndNotification" => {
                    #[allow(non_camel_case_types)]
                    struct ReportStateAndNotificationSvc<T: HomeGraphApiService>(pub Arc<T>);
                    impl<T: HomeGraphApiService>
                        tonic::server::UnaryService<super::ReportStateAndNotificationRequest>
                        for ReportStateAndNotificationSvc<T>
                    {
                        type Response = super::ReportStateAndNotificationResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ReportStateAndNotificationRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.report_state_and_notification(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ReportStateAndNotificationSvc(inner);
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
                "/google.home.graph.v1.HomeGraphApiService/DeleteAgentUser" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteAgentUserSvc<T: HomeGraphApiService>(pub Arc<T>);
                    impl<T: HomeGraphApiService>
                        tonic::server::UnaryService<super::DeleteAgentUserRequest>
                        for DeleteAgentUserSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteAgentUserRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_agent_user(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteAgentUserSvc(inner);
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
                "/google.home.graph.v1.HomeGraphApiService/Query" => {
                    #[allow(non_camel_case_types)]
                    struct QuerySvc<T: HomeGraphApiService>(pub Arc<T>);
                    impl<T: HomeGraphApiService> tonic::server::UnaryService<super::QueryRequest> for QuerySvc<T> {
                        type Response = super::QueryResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.query(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = QuerySvc(inner);
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
                "/google.home.graph.v1.HomeGraphApiService/Sync" => {
                    #[allow(non_camel_case_types)]
                    struct SyncSvc<T: HomeGraphApiService>(pub Arc<T>);
                    impl<T: HomeGraphApiService> tonic::server::UnaryService<super::SyncRequest> for SyncSvc<T> {
                        type Response = super::SyncResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SyncRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.sync(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SyncSvc(inner);
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
    impl<T: HomeGraphApiService> Clone for HomeGraphApiServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: HomeGraphApiService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: HomeGraphApiService> tonic::transport::NamedService for HomeGraphApiServiceServer<T> {
        const NAME: &'static str = "google.home.graph.v1.HomeGraphApiService";
    }
}
