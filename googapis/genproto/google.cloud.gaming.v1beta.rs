/// Represents the metadata of the long-running operation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationMetadata {
    /// Output only. The time the operation was created.
    #[prost(message, optional, tag = "1")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The time the operation finished running.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. Server-defined resource path for the target of the operation.
    #[prost(string, tag = "3")]
    pub target: std::string::String,
    /// Output only. Name of the verb executed by the operation.
    #[prost(string, tag = "4")]
    pub verb: std::string::String,
    /// Output only. Human-readable status of the operation, if any.
    #[prost(string, tag = "5")]
    pub status_message: std::string::String,
    /// Output only. Identifies whether the user has requested cancellation
    /// of the operation. Operations that have successfully been cancelled
    /// have [Operation.error][] value with a [google.rpc.Status.code][google.rpc.Status.code] of 1,
    /// corresponding to `Code.CANCELLED`.
    #[prost(bool, tag = "6")]
    pub requested_cancellation: bool,
    /// Output only. API version used to start the operation.
    #[prost(string, tag = "7")]
    pub api_version: std::string::String,
    /// Output only. List of Locations that could not be reached.
    #[prost(string, repeated, tag = "8")]
    pub unreachable: ::std::vec::Vec<std::string::String>,
    /// Output only. Operation status for gameservices API operations. Operation status is in
    /// the form of key-value pairs where keys are resource IDs and the values show
    /// the status of the operation. In case of failures, the value includes an
    /// error code and error message.
    #[prost(map = "string, message", tag = "9")]
    pub operation_status: ::std::collections::HashMap<std::string::String, OperationStatus>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OperationStatus {
    /// Output only. Whether the operation is done or still in progress.
    #[prost(bool, tag = "1")]
    pub done: bool,
    /// The error code in case of failures.
    #[prost(enumeration = "operation_status::ErrorCode", tag = "2")]
    pub error_code: i32,
    /// The human-readable error message.
    #[prost(string, tag = "3")]
    pub error_message: std::string::String,
}
pub mod operation_status {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ErrorCode {
        Unspecified = 0,
        InternalError = 1,
        PermissionDenied = 2,
        ClusterConnection = 3,
    }
}
/// The label selector, used to group labels on the resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LabelSelector {
    /// Resource labels for this selector.
    #[prost(map = "string, string", tag = "1")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
}
/// The Realm selector, used to match Realm resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RealmSelector {
    /// List of Realms to match.
    #[prost(string, repeated, tag = "1")]
    pub realms: ::std::vec::Vec<std::string::String>,
}
/// The schedule of a recurring or one time event. The event's time span is
/// specified by start_time and end_time. If the scheduled event's timespan is
/// larger than the cron_spec + cron_job_duration, the event will be recurring.
/// If only cron_spec + cron_job_duration are specified, the event is effective
/// starting at the local time specified by cron_spec, and is recurring.
///
///   start_time|-------[cron job]-------[cron job]-------[cron job]---|end_time
///   cron job: cron spec start time + duration
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Schedule {
    /// The start time of the event.
    #[prost(message, optional, tag = "1")]
    pub start_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The end time of the event.
    #[prost(message, optional, tag = "2")]
    pub end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The duration for the cron job event. The duration of the event is effective
    /// after the cron job's start time.
    #[prost(message, optional, tag = "3")]
    pub cron_job_duration: ::std::option::Option<::prost_types::Duration>,
    /// The cron definition of the scheduled event. See
    /// https://en.wikipedia.org/wiki/Cron. Cron spec specifies the local time as
    /// defined by the Realm.
    #[prost(string, tag = "4")]
    pub cron_spec: std::string::String,
}
/// Encapsulates Agones fleet spec and Agones autoscaler spec sources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpecSource {
    /// The Game Server Config resource. Uses the form:
    ///
    /// `projects/{project}/locations/{location}/gameServerDeployments/{deployment_id}/configs/{config_id}`.
    #[prost(string, tag = "1")]
    pub game_server_config_name: std::string::String,
    /// The name of the Agones leet config or Agones scaling config used to derive
    /// the Agones fleet or Agones autoscaler spec.
    #[prost(string, tag = "2")]
    pub name: std::string::String,
}
/// Details about the Agones resources.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetDetails {
    /// The Game Server Cluster name. Uses the form:
    ///
    /// `projects/{project}/locations/{location}/realms/{realm}/gameServerClusters/{cluster}`.
    #[prost(string, tag = "1")]
    pub game_server_cluster_name: std::string::String,
    /// The Game Server Deployment name. Uses the form:
    ///
    /// `projects/{project}/locations/{location}/gameServerDeployments/{deployment_id}`.
    #[prost(string, tag = "2")]
    pub game_server_deployment_name: std::string::String,
    /// Agones fleet details for Game Server Clusters and Game Server Deployments.
    #[prost(message, repeated, tag = "3")]
    pub fleet_details: ::std::vec::Vec<target_details::TargetFleetDetails>,
}
pub mod target_details {
    /// Details of the target Agones fleet.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TargetFleetDetails {
        /// Reference to target Agones fleet.
        #[prost(message, optional, tag = "1")]
        pub fleet: ::std::option::Option<target_fleet_details::TargetFleet>,
        /// Reference to target Agones fleet autoscaling policy.
        #[prost(message, optional, tag = "2")]
        pub autoscaler: ::std::option::Option<target_fleet_details::TargetFleetAutoscaler>,
    }
    pub mod target_fleet_details {
        /// Target Agones fleet specification.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TargetFleet {
            /// The name of the Agones fleet.
            #[prost(string, tag = "1")]
            pub name: std::string::String,
            /// Encapsulates the source of the Agones fleet spec.
            /// The Agones fleet spec source.
            #[prost(message, optional, tag = "2")]
            pub spec_source: ::std::option::Option<super::super::SpecSource>,
        }
        /// Target Agones autoscaler policy reference.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TargetFleetAutoscaler {
            /// The name of the Agones autoscaler.
            #[prost(string, tag = "1")]
            pub name: std::string::String,
            /// Encapsulates the source of the Agones fleet spec.
            /// Details about the Agones autoscaler spec.
            #[prost(message, optional, tag = "2")]
            pub spec_source: ::std::option::Option<super::super::SpecSource>,
        }
    }
}
/// Encapsulates the Target state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TargetState {
    /// Details about Agones fleets.
    #[prost(message, repeated, tag = "1")]
    pub details: ::std::vec::Vec<TargetDetails>,
}
/// Details of the deployed Agones fleet.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeployedFleetDetails {
    /// Information about the Agones fleet.
    #[prost(message, optional, tag = "1")]
    pub deployed_fleet: ::std::option::Option<deployed_fleet_details::DeployedFleet>,
    /// Information about the Agones autoscaler for that fleet.
    #[prost(message, optional, tag = "2")]
    pub deployed_autoscaler: ::std::option::Option<deployed_fleet_details::DeployedFleetAutoscaler>,
}
pub mod deployed_fleet_details {
    /// Agones fleet specification and details.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeployedFleet {
        /// The name of the Agones fleet.
        #[prost(string, tag = "1")]
        pub fleet: std::string::String,
        /// The fleet spec retrieved from the Agones fleet.
        #[prost(string, tag = "2")]
        pub fleet_spec: std::string::String,
        /// The source spec that is used to create the Agones fleet.
        /// The GameServerConfig resource may no longer exist in the system.
        #[prost(message, optional, tag = "3")]
        pub spec_source: ::std::option::Option<super::SpecSource>,
        /// The current status of the Agones fleet.
        /// Includes count of game servers in various states.
        #[prost(message, optional, tag = "5")]
        pub status: ::std::option::Option<deployed_fleet::DeployedFleetStatus>,
    }
    pub mod deployed_fleet {
        /// DeployedFleetStatus has details about the Agones fleets such as how many
        /// are running, how many allocated, and so on.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DeployedFleetStatus {
            /// The number of GameServer replicas in the READY state in this fleet.
            #[prost(int64, tag = "1")]
            pub ready_replicas: i64,
            /// The number of GameServer replicas in the ALLOCATED state in this fleet.
            #[prost(int64, tag = "2")]
            pub allocated_replicas: i64,
            /// The number of GameServer replicas in the RESERVED state in this fleet.
            /// Reserved instances won't be deleted on scale down, but won't cause
            /// an autoscaler to scale up.
            #[prost(int64, tag = "3")]
            pub reserved_replicas: i64,
            /// The total number of current GameServer replicas in this fleet.
            #[prost(int64, tag = "4")]
            pub replicas: i64,
        }
    }
    /// Details about the Agones autoscaler.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeployedFleetAutoscaler {
        /// The name of the Agones autoscaler.
        #[prost(string, tag = "1")]
        pub autoscaler: std::string::String,
        /// The source spec that is used to create the autoscaler.
        /// The GameServerConfig resource may no longer exist in the system.
        #[prost(message, optional, tag = "4")]
        pub spec_source: ::std::option::Option<super::SpecSource>,
        /// The autoscaler spec retrieved from Agones.
        #[prost(string, tag = "3")]
        pub fleet_autoscaler_spec: std::string::String,
    }
}
/// Request message for GameServerClustersService.ListGameServerClusters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGameServerClustersRequest {
    /// Required. The parent resource name. Uses the form:
    /// "projects/{project}/locations/{location}/realms/{realm}".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The maximum number of items to return.  If unspecified, server
    /// will pick an appropriate default. Server may return fewer items than
    /// requested. A caller should only rely on response's
    /// [next_page_token][google.cloud.gaming.v1beta.ListGameServerClustersResponse.next_page_token] to
    /// determine if there are more GameServerClusters left to be queried.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// Optional. The filter to apply to list results.
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
    /// Optional. Specifies the ordering of results following syntax at
    /// https://cloud.google.com/apis/design/design_patterns#sorting_order.
    #[prost(string, tag = "5")]
    pub order_by: std::string::String,
}
/// Response message for GameServerClustersService.ListGameServerClusters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGameServerClustersResponse {
    /// The list of Game Server Clusters.
    #[prost(message, repeated, tag = "1")]
    pub game_server_clusters: ::std::vec::Vec<GameServerCluster>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// List of Locations that could not be reached.
    #[prost(string, repeated, tag = "4")]
    pub unreachable: ::std::vec::Vec<std::string::String>,
}
/// Request message for GameServerClustersService.GetGameServerCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGameServerClusterRequest {
    /// Required. The name of the Game Server Cluster to retrieve. Uses the form:
    ///
    /// `projects/{project}/locations/{location}/realms/{realm-id}/gameServerClusters/{cluster}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for GameServerClustersService.CreateGameServerCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGameServerClusterRequest {
    /// Required. The parent resource name. Uses the form:
    /// `projects/{project}/locations/{location}/realms/{realm-id}`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The ID of the Game Server Cluster resource to be created.
    #[prost(string, tag = "2")]
    pub game_server_cluster_id: std::string::String,
    /// Required. The Game Server Cluster resource to be created.
    #[prost(message, optional, tag = "3")]
    pub game_server_cluster: ::std::option::Option<GameServerCluster>,
}
/// Request message for GameServerClustersService.PreviewCreateGameServerCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreviewCreateGameServerClusterRequest {
    /// Required. The parent resource name. Uses the form:
    /// `projects/{project}/locations/{location}/realms/{realm}`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The ID of the Game Server Cluster resource to be created.
    #[prost(string, tag = "2")]
    pub game_server_cluster_id: std::string::String,
    /// Required. The Game Server Cluster resource to be created.
    #[prost(message, optional, tag = "3")]
    pub game_server_cluster: ::std::option::Option<GameServerCluster>,
    /// Optional. The target timestamp to compute the preview.
    #[prost(message, optional, tag = "4")]
    pub preview_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// Response message for
/// GameServerClustersService.PreviewCreateGameServerCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreviewCreateGameServerClusterResponse {
    /// The ETag of the game server cluster.
    #[prost(string, tag = "2")]
    pub etag: std::string::String,
    /// The target state.
    #[prost(message, optional, tag = "3")]
    pub target_state: ::std::option::Option<TargetState>,
}
/// Request message for GameServerClustersService.DeleteGameServerCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteGameServerClusterRequest {
    /// Required. The name of the Game Server Cluster to delete. Uses the form:
    /// `projects/{project}/locations/{location}/gameServerClusters/{cluster}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for GameServerClustersService.PreviewDeleteGameServerCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreviewDeleteGameServerClusterRequest {
    /// Required. The name of the Game Server Cluster to delete. Uses the form:
    /// `projects/{project}/locations/{location}/gameServerClusters/{cluster}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Optional. The target timestamp to compute the preview.
    #[prost(message, optional, tag = "2")]
    pub preview_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// Response message for
/// GameServerClustersService.PreviewDeleteGameServerCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreviewDeleteGameServerClusterResponse {
    /// The ETag of the game server cluster.
    #[prost(string, tag = "2")]
    pub etag: std::string::String,
    /// The target state.
    #[prost(message, optional, tag = "3")]
    pub target_state: ::std::option::Option<TargetState>,
}
/// Request message for GameServerClustersService.UpdateGameServerCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGameServerClusterRequest {
    /// Required. The Game Server Cluster to be updated.
    /// Only fields specified in update_mask are updated.
    #[prost(message, optional, tag = "1")]
    pub game_server_cluster: ::std::option::Option<GameServerCluster>,
    /// Required. Mask of fields to update. At least one path must be supplied in
    /// this field. For the `FieldMask` definition, see
    ///
    /// https:
    /// //developers.google.com/protocol-buffers
    /// // /docs/reference/google.protobuf#fieldmask
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for GameServerClustersService.UpdateGameServerCluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreviewUpdateGameServerClusterRequest {
    /// Required. The Game Server Cluster to be updated.
    /// Only fields specified in update_mask are updated.
    #[prost(message, optional, tag = "1")]
    pub game_server_cluster: ::std::option::Option<GameServerCluster>,
    /// Required. Mask of fields to update. At least one path must be supplied in
    /// this field. For the `FieldMask` definition, see
    ///
    /// https:
    /// //developers.google.com/protocol-buffers
    /// // /docs/reference/google.protobuf#fieldmask
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// Optional. The target timestamp to compute the preview.
    #[prost(message, optional, tag = "3")]
    pub preview_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// Response message for GameServerClustersService.PreviewUpdateGameServerCluster
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreviewUpdateGameServerClusterResponse {
    /// The ETag of the game server cluster.
    #[prost(string, tag = "2")]
    pub etag: std::string::String,
    /// The target state.
    #[prost(message, optional, tag = "3")]
    pub target_state: ::std::option::Option<TargetState>,
}
/// The Game Server Cluster connection information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameServerClusterConnectionInfo {
    /// Namespace designated on the Game Server Cluster where the Agones game
    /// server instances will be created. Existence of the namespace will be
    /// validated during creation.
    #[prost(string, tag = "5")]
    pub namespace: std::string::String,
    /// The location of the Kubernetes cluster.
    #[prost(
        oneof = "game_server_cluster_connection_info::ClusterReference",
        tags = "7"
    )]
    pub cluster_reference:
        ::std::option::Option<game_server_cluster_connection_info::ClusterReference>,
}
pub mod game_server_cluster_connection_info {
    /// The location of the Kubernetes cluster.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ClusterReference {
        /// Reference to the GKE cluster where the game servers are installed.
        #[prost(message, tag = "7")]
        GkeClusterReference(super::GkeClusterReference),
    }
}
/// A reference to a GKE cluster.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GkeClusterReference {
    /// The full or partial name of a GKE cluster, using one of the following
    /// forms:
    ///  * `projects/{project}/locations/{location}/clusters/{cluster}`
    ///  * `locations/{location}/clusters/{cluster}`
    ///  * `{cluster}`
    /// If project and location are not specified, the project and location of the
    /// GameServerCluster resource are used to generate the full name of the
    /// GKE cluster.
    #[prost(string, tag = "1")]
    pub cluster: std::string::String,
}
/// A Game Server Cluster resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameServerCluster {
    /// Required. The resource name of the Game Server Cluster. Uses the form:
    ///
    /// `projects/{project}/locations/{location}/realms/{realm}/gameServerClusters/{cluster}`.
    /// For example,
    ///
    /// `projects/my-project/locations/{location}/realms/zanzibar/gameServerClusters/my-onprem-cluster`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Output only. The creation time.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The last-modified time.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The labels associated with this Game Server Cluster. Each label is a
    /// key-value pair.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Game Server Cluster connection information. This information is used to
    /// manage Game Server Clusters.
    #[prost(message, optional, tag = "5")]
    pub connection_info: ::std::option::Option<GameServerClusterConnectionInfo>,
    /// ETag of the resource.
    #[prost(string, tag = "6")]
    pub etag: std::string::String,
    /// Human readable description of the cluster.
    #[prost(string, tag = "7")]
    pub description: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod game_server_clusters_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The game server cluster maps to Kubernetes clusters running Agones and is"]
    #[doc = " used to manage fleets within clusters."]
    pub struct GameServerClustersServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GameServerClustersServiceClient<tonic::transport::Channel> {
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
    impl<T> GameServerClustersServiceClient<T>
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
        #[doc = " Lists Game Server Clusters in a given project and location."]
        pub async fn list_game_server_clusters(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGameServerClustersRequest>,
        ) -> Result<tonic::Response<super::ListGameServerClustersResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gaming.v1beta.GameServerClustersService/ListGameServerClusters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of a single game server cluster."]
        pub async fn get_game_server_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGameServerClusterRequest>,
        ) -> Result<tonic::Response<super::GameServerCluster>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gaming.v1beta.GameServerClustersService/GetGameServerCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new game server cluster in a given project and location."]
        pub async fn create_game_server_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateGameServerClusterRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gaming.v1beta.GameServerClustersService/CreateGameServerCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Previews creation of a new game server cluster in a given project and"]
        #[doc = " location."]
        pub async fn preview_create_game_server_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::PreviewCreateGameServerClusterRequest>,
        ) -> Result<tonic::Response<super::PreviewCreateGameServerClusterResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.gaming.v1beta.GameServerClustersService/PreviewCreateGameServerCluster" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a single game server cluster."]
        pub async fn delete_game_server_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteGameServerClusterRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gaming.v1beta.GameServerClustersService/DeleteGameServerCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Previews deletion of a single game server cluster."]
        pub async fn preview_delete_game_server_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::PreviewDeleteGameServerClusterRequest>,
        ) -> Result<tonic::Response<super::PreviewDeleteGameServerClusterResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.gaming.v1beta.GameServerClustersService/PreviewDeleteGameServerCluster" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Patches a single game server cluster."]
        pub async fn update_game_server_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGameServerClusterRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gaming.v1beta.GameServerClustersService/UpdateGameServerCluster",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Previews updating a GameServerCluster."]
        pub async fn preview_update_game_server_cluster(
            &mut self,
            request: impl tonic::IntoRequest<super::PreviewUpdateGameServerClusterRequest>,
        ) -> Result<tonic::Response<super::PreviewUpdateGameServerClusterResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.gaming.v1beta.GameServerClustersService/PreviewUpdateGameServerCluster" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for GameServerClustersServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for GameServerClustersServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "GameServerClustersServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod game_server_clusters_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with GameServerClustersServiceServer."]
    #[async_trait]
    pub trait GameServerClustersService: Send + Sync + 'static {
        #[doc = " Lists Game Server Clusters in a given project and location."]
        async fn list_game_server_clusters(
            &self,
            request: tonic::Request<super::ListGameServerClustersRequest>,
        ) -> Result<tonic::Response<super::ListGameServerClustersResponse>, tonic::Status>;
        #[doc = " Gets details of a single game server cluster."]
        async fn get_game_server_cluster(
            &self,
            request: tonic::Request<super::GetGameServerClusterRequest>,
        ) -> Result<tonic::Response<super::GameServerCluster>, tonic::Status>;
        #[doc = " Creates a new game server cluster in a given project and location."]
        async fn create_game_server_cluster(
            &self,
            request: tonic::Request<super::CreateGameServerClusterRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Previews creation of a new game server cluster in a given project and"]
        #[doc = " location."]
        async fn preview_create_game_server_cluster(
            &self,
            request: tonic::Request<super::PreviewCreateGameServerClusterRequest>,
        ) -> Result<tonic::Response<super::PreviewCreateGameServerClusterResponse>, tonic::Status>;
        #[doc = " Deletes a single game server cluster."]
        async fn delete_game_server_cluster(
            &self,
            request: tonic::Request<super::DeleteGameServerClusterRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Previews deletion of a single game server cluster."]
        async fn preview_delete_game_server_cluster(
            &self,
            request: tonic::Request<super::PreviewDeleteGameServerClusterRequest>,
        ) -> Result<tonic::Response<super::PreviewDeleteGameServerClusterResponse>, tonic::Status>;
        #[doc = " Patches a single game server cluster."]
        async fn update_game_server_cluster(
            &self,
            request: tonic::Request<super::UpdateGameServerClusterRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Previews updating a GameServerCluster."]
        async fn preview_update_game_server_cluster(
            &self,
            request: tonic::Request<super::PreviewUpdateGameServerClusterRequest>,
        ) -> Result<tonic::Response<super::PreviewUpdateGameServerClusterResponse>, tonic::Status>;
    }
    #[doc = " The game server cluster maps to Kubernetes clusters running Agones and is"]
    #[doc = " used to manage fleets within clusters."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct GameServerClustersServiceServer<T: GameServerClustersService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: GameServerClustersService> GameServerClustersServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for GameServerClustersServiceServer<T>
    where
        T: GameServerClustersService,
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
            match req . uri ( ) . path ( ) { "/google.cloud.gaming.v1beta.GameServerClustersService/ListGameServerClusters" => { # [ allow ( non_camel_case_types ) ] struct ListGameServerClustersSvc < T : GameServerClustersService > ( pub Arc < T > ) ; impl < T : GameServerClustersService > tonic :: server :: UnaryService < super :: ListGameServerClustersRequest > for ListGameServerClustersSvc < T > { type Response = super :: ListGameServerClustersResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListGameServerClustersRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . list_game_server_clusters ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListGameServerClustersSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.gaming.v1beta.GameServerClustersService/GetGameServerCluster" => { # [ allow ( non_camel_case_types ) ] struct GetGameServerClusterSvc < T : GameServerClustersService > ( pub Arc < T > ) ; impl < T : GameServerClustersService > tonic :: server :: UnaryService < super :: GetGameServerClusterRequest > for GetGameServerClusterSvc < T > { type Response = super :: GameServerCluster ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetGameServerClusterRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_game_server_cluster ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetGameServerClusterSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.gaming.v1beta.GameServerClustersService/CreateGameServerCluster" => { # [ allow ( non_camel_case_types ) ] struct CreateGameServerClusterSvc < T : GameServerClustersService > ( pub Arc < T > ) ; impl < T : GameServerClustersService > tonic :: server :: UnaryService < super :: CreateGameServerClusterRequest > for CreateGameServerClusterSvc < T > { type Response = super :: super :: super :: super :: longrunning :: Operation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: CreateGameServerClusterRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . create_game_server_cluster ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = CreateGameServerClusterSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.gaming.v1beta.GameServerClustersService/PreviewCreateGameServerCluster" => { # [ allow ( non_camel_case_types ) ] struct PreviewCreateGameServerClusterSvc < T : GameServerClustersService > ( pub Arc < T > ) ; impl < T : GameServerClustersService > tonic :: server :: UnaryService < super :: PreviewCreateGameServerClusterRequest > for PreviewCreateGameServerClusterSvc < T > { type Response = super :: PreviewCreateGameServerClusterResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: PreviewCreateGameServerClusterRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . preview_create_game_server_cluster ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = PreviewCreateGameServerClusterSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.gaming.v1beta.GameServerClustersService/DeleteGameServerCluster" => { # [ allow ( non_camel_case_types ) ] struct DeleteGameServerClusterSvc < T : GameServerClustersService > ( pub Arc < T > ) ; impl < T : GameServerClustersService > tonic :: server :: UnaryService < super :: DeleteGameServerClusterRequest > for DeleteGameServerClusterSvc < T > { type Response = super :: super :: super :: super :: longrunning :: Operation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: DeleteGameServerClusterRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . delete_game_server_cluster ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = DeleteGameServerClusterSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.gaming.v1beta.GameServerClustersService/PreviewDeleteGameServerCluster" => { # [ allow ( non_camel_case_types ) ] struct PreviewDeleteGameServerClusterSvc < T : GameServerClustersService > ( pub Arc < T > ) ; impl < T : GameServerClustersService > tonic :: server :: UnaryService < super :: PreviewDeleteGameServerClusterRequest > for PreviewDeleteGameServerClusterSvc < T > { type Response = super :: PreviewDeleteGameServerClusterResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: PreviewDeleteGameServerClusterRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . preview_delete_game_server_cluster ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = PreviewDeleteGameServerClusterSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.gaming.v1beta.GameServerClustersService/UpdateGameServerCluster" => { # [ allow ( non_camel_case_types ) ] struct UpdateGameServerClusterSvc < T : GameServerClustersService > ( pub Arc < T > ) ; impl < T : GameServerClustersService > tonic :: server :: UnaryService < super :: UpdateGameServerClusterRequest > for UpdateGameServerClusterSvc < T > { type Response = super :: super :: super :: super :: longrunning :: Operation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: UpdateGameServerClusterRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . update_game_server_cluster ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = UpdateGameServerClusterSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.gaming.v1beta.GameServerClustersService/PreviewUpdateGameServerCluster" => { # [ allow ( non_camel_case_types ) ] struct PreviewUpdateGameServerClusterSvc < T : GameServerClustersService > ( pub Arc < T > ) ; impl < T : GameServerClustersService > tonic :: server :: UnaryService < super :: PreviewUpdateGameServerClusterRequest > for PreviewUpdateGameServerClusterSvc < T > { type Response = super :: PreviewUpdateGameServerClusterResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: PreviewUpdateGameServerClusterRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . preview_update_game_server_cluster ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = PreviewUpdateGameServerClusterSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } _ => Box :: pin ( async move { Ok ( http :: Response :: builder ( ) . status ( 200 ) . header ( "grpc-status" , "12" ) . body ( tonic :: body :: BoxBody :: empty ( ) ) . unwrap ( ) ) } ) , }
        }
    }
    impl<T: GameServerClustersService> Clone for GameServerClustersServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: GameServerClustersService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: GameServerClustersService> tonic::transport::NamedService
        for GameServerClustersServiceServer<T>
    {
        const NAME: &'static str = "google.cloud.gaming.v1beta.GameServerClustersService";
    }
}
/// Request message for GameServerConfigsService.ListGameServerConfigs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGameServerConfigsRequest {
    /// Required. The parent resource name. Uses the form:
    ///
    /// `projects/{project}/locations/{location}/gameServerDeployments/{deployment}/configs/*`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The maximum number of items to return.  If unspecified, server
    /// will pick an appropriate default. Server may return fewer items than
    /// requested. A caller should only rely on response's
    /// [next_page_token][google.cloud.gaming.v1beta.ListGameServerConfigsResponse.next_page_token] to
    /// determine if there are more GameServerConfigs left to be queried.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// Optional. The filter to apply to list results.
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
    /// Optional. Specifies the ordering of results following syntax at
    /// https://cloud.google.com/apis/design/design_patterns#sorting_order.
    #[prost(string, tag = "5")]
    pub order_by: std::string::String,
}
/// Response message for GameServerConfigsService.ListGameServerConfigs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGameServerConfigsResponse {
    /// The list of Game Server Configs.
    #[prost(message, repeated, tag = "1")]
    pub game_server_configs: ::std::vec::Vec<GameServerConfig>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// List of Locations that could not be reached.
    #[prost(string, repeated, tag = "4")]
    pub unreachable: ::std::vec::Vec<std::string::String>,
}
/// Request message for GameServerConfigsService.GetGameServerConfig.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGameServerConfigRequest {
    /// Required. The name of the Game Server Config to retrieve. Uses the form:
    ///
    /// `projects/{project}/locations/{location}/gameServerDeployments/{deployment}/configs/{config}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for GameServerConfigsService.CreateGameServerConfig.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGameServerConfigRequest {
    /// Required. The parent resource name. Uses the form:
    ///
    /// `projects/{project}/locations/{location}/gameServerDeployments/{deployment}/`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The ID of the Game Server Config resource to be created.
    #[prost(string, tag = "2")]
    pub config_id: std::string::String,
    /// Required. The Game Server Config resource to be created.
    #[prost(message, optional, tag = "3")]
    pub game_server_config: ::std::option::Option<GameServerConfig>,
}
/// Request message for GameServerConfigsService.DeleteGameServerConfig.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteGameServerConfigRequest {
    /// Required. The name of the Game Server Config to delete. Uses the form:
    ///
    /// `projects/{project}/locations/{location}/gameServerDeployments/{deployment}/configs/{config}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Autoscaling config for an Agones fleet.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScalingConfig {
    /// Required. The name of the Scaling Config
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. Agones fleet autoscaler spec. Example spec:
    /// https://agones.dev/site/docs/reference/fleetautoscaler/
    #[prost(string, tag = "2")]
    pub fleet_autoscaler_spec: std::string::String,
    /// Labels used to identify the Game Server Clusters to which this Agones
    /// scaling config applies. A Game Server Cluster is subject to this Agones
    /// scaling config if its labels match any of the selector entries.
    #[prost(message, repeated, tag = "4")]
    pub selectors: ::std::vec::Vec<LabelSelector>,
    /// The schedules to which this Scaling Config applies.
    #[prost(message, repeated, tag = "5")]
    pub schedules: ::std::vec::Vec<Schedule>,
}
/// Fleet configs for Agones.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FleetConfig {
    /// Agones fleet spec. Example spec:
    /// `https://agones.dev/site/docs/reference/fleet/`.
    #[prost(string, tag = "1")]
    pub fleet_spec: std::string::String,
    /// The name of the FleetConfig.
    #[prost(string, tag = "2")]
    pub name: std::string::String,
}
/// A Game Server Config resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameServerConfig {
    /// The resource name of the Game Server Config. Uses the form:
    ///
    /// `projects/{project}/locations/{location}/gameServerDeployments/{deployment}/configs/{config}`.
    /// For example,
    ///
    /// `projects/my-project/locations/global/gameServerDeployments/my-game/configs/my-config`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Output only. The creation time.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The last-modified time.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The labels associated with this Game Server Config. Each label is a
    /// key-value pair.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// FleetConfig contains a list of Agones fleet specs. Only one FleetConfig
    /// is allowed.
    #[prost(message, repeated, tag = "5")]
    pub fleet_configs: ::std::vec::Vec<FleetConfig>,
    /// The autoscaling settings.
    #[prost(message, repeated, tag = "6")]
    pub scaling_configs: ::std::vec::Vec<ScalingConfig>,
    /// The description of the Game Server Config.
    #[prost(string, tag = "7")]
    pub description: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod game_server_configs_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The Game Server Config configures the game servers in an Agones fleet."]
    pub struct GameServerConfigsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GameServerConfigsServiceClient<tonic::transport::Channel> {
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
    impl<T> GameServerConfigsServiceClient<T>
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
        #[doc = " Lists Game Server Configs in a given project, Location, and Game Server"]
        #[doc = " Deployment."]
        pub async fn list_game_server_configs(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGameServerConfigsRequest>,
        ) -> Result<tonic::Response<super::ListGameServerConfigsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gaming.v1beta.GameServerConfigsService/ListGameServerConfigs",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of a single Game Server Config."]
        pub async fn get_game_server_config(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGameServerConfigRequest>,
        ) -> Result<tonic::Response<super::GameServerConfig>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gaming.v1beta.GameServerConfigsService/GetGameServerConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new Game Server Config in a given project, Location, and Game"]
        #[doc = " Server Deployment. Game Server Configs are immutable, and are not applied"]
        #[doc = " until referenced in the Game Server Deployment Rollout resource."]
        pub async fn create_game_server_config(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateGameServerConfigRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gaming.v1beta.GameServerConfigsService/CreateGameServerConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a single Game Server Config. The deletion will fail if the Game"]
        #[doc = " Server Config is referenced in a Game Server Deployment Rollout."]
        pub async fn delete_game_server_config(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteGameServerConfigRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gaming.v1beta.GameServerConfigsService/DeleteGameServerConfig",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for GameServerConfigsServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for GameServerConfigsServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "GameServerConfigsServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod game_server_configs_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with GameServerConfigsServiceServer."]
    #[async_trait]
    pub trait GameServerConfigsService: Send + Sync + 'static {
        #[doc = " Lists Game Server Configs in a given project, Location, and Game Server"]
        #[doc = " Deployment."]
        async fn list_game_server_configs(
            &self,
            request: tonic::Request<super::ListGameServerConfigsRequest>,
        ) -> Result<tonic::Response<super::ListGameServerConfigsResponse>, tonic::Status>;
        #[doc = " Gets details of a single Game Server Config."]
        async fn get_game_server_config(
            &self,
            request: tonic::Request<super::GetGameServerConfigRequest>,
        ) -> Result<tonic::Response<super::GameServerConfig>, tonic::Status>;
        #[doc = " Creates a new Game Server Config in a given project, Location, and Game"]
        #[doc = " Server Deployment. Game Server Configs are immutable, and are not applied"]
        #[doc = " until referenced in the Game Server Deployment Rollout resource."]
        async fn create_game_server_config(
            &self,
            request: tonic::Request<super::CreateGameServerConfigRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Deletes a single Game Server Config. The deletion will fail if the Game"]
        #[doc = " Server Config is referenced in a Game Server Deployment Rollout."]
        async fn delete_game_server_config(
            &self,
            request: tonic::Request<super::DeleteGameServerConfigRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
    }
    #[doc = " The Game Server Config configures the game servers in an Agones fleet."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct GameServerConfigsServiceServer<T: GameServerConfigsService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: GameServerConfigsService> GameServerConfigsServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for GameServerConfigsServiceServer<T>
    where
        T: GameServerConfigsService,
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
                "/google.cloud.gaming.v1beta.GameServerConfigsService/ListGameServerConfigs" => {
                    #[allow(non_camel_case_types)]
                    struct ListGameServerConfigsSvc<T: GameServerConfigsService>(pub Arc<T>);
                    impl<T: GameServerConfigsService>
                        tonic::server::UnaryService<super::ListGameServerConfigsRequest>
                        for ListGameServerConfigsSvc<T>
                    {
                        type Response = super::ListGameServerConfigsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListGameServerConfigsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_game_server_configs(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListGameServerConfigsSvc(inner);
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
                "/google.cloud.gaming.v1beta.GameServerConfigsService/GetGameServerConfig" => {
                    #[allow(non_camel_case_types)]
                    struct GetGameServerConfigSvc<T: GameServerConfigsService>(pub Arc<T>);
                    impl<T: GameServerConfigsService>
                        tonic::server::UnaryService<super::GetGameServerConfigRequest>
                        for GetGameServerConfigSvc<T>
                    {
                        type Response = super::GameServerConfig;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetGameServerConfigRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_game_server_config(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetGameServerConfigSvc(inner);
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
                "/google.cloud.gaming.v1beta.GameServerConfigsService/CreateGameServerConfig" => {
                    #[allow(non_camel_case_types)]
                    struct CreateGameServerConfigSvc<T: GameServerConfigsService>(pub Arc<T>);
                    impl<T: GameServerConfigsService>
                        tonic::server::UnaryService<super::CreateGameServerConfigRequest>
                        for CreateGameServerConfigSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateGameServerConfigRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_game_server_config(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateGameServerConfigSvc(inner);
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
                "/google.cloud.gaming.v1beta.GameServerConfigsService/DeleteGameServerConfig" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteGameServerConfigSvc<T: GameServerConfigsService>(pub Arc<T>);
                    impl<T: GameServerConfigsService>
                        tonic::server::UnaryService<super::DeleteGameServerConfigRequest>
                        for DeleteGameServerConfigSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteGameServerConfigRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_game_server_config(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteGameServerConfigSvc(inner);
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
    impl<T: GameServerConfigsService> Clone for GameServerConfigsServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: GameServerConfigsService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: GameServerConfigsService> tonic::transport::NamedService
        for GameServerConfigsServiceServer<T>
    {
        const NAME: &'static str = "google.cloud.gaming.v1beta.GameServerConfigsService";
    }
}
/// Request message for GameServerDeploymentsService.ListGameServerDeployments.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGameServerDeploymentsRequest {
    /// Required. The parent resource name. Uses the form:
    /// `projects/{project}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The maximum number of items to return.  If unspecified, server
    /// will pick an appropriate default. Server may return fewer items than
    /// requested. A caller should only rely on response's
    /// [next_page_token][google.cloud.gaming.v1beta.ListGameServerDeploymentsResponse.next_page_token] to
    /// determine if there are more GameServerDeployments left to be queried.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous List request,
    /// if any.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// Optional. The filter to apply to list results.
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
    /// Optional. Specifies the ordering of results following syntax at
    /// https://cloud.google.com/apis/design/design_patterns#sorting_order.
    #[prost(string, tag = "5")]
    pub order_by: std::string::String,
}
/// Response message for GameServerDeploymentsService.ListGameServerDeployments.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListGameServerDeploymentsResponse {
    /// The list of Game Server Delpoyments.
    #[prost(message, repeated, tag = "1")]
    pub game_server_deployments: ::std::vec::Vec<GameServerDeployment>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// List of Locations that could not be reached.
    #[prost(string, repeated, tag = "4")]
    pub unreachable: ::std::vec::Vec<std::string::String>,
}
/// Request message for GameServerDeploymentsService.GetGameServerDeployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGameServerDeploymentRequest {
    /// Required. The name of the Game Server Deployment to retrieve. Uses the form:
    ///
    /// `projects/{project}/locations/{location}/gameServerDeployments/{deployment}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for
/// GameServerDeploymentsService.GetGameServerDeploymentRollout.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGameServerDeploymentRolloutRequest {
    /// Required. The name of the Game Server Deployment to retrieve. Uses the form:
    ///
    /// `projects/{project}/locations/{location}/gameServerDeployments/{deployment}/rollout`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for GameServerDeploymentsService.CreateGameServerDeployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateGameServerDeploymentRequest {
    /// Required. The parent resource name. Uses the form:
    /// `projects/{project}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The ID of the Game Server Deployment resource to be created.
    #[prost(string, tag = "2")]
    pub deployment_id: std::string::String,
    /// Required. The Game Server Deployment resource to be created.
    #[prost(message, optional, tag = "3")]
    pub game_server_deployment: ::std::option::Option<GameServerDeployment>,
}
/// Request message for GameServerDeploymentsService.DeleteGameServerDeployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteGameServerDeploymentRequest {
    /// Required. The name of the Game Server Deployment to delete. Uses the form:
    ///
    /// `projects/{project}/locations/{location}/gameServerDeployments/{deployment}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for GameServerDeploymentsService.UpdateGameServerDeployment.
/// Only allows updates for labels.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGameServerDeploymentRequest {
    /// Required. The Game Server Deployment to be updated.
    /// Only fields specified in update_mask are updated.
    #[prost(message, optional, tag = "1")]
    pub game_server_deployment: ::std::option::Option<GameServerDeployment>,
    /// Required. Mask of fields to update. At least one path must be supplied in
    /// this field. For the `FieldMask` definition, see
    ///
    /// https:
    /// //developers.google.com/protocol-buffers
    /// // /docs/reference/google.protobuf#fieldmask
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for
/// GameServerDeploymentsService.UpdateGameServerRolloutDeployment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateGameServerDeploymentRolloutRequest {
    /// Required. The Game Server Deployment Rollout to be updated.
    /// Only fields specified in update_mask are updated.
    #[prost(message, optional, tag = "1")]
    pub rollout: ::std::option::Option<GameServerDeploymentRollout>,
    /// Required. Mask of fields to update. At least one path must be supplied in
    /// this field. For the `FieldMask` definition, see
    ///
    /// https:
    /// //developers.google.com/protocol-buffers
    /// // /docs/reference/google.protobuf#fieldmask
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for GameServerDeploymentsService.FetchDeploymentState.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchDeploymentStateRequest {
    /// Required. The name of the Game Server Deployment. Uses the form:
    ///
    /// `projects/{project}/locations/{location}/gameServerDeployments/{deployment}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Response message for GameServerDeploymentsService.FetchDeploymentState.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchDeploymentStateResponse {
    /// The state of the Game Server Deployment in each Game Server Cluster.
    #[prost(message, repeated, tag = "1")]
    pub cluster_state: ::std::vec::Vec<fetch_deployment_state_response::DeployedClusterState>,
    /// List of Locations that could not be reached.
    #[prost(string, repeated, tag = "2")]
    pub unavailable: ::std::vec::Vec<std::string::String>,
}
pub mod fetch_deployment_state_response {
    /// The Game Server Cluster changes made by the Game Server Deployment.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DeployedClusterState {
        /// The name of the cluster.
        #[prost(string, tag = "1")]
        pub cluster: std::string::String,
        /// The details about the Agones fleets and autoscalers created in the
        /// Game Server Cluster.
        #[prost(message, repeated, tag = "2")]
        pub fleet_details: ::std::vec::Vec<super::DeployedFleetDetails>,
    }
}
/// A Game Server Deployment resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameServerDeployment {
    /// The resource name of the Game Server Deployment. Uses the form:
    ///
    /// `projects/{project}/locations/{location}/gameServerDeployments/{deployment}`.
    /// For example,
    ///
    /// `projects/my-project/locations/{location}/gameServerDeployments/my-deployment`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Output only. The creation time.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The last-modified time.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The labels associated with this Game Server Deployment. Each label is a
    /// key-value pair.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// ETag of the resource.
    #[prost(string, tag = "7")]
    pub etag: std::string::String,
    /// Human readable description of the Game Server Deployment.
    #[prost(string, tag = "8")]
    pub description: std::string::String,
}
/// A Game Server Config override.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameServerConfigOverride {
    /// Selector chooses the Game Server Config targets.
    #[prost(oneof = "game_server_config_override::Selector", tags = "1")]
    pub selector: ::std::option::Option<game_server_config_override::Selector>,
    /// Selects the Game Server Config and how it should be applied.
    #[prost(oneof = "game_server_config_override::Change", tags = "100")]
    pub change: ::std::option::Option<game_server_config_override::Change>,
}
pub mod game_server_config_override {
    /// Selector chooses the Game Server Config targets.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Selector {
        /// Selector for choosing applicable realms.
        #[prost(message, tag = "1")]
        RealmsSelector(super::RealmSelector),
    }
    /// Selects the Game Server Config and how it should be applied.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Change {
        /// The game server config for this override.
        #[prost(string, tag = "100")]
        ConfigVersion(std::string::String),
    }
}
/// The Game Server Deployment Rollout which represents the desired rollout
/// state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GameServerDeploymentRollout {
    /// The resource name of the Game Server Deployment Rollout. Uses the form:
    ///
    /// `projects/{project}/locations/{location}/gameServerDeployments/{deployment}/rollout`.
    /// For example,
    ///
    /// `projects/my-project/locations/{location}/gameServerDeployments/my-deployment/rollout`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Output only. The creation time.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The last-modified time.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The default Game Server Config is applied to all Realms unless overridden
    /// in the Rollout. For example,
    ///
    /// `projects/my-project/locations/global/gameServerDeployments/my-game/configs/my-config`.
    #[prost(string, tag = "4")]
    pub default_game_server_config: std::string::String,
    /// Contains the Game Server Config Rollout overrides. Overrides are processed
    /// in the order they are listed. Once a match is found for a Realm, the rest
    /// of the list is not processed.
    #[prost(message, repeated, tag = "5")]
    pub game_server_config_overrides: ::std::vec::Vec<GameServerConfigOverride>,
    /// ETag of the resource.
    #[prost(string, tag = "6")]
    pub etag: std::string::String,
}
/// Request message for PreviewGameServerDeploymentRollout.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreviewGameServerDeploymentRolloutRequest {
    /// Required. The Game Server Deployment Rollout to be updated.
    /// Only fields specified in update_mask are updated.
    #[prost(message, optional, tag = "1")]
    pub rollout: ::std::option::Option<GameServerDeploymentRollout>,
    /// Optional. Mask of fields to update. At least one path must be supplied in
    /// this field. For the `FieldMask` definition, see
    ///
    /// https:
    /// //developers.google.com/protocol-buffers
    /// // /docs/reference/google.protobuf#fieldmask
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// Optional. The target timestamp to compute the preview. Defaults to the immediately
    /// after the proposed Rollout completes.
    #[prost(message, optional, tag = "3")]
    pub preview_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// Response message for PreviewGameServerDeploymentRollout.
/// This has details about the Agones fleet and autoscaler to be actuated.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreviewGameServerDeploymentRolloutResponse {
    /// Locations that could not be reached on this request.
    #[prost(string, repeated, tag = "2")]
    pub unavailable: ::std::vec::Vec<std::string::String>,
    /// ETag of the Game Server Deployment.
    #[prost(string, tag = "3")]
    pub etag: std::string::String,
    /// The target state.
    #[prost(message, optional, tag = "4")]
    pub target_state: ::std::option::Option<TargetState>,
}
#[doc = r" Generated client implementations."]
pub mod game_server_deployments_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " The Game Server Deployment is used to control the deployment of Agones"]
    #[doc = " fleets."]
    pub struct GameServerDeploymentsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl GameServerDeploymentsServiceClient<tonic::transport::Channel> {
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
    impl<T> GameServerDeploymentsServiceClient<T>
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
        #[doc = " Lists Game Server Deployments in a given project and Location."]
        pub async fn list_game_server_deployments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListGameServerDeploymentsRequest>,
        ) -> Result<tonic::Response<super::ListGameServerDeploymentsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.gaming.v1beta.GameServerDeploymentsService/ListGameServerDeployments" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of a single Game Server Deployment."]
        pub async fn get_game_server_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGameServerDeploymentRequest>,
        ) -> Result<tonic::Response<super::GameServerDeployment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gaming.v1beta.GameServerDeploymentsService/GetGameServerDeployment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new Game Server Deployment in a given project and Location."]
        pub async fn create_game_server_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateGameServerDeploymentRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.gaming.v1beta.GameServerDeploymentsService/CreateGameServerDeployment" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a single Game Server Deployment."]
        pub async fn delete_game_server_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteGameServerDeploymentRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.gaming.v1beta.GameServerDeploymentsService/DeleteGameServerDeployment" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Patches a Game Server Deployment."]
        pub async fn update_game_server_deployment(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGameServerDeploymentRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.gaming.v1beta.GameServerDeploymentsService/UpdateGameServerDeployment" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details a single Game Server Deployment Rollout."]
        pub async fn get_game_server_deployment_rollout(
            &mut self,
            request: impl tonic::IntoRequest<super::GetGameServerDeploymentRolloutRequest>,
        ) -> Result<tonic::Response<super::GameServerDeploymentRollout>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.gaming.v1beta.GameServerDeploymentsService/GetGameServerDeploymentRollout" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Patches a single Game Server Deployment Rollout."]
        #[doc = " The method will not return an error if the update does not affect any"]
        #[doc = " existing realms. For example - if the default_game_server_config is changed"]
        #[doc = " but all existing realms use the override, that is valid. Similarly, if a"]
        #[doc = " non existing realm is explicitly called out in game_server_config_overrides"]
        #[doc = " field, that will also not result in an error."]
        pub async fn update_game_server_deployment_rollout(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateGameServerDeploymentRolloutRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.gaming.v1beta.GameServerDeploymentsService/UpdateGameServerDeploymentRollout" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Previews the Game Server Deployment Rollout. This API does not mutate the"]
        #[doc = " Rollout resource."]
        pub async fn preview_game_server_deployment_rollout(
            &mut self,
            request: impl tonic::IntoRequest<super::PreviewGameServerDeploymentRolloutRequest>,
        ) -> Result<tonic::Response<super::PreviewGameServerDeploymentRolloutResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.gaming.v1beta.GameServerDeploymentsService/PreviewGameServerDeploymentRollout" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves information about the current state of the Game Server"]
        #[doc = " Ddeployment. Gathers all the Agones fleets and Agones autoscalers,"]
        #[doc = " including fleets running an older version of the Game Server Deployment."]
        pub async fn fetch_deployment_state(
            &mut self,
            request: impl tonic::IntoRequest<super::FetchDeploymentStateRequest>,
        ) -> Result<tonic::Response<super::FetchDeploymentStateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gaming.v1beta.GameServerDeploymentsService/FetchDeploymentState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for GameServerDeploymentsServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for GameServerDeploymentsServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "GameServerDeploymentsServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod game_server_deployments_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with GameServerDeploymentsServiceServer."]
    #[async_trait]
    pub trait GameServerDeploymentsService: Send + Sync + 'static {
        #[doc = " Lists Game Server Deployments in a given project and Location."]
        async fn list_game_server_deployments(
            &self,
            request: tonic::Request<super::ListGameServerDeploymentsRequest>,
        ) -> Result<tonic::Response<super::ListGameServerDeploymentsResponse>, tonic::Status>;
        #[doc = " Gets details of a single Game Server Deployment."]
        async fn get_game_server_deployment(
            &self,
            request: tonic::Request<super::GetGameServerDeploymentRequest>,
        ) -> Result<tonic::Response<super::GameServerDeployment>, tonic::Status>;
        #[doc = " Creates a new Game Server Deployment in a given project and Location."]
        async fn create_game_server_deployment(
            &self,
            request: tonic::Request<super::CreateGameServerDeploymentRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Deletes a single Game Server Deployment."]
        async fn delete_game_server_deployment(
            &self,
            request: tonic::Request<super::DeleteGameServerDeploymentRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Patches a Game Server Deployment."]
        async fn update_game_server_deployment(
            &self,
            request: tonic::Request<super::UpdateGameServerDeploymentRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Gets details a single Game Server Deployment Rollout."]
        async fn get_game_server_deployment_rollout(
            &self,
            request: tonic::Request<super::GetGameServerDeploymentRolloutRequest>,
        ) -> Result<tonic::Response<super::GameServerDeploymentRollout>, tonic::Status>;
        #[doc = " Patches a single Game Server Deployment Rollout."]
        #[doc = " The method will not return an error if the update does not affect any"]
        #[doc = " existing realms. For example - if the default_game_server_config is changed"]
        #[doc = " but all existing realms use the override, that is valid. Similarly, if a"]
        #[doc = " non existing realm is explicitly called out in game_server_config_overrides"]
        #[doc = " field, that will also not result in an error."]
        async fn update_game_server_deployment_rollout(
            &self,
            request: tonic::Request<super::UpdateGameServerDeploymentRolloutRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Previews the Game Server Deployment Rollout. This API does not mutate the"]
        #[doc = " Rollout resource."]
        async fn preview_game_server_deployment_rollout(
            &self,
            request: tonic::Request<super::PreviewGameServerDeploymentRolloutRequest>,
        ) -> Result<tonic::Response<super::PreviewGameServerDeploymentRolloutResponse>, tonic::Status>;
        #[doc = " Retrieves information about the current state of the Game Server"]
        #[doc = " Ddeployment. Gathers all the Agones fleets and Agones autoscalers,"]
        #[doc = " including fleets running an older version of the Game Server Deployment."]
        async fn fetch_deployment_state(
            &self,
            request: tonic::Request<super::FetchDeploymentStateRequest>,
        ) -> Result<tonic::Response<super::FetchDeploymentStateResponse>, tonic::Status>;
    }
    #[doc = " The Game Server Deployment is used to control the deployment of Agones"]
    #[doc = " fleets."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct GameServerDeploymentsServiceServer<T: GameServerDeploymentsService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: GameServerDeploymentsService> GameServerDeploymentsServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for GameServerDeploymentsServiceServer<T>
    where
        T: GameServerDeploymentsService,
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
            match req . uri ( ) . path ( ) { "/google.cloud.gaming.v1beta.GameServerDeploymentsService/ListGameServerDeployments" => { # [ allow ( non_camel_case_types ) ] struct ListGameServerDeploymentsSvc < T : GameServerDeploymentsService > ( pub Arc < T > ) ; impl < T : GameServerDeploymentsService > tonic :: server :: UnaryService < super :: ListGameServerDeploymentsRequest > for ListGameServerDeploymentsSvc < T > { type Response = super :: ListGameServerDeploymentsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListGameServerDeploymentsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . list_game_server_deployments ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListGameServerDeploymentsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.gaming.v1beta.GameServerDeploymentsService/GetGameServerDeployment" => { # [ allow ( non_camel_case_types ) ] struct GetGameServerDeploymentSvc < T : GameServerDeploymentsService > ( pub Arc < T > ) ; impl < T : GameServerDeploymentsService > tonic :: server :: UnaryService < super :: GetGameServerDeploymentRequest > for GetGameServerDeploymentSvc < T > { type Response = super :: GameServerDeployment ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetGameServerDeploymentRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_game_server_deployment ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetGameServerDeploymentSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.gaming.v1beta.GameServerDeploymentsService/CreateGameServerDeployment" => { # [ allow ( non_camel_case_types ) ] struct CreateGameServerDeploymentSvc < T : GameServerDeploymentsService > ( pub Arc < T > ) ; impl < T : GameServerDeploymentsService > tonic :: server :: UnaryService < super :: CreateGameServerDeploymentRequest > for CreateGameServerDeploymentSvc < T > { type Response = super :: super :: super :: super :: longrunning :: Operation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: CreateGameServerDeploymentRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . create_game_server_deployment ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = CreateGameServerDeploymentSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.gaming.v1beta.GameServerDeploymentsService/DeleteGameServerDeployment" => { # [ allow ( non_camel_case_types ) ] struct DeleteGameServerDeploymentSvc < T : GameServerDeploymentsService > ( pub Arc < T > ) ; impl < T : GameServerDeploymentsService > tonic :: server :: UnaryService < super :: DeleteGameServerDeploymentRequest > for DeleteGameServerDeploymentSvc < T > { type Response = super :: super :: super :: super :: longrunning :: Operation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: DeleteGameServerDeploymentRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . delete_game_server_deployment ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = DeleteGameServerDeploymentSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.gaming.v1beta.GameServerDeploymentsService/UpdateGameServerDeployment" => { # [ allow ( non_camel_case_types ) ] struct UpdateGameServerDeploymentSvc < T : GameServerDeploymentsService > ( pub Arc < T > ) ; impl < T : GameServerDeploymentsService > tonic :: server :: UnaryService < super :: UpdateGameServerDeploymentRequest > for UpdateGameServerDeploymentSvc < T > { type Response = super :: super :: super :: super :: longrunning :: Operation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: UpdateGameServerDeploymentRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . update_game_server_deployment ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = UpdateGameServerDeploymentSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.gaming.v1beta.GameServerDeploymentsService/GetGameServerDeploymentRollout" => { # [ allow ( non_camel_case_types ) ] struct GetGameServerDeploymentRolloutSvc < T : GameServerDeploymentsService > ( pub Arc < T > ) ; impl < T : GameServerDeploymentsService > tonic :: server :: UnaryService < super :: GetGameServerDeploymentRolloutRequest > for GetGameServerDeploymentRolloutSvc < T > { type Response = super :: GameServerDeploymentRollout ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetGameServerDeploymentRolloutRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_game_server_deployment_rollout ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetGameServerDeploymentRolloutSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.gaming.v1beta.GameServerDeploymentsService/UpdateGameServerDeploymentRollout" => { # [ allow ( non_camel_case_types ) ] struct UpdateGameServerDeploymentRolloutSvc < T : GameServerDeploymentsService > ( pub Arc < T > ) ; impl < T : GameServerDeploymentsService > tonic :: server :: UnaryService < super :: UpdateGameServerDeploymentRolloutRequest > for UpdateGameServerDeploymentRolloutSvc < T > { type Response = super :: super :: super :: super :: longrunning :: Operation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: UpdateGameServerDeploymentRolloutRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . update_game_server_deployment_rollout ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = UpdateGameServerDeploymentRolloutSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.gaming.v1beta.GameServerDeploymentsService/PreviewGameServerDeploymentRollout" => { # [ allow ( non_camel_case_types ) ] struct PreviewGameServerDeploymentRolloutSvc < T : GameServerDeploymentsService > ( pub Arc < T > ) ; impl < T : GameServerDeploymentsService > tonic :: server :: UnaryService < super :: PreviewGameServerDeploymentRolloutRequest > for PreviewGameServerDeploymentRolloutSvc < T > { type Response = super :: PreviewGameServerDeploymentRolloutResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: PreviewGameServerDeploymentRolloutRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . preview_game_server_deployment_rollout ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = PreviewGameServerDeploymentRolloutSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.gaming.v1beta.GameServerDeploymentsService/FetchDeploymentState" => { # [ allow ( non_camel_case_types ) ] struct FetchDeploymentStateSvc < T : GameServerDeploymentsService > ( pub Arc < T > ) ; impl < T : GameServerDeploymentsService > tonic :: server :: UnaryService < super :: FetchDeploymentStateRequest > for FetchDeploymentStateSvc < T > { type Response = super :: FetchDeploymentStateResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: FetchDeploymentStateRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . fetch_deployment_state ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = FetchDeploymentStateSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } _ => Box :: pin ( async move { Ok ( http :: Response :: builder ( ) . status ( 200 ) . header ( "grpc-status" , "12" ) . body ( tonic :: body :: BoxBody :: empty ( ) ) . unwrap ( ) ) } ) , }
        }
    }
    impl<T: GameServerDeploymentsService> Clone for GameServerDeploymentsServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: GameServerDeploymentsService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: GameServerDeploymentsService> tonic::transport::NamedService
        for GameServerDeploymentsServiceServer<T>
    {
        const NAME: &'static str = "google.cloud.gaming.v1beta.GameServerDeploymentsService";
    }
}
/// Request message for RealmsService.ListRealms.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRealmsRequest {
    /// Required. The parent resource name. Uses the form:
    /// `projects/{project}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The maximum number of items to return.  If unspecified, server
    /// will pick an appropriate default. Server may return fewer items than
    /// requested. A caller should only rely on response's
    /// [next_page_token][google.cloud.gaming.v1beta.ListRealmsResponse.next_page_token] to
    /// determine if there are more Realms left to be queried.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous List request,
    /// if any.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// Optional. The filter to apply to list results.
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
    /// Optional. Specifies the ordering of results following syntax at
    /// https://cloud.google.com/apis/design/design_patterns#sorting_order.
    #[prost(string, tag = "5")]
    pub order_by: std::string::String,
}
/// Response message for RealmsService.ListRealms.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRealmsResponse {
    /// The list of Realms.
    #[prost(message, repeated, tag = "1")]
    pub realms: ::std::vec::Vec<Realm>,
    /// Token to retrieve the next page of results, or empty if there are no more
    /// results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
    /// List of Locations that could not be reached.
    #[prost(string, repeated, tag = "3")]
    pub unreachable: ::std::vec::Vec<std::string::String>,
}
/// Request message for RealmsService.GetRealm.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRealmRequest {
    /// Required. The name of the Realm to retrieve. Uses the form:
    /// `projects/{project}/locations/{location}/realms/{realm}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for RealmsService.CreateRealm.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateRealmRequest {
    /// Required. The parent resource name. Uses the form:
    /// `projects/{project}/locations/{location}`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The ID of the Realm resource to be created.
    #[prost(string, tag = "2")]
    pub realm_id: std::string::String,
    /// Required. The Realm resource to be created.
    #[prost(message, optional, tag = "3")]
    pub realm: ::std::option::Option<Realm>,
}
/// Request message for RealmsService.DeleteRealm.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRealmRequest {
    /// Required. The name of the Realm to delete. Uses the form:
    /// `projects/{project}/locations/{location}/realms/{realm}`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Request message for RealmsService.UpdateRealm.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateRealmRequest {
    /// Required. The Realm to be updated.
    /// Only fields specified in update_mask are updated.
    #[prost(message, optional, tag = "1")]
    pub realm: ::std::option::Option<Realm>,
    /// Required. The update mask applies to the resource. For the `FieldMask`
    /// definition, see
    ///
    /// https:
    /// //developers.google.com/protocol-buffers
    /// // /docs/reference/google.protobuf#fieldmask
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// Request message for RealmsService.PreviewRealmUpdate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreviewRealmUpdateRequest {
    /// Required. The Realm to be updated.
    /// Only fields specified in update_mask are updated.
    #[prost(message, optional, tag = "1")]
    pub realm: ::std::option::Option<Realm>,
    /// Required. The update mask applies to the resource. For the `FieldMask`
    /// definition, see
    ///
    /// https:
    /// //developers.google.com/protocol-buffers
    /// // /docs/reference/google.protobuf#fieldmask
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// Optional. The target timestamp to compute the preview.
    #[prost(message, optional, tag = "3")]
    pub preview_time: ::std::option::Option<::prost_types::Timestamp>,
}
/// Response message for RealmsService.PreviewRealmUpdate.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PreviewRealmUpdateResponse {
    /// ETag of the realm.
    #[prost(string, tag = "2")]
    pub etag: std::string::String,
    /// The target state.
    #[prost(message, optional, tag = "3")]
    pub target_state: ::std::option::Option<TargetState>,
}
/// A Realm resource.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Realm {
    /// The resource name of the Realm. Uses the form:
    /// `projects/{project}/locations/{location}/realms/{realm}`. For
    /// example, `projects/my-project/locations/{location}/realms/my-realm`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Output only. The creation time.
    #[prost(message, optional, tag = "2")]
    pub create_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. The last-modified time.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// The labels associated with this Realm. Each label is a key-value pair.
    #[prost(map = "string, string", tag = "4")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
    /// Required. Time zone where all policies targeting this Realm are evaluated. The value
    /// of this field must be from the IANA time zone database:
    /// https://www.iana.org/time-zones.
    #[prost(string, tag = "6")]
    pub time_zone: std::string::String,
    /// ETag of the resource.
    #[prost(string, tag = "7")]
    pub etag: std::string::String,
    /// Human readable description of the Realm.
    #[prost(string, tag = "8")]
    pub description: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod realms_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " A Realm is a grouping of Game Server Clusters that are considered"]
    #[doc = " interchangeable."]
    pub struct RealmsServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RealmsServiceClient<tonic::transport::Channel> {
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
    impl<T> RealmsServiceClient<T>
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
        #[doc = " Lists Realms in a given project and Location."]
        pub async fn list_realms(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRealmsRequest>,
        ) -> Result<tonic::Response<super::ListRealmsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gaming.v1beta.RealmsService/ListRealms",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets details of a single Realm."]
        pub async fn get_realm(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRealmRequest>,
        ) -> Result<tonic::Response<super::Realm>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gaming.v1beta.RealmsService/GetRealm",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a new Realm in a given project and Location."]
        pub async fn create_realm(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateRealmRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gaming.v1beta.RealmsService/CreateRealm",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a single Realm."]
        pub async fn delete_realm(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteRealmRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gaming.v1beta.RealmsService/DeleteRealm",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Patches a single Realm."]
        pub async fn update_realm(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateRealmRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gaming.v1beta.RealmsService/UpdateRealm",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Previews patches to a single Realm."]
        pub async fn preview_realm_update(
            &mut self,
            request: impl tonic::IntoRequest<super::PreviewRealmUpdateRequest>,
        ) -> Result<tonic::Response<super::PreviewRealmUpdateResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.gaming.v1beta.RealmsService/PreviewRealmUpdate",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for RealmsServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for RealmsServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "RealmsServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod realms_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with RealmsServiceServer."]
    #[async_trait]
    pub trait RealmsService: Send + Sync + 'static {
        #[doc = " Lists Realms in a given project and Location."]
        async fn list_realms(
            &self,
            request: tonic::Request<super::ListRealmsRequest>,
        ) -> Result<tonic::Response<super::ListRealmsResponse>, tonic::Status>;
        #[doc = " Gets details of a single Realm."]
        async fn get_realm(
            &self,
            request: tonic::Request<super::GetRealmRequest>,
        ) -> Result<tonic::Response<super::Realm>, tonic::Status>;
        #[doc = " Creates a new Realm in a given project and Location."]
        async fn create_realm(
            &self,
            request: tonic::Request<super::CreateRealmRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Deletes a single Realm."]
        async fn delete_realm(
            &self,
            request: tonic::Request<super::DeleteRealmRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Patches a single Realm."]
        async fn update_realm(
            &self,
            request: tonic::Request<super::UpdateRealmRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Previews patches to a single Realm."]
        async fn preview_realm_update(
            &self,
            request: tonic::Request<super::PreviewRealmUpdateRequest>,
        ) -> Result<tonic::Response<super::PreviewRealmUpdateResponse>, tonic::Status>;
    }
    #[doc = " A Realm is a grouping of Game Server Clusters that are considered"]
    #[doc = " interchangeable."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct RealmsServiceServer<T: RealmsService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: RealmsService> RealmsServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for RealmsServiceServer<T>
    where
        T: RealmsService,
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
                "/google.cloud.gaming.v1beta.RealmsService/ListRealms" => {
                    #[allow(non_camel_case_types)]
                    struct ListRealmsSvc<T: RealmsService>(pub Arc<T>);
                    impl<T: RealmsService> tonic::server::UnaryService<super::ListRealmsRequest> for ListRealmsSvc<T> {
                        type Response = super::ListRealmsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListRealmsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_realms(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListRealmsSvc(inner);
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
                "/google.cloud.gaming.v1beta.RealmsService/GetRealm" => {
                    #[allow(non_camel_case_types)]
                    struct GetRealmSvc<T: RealmsService>(pub Arc<T>);
                    impl<T: RealmsService> tonic::server::UnaryService<super::GetRealmRequest> for GetRealmSvc<T> {
                        type Response = super::Realm;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetRealmRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_realm(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetRealmSvc(inner);
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
                "/google.cloud.gaming.v1beta.RealmsService/CreateRealm" => {
                    #[allow(non_camel_case_types)]
                    struct CreateRealmSvc<T: RealmsService>(pub Arc<T>);
                    impl<T: RealmsService> tonic::server::UnaryService<super::CreateRealmRequest>
                        for CreateRealmSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateRealmRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_realm(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateRealmSvc(inner);
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
                "/google.cloud.gaming.v1beta.RealmsService/DeleteRealm" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteRealmSvc<T: RealmsService>(pub Arc<T>);
                    impl<T: RealmsService> tonic::server::UnaryService<super::DeleteRealmRequest>
                        for DeleteRealmSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteRealmRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_realm(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteRealmSvc(inner);
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
                "/google.cloud.gaming.v1beta.RealmsService/UpdateRealm" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateRealmSvc<T: RealmsService>(pub Arc<T>);
                    impl<T: RealmsService> tonic::server::UnaryService<super::UpdateRealmRequest>
                        for UpdateRealmSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateRealmRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_realm(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateRealmSvc(inner);
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
                "/google.cloud.gaming.v1beta.RealmsService/PreviewRealmUpdate" => {
                    #[allow(non_camel_case_types)]
                    struct PreviewRealmUpdateSvc<T: RealmsService>(pub Arc<T>);
                    impl<T: RealmsService>
                        tonic::server::UnaryService<super::PreviewRealmUpdateRequest>
                        for PreviewRealmUpdateSvc<T>
                    {
                        type Response = super::PreviewRealmUpdateResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::PreviewRealmUpdateRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.preview_realm_update(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = PreviewRealmUpdateSvc(inner);
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
    impl<T: RealmsService> Clone for RealmsServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: RealmsService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: RealmsService> tonic::transport::NamedService for RealmsServiceServer<T> {
        const NAME: &'static str = "google.cloud.gaming.v1beta.RealmsService";
    }
}
