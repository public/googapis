/// A reservation is a mechanism used to guarantee slots to users.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Reservation {
    /// The resource name of the reservation, e.g.,
    /// "projects/*/locations/*/reservations/team1-prod".
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Minimum slots available to this reservation. A slot is a unit of
    /// computational power in BigQuery, and serves as the unit of parallelism.
    /// Queries using this reservation might use more slots during runtime if
    /// ignore_idle_slots is set to false.
    /// If the new reservation's slot capacity exceed the parent's slot capacity or
    /// if total slot capacity of the new reservation and its siblings exceeds the
    /// parent's slot capacity, the request will fail with
    /// `google.rpc.Code.RESOURCE_EXHAUSTED`.
    #[prost(int64, tag = "2")]
    pub slot_capacity: i64,
    /// If false, any query using this reservation will use idle slots from other
    /// reservations within the same admin project. If true, a query using this
    /// reservation will execute with the slot capacity specified above at most.
    #[prost(bool, tag = "4")]
    pub ignore_idle_slots: bool,
}
/// Capacity commitment is a way to purchase compute capacity for BigQuery jobs
/// (in the form of slots) with some committed period of usage. Monthly and
/// annual commitments renew by default. Only flex commitments can be removed. In
/// order to remove monthly or annual commitments, their plan needs to be changed
/// to flex first.
///
/// A capacity commitment resource exists as a child resource of the admin
/// project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CapacityCommitment {
    /// Output only. The resource name of the capacity commitment, e.g.,
    ///    projects/myproject/locations/US/capacityCommitments/123
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Number of slots in this commitment.
    #[prost(int64, tag = "2")]
    pub slot_count: i64,
    /// Capacity commitment commitment plan.
    #[prost(enumeration = "capacity_commitment::CommitmentPlan", tag = "3")]
    pub plan: i32,
    /// Output only. State of the commitment.
    #[prost(enumeration = "capacity_commitment::State", tag = "4")]
    pub state: i32,
    /// Output only. The end of the current commitment period. It is applicable
    /// only for ACTIVE capacity commitments.
    #[prost(message, optional, tag = "5")]
    pub commitment_end_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Output only. For FAILED commitment plan, provides the reason of failure.
    #[prost(message, optional, tag = "7")]
    pub failure_status: ::std::option::Option<super::super::super::super::rpc::Status>,
    /// The plan this capacity commitment is converted to after commitment_end_time
    /// passes. Once the plan is changed, committed period is extended according to
    /// commitment plan. Only applicable for MONTHLY and ANNUAL commitments.
    #[prost(enumeration = "capacity_commitment::CommitmentPlan", tag = "8")]
    pub renewal_plan: i32,
}
pub mod capacity_commitment {
    /// Commitment plan defines the current committed period. Capacity commitment
    /// cannot be deleted during it's committed period.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum CommitmentPlan {
        /// Invalid plan value. Requests with this value will be rejected with
        /// error code `google.rpc.Code.INVALID_ARGUMENT`.
        Unspecified = 0,
        /// Flex commitments have committed period of 1 minute after becoming ACTIVE.
        /// After that, they are not in a committed period anymore and can be removed
        /// any time.
        Flex = 3,
        /// Monthly commitments have a committed period of 30 days after becoming
        /// ACTIVE.
        Monthly = 2,
        /// Annual commitments have a committed period of 365 days after becoming
        /// ACTIVE.
        Annual = 4,
    }
    /// Capacity commitment can either become ACTIVE right away or transition
    /// from PENDING to ACTIVE or FAILED.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Invalid state value.
        Unspecified = 0,
        /// Capacity commitment is pending provisioning. Pending capacity commitment
        /// does not contribute to the parent's slot_capacity.
        Pending = 1,
        /// Once slots are provisioned, capacity commitment becomes active.
        /// slot_count is added to the parent's slot_capacity.
        Active = 2,
        /// Capacity commitment is failed to be activated by the backend.
        Failed = 3,
    }
}
/// The request for
/// [ReservationService.CreateReservation][google.cloud.bigquery.reservation.v1beta1.ReservationService.CreateReservation].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateReservationRequest {
    /// Required. Project, location. E.g.,
    ///    projects/myproject/locations/US
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The reservation ID. This field must only contain lower case alphanumeric
    /// characters or dash. Max length is 64 characters.
    #[prost(string, tag = "2")]
    pub reservation_id: std::string::String,
    /// Content of the new reservation to create.
    #[prost(message, optional, tag = "3")]
    pub reservation: ::std::option::Option<Reservation>,
}
/// The request for
/// [ReservationService.ListReservations][google.cloud.bigquery.reservation.v1beta1.ReservationService.ListReservations].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReservationsRequest {
    /// Required. The parent resource name containing project and location, e.g.:
    ///   "projects/myproject/locations/US"
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
    /// Can be used to filter out reservations based on names, capacity, etc, e.g.:
    /// filter="reservation.slot_capacity > 200"
    /// filter="reservation.name = \"*dev/*\""
    /// Advanced filtering syntax can be
    /// [here](https://cloud.google.com/logging/docs/view/advanced-filters).
    #[prost(string, tag = "4")]
    pub filter: std::string::String,
}
/// The response for
/// [ReservationService.ListReservations][google.cloud.bigquery.reservation.v1beta1.ReservationService.ListReservations].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListReservationsResponse {
    /// List of reservations visible to the user.
    #[prost(message, repeated, tag = "1")]
    pub reservations: ::std::vec::Vec<Reservation>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The request for
/// [ReservationService.GetReservation][google.cloud.bigquery.reservation.v1beta1.ReservationService.GetReservation].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetReservationRequest {
    /// Required. Resource name of the reservation to retrieve. E.g.,
    ///    projects/myproject/locations/US/reservations/team1-prod
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The request for
/// [ReservationService.DeleteReservation][google.cloud.bigquery.reservation.v1beta1.ReservationService.DeleteReservation].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteReservationRequest {
    /// Required. Resource name of the reservation to retrieve. E.g.,
    ///    projects/myproject/locations/US/reservations/team1-prod
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The request for
/// [ReservationService.UpdateReservation][google.cloud.bigquery.reservation.v1beta1.ReservationService.UpdateReservation].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateReservationRequest {
    /// Content of the reservation to update.
    #[prost(message, optional, tag = "1")]
    pub reservation: ::std::option::Option<Reservation>,
    /// Standard field mask for the set of fields to be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// The request for
/// [ReservationService.ListCapacityCommitments][google.cloud.bigquery.reservation.v1beta1.ReservationService.ListCapacityCommitments].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCapacityCommitmentsRequest {
    /// Required. Resource name of the parent reservation. E.g.,
    ///    projects/myproject/locations/US
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// The response for
/// [ReservationService.ListCapacityCommitments][google.cloud.bigquery.reservation.v1beta1.ReservationService.ListCapacityCommitments].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCapacityCommitmentsResponse {
    /// List of capacity commitments visible to the user.
    #[prost(message, repeated, tag = "1")]
    pub capacity_commitments: ::std::vec::Vec<CapacityCommitment>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The request for
/// [ReservationService.GetCapacityCommitment][google.cloud.bigquery.reservation.v1beta1.ReservationService.GetCapacityCommitment].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCapacityCommitmentRequest {
    /// Required. Resource name of the capacity commitment to retrieve. E.g.,
    ///    projects/myproject/locations/US/capacityCommitments/123
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The request for
/// [ReservationService.DeleteCapacityCommitment][google.cloud.bigquery.reservation.v1beta1.ReservationService.DeleteCapacityCommitment].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCapacityCommitmentRequest {
    /// Required. Resource name of the capacity commitment to delete. E.g.,
    ///    projects/myproject/locations/US/capacityCommitments/123
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The request for
/// [ReservationService.UpdateCapacityCommitment][google.cloud.bigquery.reservation.v1beta1.ReservationService.UpdateCapacityCommitment].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCapacityCommitmentRequest {
    /// Content of the capacity commitment to update.
    #[prost(message, optional, tag = "1")]
    pub capacity_commitment: ::std::option::Option<CapacityCommitment>,
    /// Standard field mask for the set of fields to be updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// The request for
/// [ReservationService.SplitCapacityCommitment][google.cloud.bigquery.reservation.v1beta1.ReservationService.SplitCapacityCommitment].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitCapacityCommitmentRequest {
    /// Required. The resource name e.g.,:
    ///   projects/myproject/locations/US/capacityCommitments/123
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Number of slots in the capacity commitment after the split.
    #[prost(int64, tag = "2")]
    pub slot_count: i64,
}
/// The response for
/// [ReservationService.SplitCapacityCommitment][google.cloud.bigquery.reservation.v1beta1.ReservationService.SplitCapacityCommitment].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SplitCapacityCommitmentResponse {
    /// First capacity commitment, result of a split.
    #[prost(message, optional, tag = "1")]
    pub first: ::std::option::Option<CapacityCommitment>,
    /// Second capacity commitment, result of a split.
    #[prost(message, optional, tag = "2")]
    pub second: ::std::option::Option<CapacityCommitment>,
}
/// The request for
/// [ReservationService.MergeCapacityCommitments][google.cloud.bigquery.reservation.v1beta1.ReservationService.MergeCapacityCommitments].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MergeCapacityCommitmentsRequest {
    /// Parent resource that identifies admin project and location e.g.,
    /// projects/myproject/locations/us
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Ids of capacity commitments to merge.
    /// These capacity commitments must exist under admin project and location
    /// specified in the parent.
    #[prost(string, repeated, tag = "2")]
    pub capacity_commitment_ids: ::std::vec::Vec<std::string::String>,
}
/// A Assignment allows a project to submit jobs
/// of a certain type using slots from the specified reservation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Assignment {
    /// Output only. Name of the resource. E.g.:
    /// projects/myproject/locations/US/reservations/team1-prod/assignments/123.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The resource which will use the reservation. E.g.
    /// projects/myproject, folders/123, organizations/456.
    #[prost(string, tag = "4")]
    pub assignee: std::string::String,
    /// Which type of jobs will use the reservation.
    #[prost(enumeration = "assignment::JobType", tag = "3")]
    pub job_type: i32,
    /// Output only. State of the assignment.
    #[prost(enumeration = "assignment::State", tag = "6")]
    pub state: i32,
}
pub mod assignment {
    /// Types of job, which could be specified when using the reservation.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum JobType {
        /// Invalid type. Requests with this value will be rejected with
        /// error code `google.rpc.Code.INVALID_ARGUMENT`.
        Unspecified = 0,
        /// Pipeline (load/export) jobs from the project will use the reservation.
        Pipeline = 1,
        /// Query jobs from the project will use the reservation.
        Query = 2,
    }
    /// Assignment will remain in PENDING state if no active capacity commitment is
    /// present. It will become ACTIVE when some capacity commitment becomes
    /// active.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Invalid state value.
        Unspecified = 0,
        /// Queries from assignee will be executed as on-demand, if related
        /// assignment is pending.
        Pending = 1,
        /// Assignment is ready.
        Active = 2,
    }
}
/// The request for
/// [ReservationService.CreateAssignment][google.cloud.bigquery.reservation.v1beta1.ReservationService.CreateAssignment].
/// Note: "bigquery.reservationAssignments.create" permission is required on the
/// related assignee.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAssignmentRequest {
    /// Required. The parent resource name of the assignment
    /// E.g.: projects/myproject/locations/US/reservations/team1-prod
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Assignment resource to create.
    #[prost(message, optional, tag = "2")]
    pub assignment: ::std::option::Option<Assignment>,
}
/// The request for
/// [ReservationService.ListAssignments][google.cloud.bigquery.reservation.v1beta1.ReservationService.ListAssignments].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAssignmentsRequest {
    /// Required. The parent resource name e.g.:
    /// projects/myproject/locations/US/reservations/team1-prod
    /// Or:
    /// projects/myproject/locations/US/reservations/-
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// The response for
/// [ReservationService.ListAssignments][google.cloud.bigquery.reservation.v1beta1.ReservationService.ListAssignments].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListAssignmentsResponse {
    /// List of assignments visible to the user.
    #[prost(message, repeated, tag = "1")]
    pub assignments: ::std::vec::Vec<Assignment>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The request for
/// [ReservationService.DeleteAssignment][google.cloud.bigquery.reservation.v1beta1.ReservationService.DeleteAssignment].
/// Note: "bigquery.reservationAssignments.delete" permission is required on the
/// related assignee.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAssignmentRequest {
    /// Required. Name of the resource, e.g.:
    ///   projects/myproject/locations/US/reservations/team1-prod/assignments/123
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The request for
/// [ReservationService.SearchAssignments][google.cloud.bigquery.reservation.v1beta1.ReservationService.SearchAssignments].
/// Note: "bigquery.reservationAssignments.search" permission is required on the
/// related assignee.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAssignmentsRequest {
    /// Required. The resource name of the admin project(containing project and
    /// location), e.g.:
    ///   "projects/myproject/locations/US".
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Please specify resource name as assignee in the query.
    /// e.g., "assignee=projects/myproject"
    ///       "assignee=folders/123"
    ///       "assignee=organizations/456"
    #[prost(string, tag = "2")]
    pub query: std::string::String,
    /// The maximum number of items to return.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous List request, if any.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
}
/// The response for
/// [ReservationService.SearchAssignments][google.cloud.bigquery.reservation.v1beta1.ReservationService.SearchAssignments].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAssignmentsResponse {
    /// List of assignments visible to the user.
    #[prost(message, repeated, tag = "1")]
    pub assignments: ::std::vec::Vec<Assignment>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The request for
/// [ReservationService.MoveAssignment][google.cloud.bigquery.reservation.v1beta1.ReservationService.MoveAssignment].
/// Note: "bigquery.reservationAssignments.create" permission is required on the
/// destination_id. Note: "bigquery.reservationAssignments.create" and
/// "bigquery.reservationAssignments.delete" permission is required on the
/// related assignee.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveAssignmentRequest {
    /// Required. The resource name of the assignment,
    /// e.g.:
    ///   projects/myproject/locations/US/reservations/team1-prod/assignments/123
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The new reservation ID, e.g.:
    ///   projects/myotherproject/locations/US/reservations/team2-prod
    #[prost(string, tag = "3")]
    pub destination_id: std::string::String,
}
/// Represents a BI Reservation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BiReservation {
    /// The resource name of the singleton BI reservation.
    /// Reservation names have the form
    /// `projects/{project_id}/locations/{location_id}/bireservation`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Output only. The last update timestamp of a reservation.
    #[prost(message, optional, tag = "3")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
    /// Size of a reservation, in bytes.
    #[prost(int64, tag = "4")]
    pub size: i64,
}
/// A request to get a singleton BI reservation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBiReservationRequest {
    /// Required. Name of the requested reservation, for example:
    /// `projects/{project_id}/locations/{location_id}/bireservation`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// A request to update a BI reservation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateBiReservationRequest {
    /// A reservation to update.
    #[prost(message, optional, tag = "1")]
    pub bi_reservation: ::std::option::Option<BiReservation>,
    /// A list of fields to be updated in this request.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
#[doc = r" Generated client implementations."]
pub mod reservation_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " This API allows users to manage their flat-rate BigQuery reservations."]
    #[doc = ""]
    #[doc = " A reservation provides computational resource guarantees, in the form of"]
    #[doc = " [slots](https://cloud.google.com/bigquery/docs/slots), to users. A slot is a"]
    #[doc = " unit of computational power in BigQuery, and serves as the basic unit of"]
    #[doc = " parallelism. In a scan of a multi-partitioned table, a single slot operates"]
    #[doc = " on a single partition of the table. A reservation resource exists as a child"]
    #[doc = " resource of the admin project and location, e.g.:"]
    #[doc = "   projects/myproject/locations/US/reservations/reservationName."]
    #[doc = ""]
    #[doc = " A capacity commitment is a way to purchase compute capacity for BigQuery jobs"]
    #[doc = " (in the form of slots) with some committed period of usage. A capacity"]
    #[doc = " commitment resource exists as a child resource of the admin project and"]
    #[doc = " location, e.g.:"]
    #[doc = "   projects/myproject/locations/US/capacityCommitments/id."]
    pub struct ReservationServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ReservationServiceClient<tonic::transport::Channel> {
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
    impl<T> ReservationServiceClient<T>
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
        #[doc = " Creates a new reservation resource."]
        pub async fn create_reservation(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateReservationRequest>,
        ) -> Result<tonic::Response<super::Reservation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/CreateReservation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all the reservations for the project in the specified location."]
        pub async fn list_reservations(
            &mut self,
            request: impl tonic::IntoRequest<super::ListReservationsRequest>,
        ) -> Result<tonic::Response<super::ListReservationsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/ListReservations",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns information about the reservation."]
        pub async fn get_reservation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetReservationRequest>,
        ) -> Result<tonic::Response<super::Reservation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/GetReservation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a reservation."]
        #[doc = " Returns `google.rpc.Code.FAILED_PRECONDITION` when reservation has"]
        #[doc = " assignments."]
        pub async fn delete_reservation(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteReservationRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/DeleteReservation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an existing reservation resource."]
        pub async fn update_reservation(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateReservationRequest>,
        ) -> Result<tonic::Response<super::Reservation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/UpdateReservation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists all the capacity commitments for the admin project."]
        pub async fn list_capacity_commitments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCapacityCommitmentsRequest>,
        ) -> Result<tonic::Response<super::ListCapacityCommitmentsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.bigquery.reservation.v1beta1.ReservationService/ListCapacityCommitments" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns information about the capacity commitment."]
        pub async fn get_capacity_commitment(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCapacityCommitmentRequest>,
        ) -> Result<tonic::Response<super::CapacityCommitment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.bigquery.reservation.v1beta1.ReservationService/GetCapacityCommitment" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a capacity commitment. Attempting to delete capacity commitment"]
        #[doc = " before its commitment_end_time will fail with the error code"]
        #[doc = " `google.rpc.Code.FAILED_PRECONDITION`."]
        pub async fn delete_capacity_commitment(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCapacityCommitmentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.bigquery.reservation.v1beta1.ReservationService/DeleteCapacityCommitment" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates an existing capacity commitment."]
        #[doc = ""]
        #[doc = " Only renewal_plan field can be updated."]
        pub async fn update_capacity_commitment(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCapacityCommitmentRequest>,
        ) -> Result<tonic::Response<super::CapacityCommitment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.bigquery.reservation.v1beta1.ReservationService/UpdateCapacityCommitment" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Splits capacity commitment to two commitments of the same plan and"]
        #[doc = " commitment_end_time. A common use case to do that is to perform a downgrade"]
        #[doc = " e.g., in order to downgrade from 10000 slots to 8000, one might split 10000"]
        #[doc = " capacity commitment to 2000 and 8000, change the plan of the first one to"]
        #[doc = " flex and then delete it."]
        pub async fn split_capacity_commitment(
            &mut self,
            request: impl tonic::IntoRequest<super::SplitCapacityCommitmentRequest>,
        ) -> Result<tonic::Response<super::SplitCapacityCommitmentResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.bigquery.reservation.v1beta1.ReservationService/SplitCapacityCommitment" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Merges capacity commitments of the same plan into one. Resulting capacity"]
        #[doc = " commitment has the longer commitment_end_time out of the two. Attempting to"]
        #[doc = " merge capacity commitments of different plan will fail with the error code"]
        #[doc = " `google.rpc.Code.FAILED_PRECONDITION`."]
        pub async fn merge_capacity_commitments(
            &mut self,
            request: impl tonic::IntoRequest<super::MergeCapacityCommitmentsRequest>,
        ) -> Result<tonic::Response<super::CapacityCommitment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http :: uri :: PathAndQuery :: from_static ( "/google.cloud.bigquery.reservation.v1beta1.ReservationService/MergeCapacityCommitments" ) ;
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns `google.rpc.Code.PERMISSION_DENIED` if user does not have"]
        #[doc = " 'bigquery.admin' permissions on the project using the reservation"]
        #[doc = " and the project that owns this reservation."]
        #[doc = " Returns `google.rpc.Code.INVALID_ARGUMENT` when location of the assignment"]
        #[doc = " does not match location of the reservation."]
        pub async fn create_assignment(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateAssignmentRequest>,
        ) -> Result<tonic::Response<super::Assignment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/CreateAssignment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Lists assignments."]
        #[doc = " Only explicitly created assignments will be returned. E.g:"]
        #[doc = " organizationA contains project1 and project2. Reservation res1 exists."]
        #[doc = " CreateAssignment was invoked previously and following assignments were"]
        #[doc = " created explicitly:"]
        #[doc = "   <organizationA, res1>"]
        #[doc = "   <project1, res1>"]
        #[doc = " Then this API will just return the above two assignments for reservation"]
        #[doc = " res1, and no expansion/merge will happen. Wildcard \"-\" can be used for"]
        #[doc = " reservations in the request. In that case all assignments belongs to the"]
        #[doc = " specified project and location will be listed. Note"]
        #[doc = " \"-\" cannot be used for projects nor locations."]
        pub async fn list_assignments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListAssignmentsRequest>,
        ) -> Result<tonic::Response<super::ListAssignmentsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/ListAssignments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes a assignment. No expansion will happen."]
        #[doc = " E.g:"]
        #[doc = " organizationA contains project1 and project2. Reservation res1 exists."]
        #[doc = " CreateAssignment was invoked previously and following assignments were"]
        #[doc = " created explicitly:"]
        #[doc = "   <organizationA, res1>"]
        #[doc = "   <project1, res1>"]
        #[doc = " Then deletion of <organizationA, res1> won't affect <project1, res1>. After"]
        #[doc = " deletion of <organizationA, res1>, queries from project1 will still use"]
        #[doc = " res1, while queries from project2 will use on-demand mode."]
        pub async fn delete_assignment(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAssignmentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/DeleteAssignment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Looks up assignments for a specified resource for a particular region."]
        #[doc = " If the request is about a project:"]
        #[doc = "   1) Assignments created on the project will be returned if they exist."]
        #[doc = "   2) Otherwise assignments created on the closest ancestor will be"]
        #[doc = "   returned. 3) Assignments for different JobTypes will all be returned."]
        #[doc = " Same logic applies if the request is about a folder."]
        #[doc = " If the request is about an organization, then assignments created on the"]
        #[doc = " organization will be returned (organization doesn't have ancestors)."]
        #[doc = " Comparing to ListAssignments, there are some behavior"]
        #[doc = " differences:"]
        #[doc = "   1) permission on the assignee will be verified in this API."]
        #[doc = "   2) Hierarchy lookup (project->folder->organization) happens in this API."]
        #[doc = "   3) Parent here is projects/*/locations/*, instead of"]
        #[doc = "   projects/*/locations/*reservations/*."]
        #[doc = " Note \"-\" cannot be used for projects"]
        #[doc = " nor locations."]
        pub async fn search_assignments(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchAssignmentsRequest>,
        ) -> Result<tonic::Response<super::SearchAssignmentsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/SearchAssignments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Moves a assignment under a new reservation. Customers can do this by"]
        #[doc = " deleting the existing assignment followed by creating another assignment"]
        #[doc = " under the new reservation, but this method provides a transactional way to"]
        #[doc = " do so, to make sure the assignee always has an associated reservation."]
        #[doc = " Without the method customers might see some queries run on-demand which"]
        #[doc = " might be unexpected."]
        pub async fn move_assignment(
            &mut self,
            request: impl tonic::IntoRequest<super::MoveAssignmentRequest>,
        ) -> Result<tonic::Response<super::Assignment>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/MoveAssignment",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves a BI reservation."]
        pub async fn get_bi_reservation(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBiReservationRequest>,
        ) -> Result<tonic::Response<super::BiReservation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/GetBiReservation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates a BI reservation."]
        #[doc = " Only fields specified in the field_mask are updated."]
        #[doc = " Singleton BI reservation always exists with default size 0."]
        #[doc = " In order to reserve BI capacity it needs to be updated to an amount"]
        #[doc = " greater than 0. In order to release BI capacity reservation size"]
        #[doc = " must be set to 0."]
        pub async fn update_bi_reservation(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateBiReservationRequest>,
        ) -> Result<tonic::Response<super::BiReservation>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.bigquery.reservation.v1beta1.ReservationService/UpdateBiReservation",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ReservationServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ReservationServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ReservationServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod reservation_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ReservationServiceServer."]
    #[async_trait]
    pub trait ReservationService: Send + Sync + 'static {
        #[doc = " Creates a new reservation resource."]
        async fn create_reservation(
            &self,
            request: tonic::Request<super::CreateReservationRequest>,
        ) -> Result<tonic::Response<super::Reservation>, tonic::Status>;
        #[doc = " Lists all the reservations for the project in the specified location."]
        async fn list_reservations(
            &self,
            request: tonic::Request<super::ListReservationsRequest>,
        ) -> Result<tonic::Response<super::ListReservationsResponse>, tonic::Status>;
        #[doc = " Returns information about the reservation."]
        async fn get_reservation(
            &self,
            request: tonic::Request<super::GetReservationRequest>,
        ) -> Result<tonic::Response<super::Reservation>, tonic::Status>;
        #[doc = " Deletes a reservation."]
        #[doc = " Returns `google.rpc.Code.FAILED_PRECONDITION` when reservation has"]
        #[doc = " assignments."]
        async fn delete_reservation(
            &self,
            request: tonic::Request<super::DeleteReservationRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Updates an existing reservation resource."]
        async fn update_reservation(
            &self,
            request: tonic::Request<super::UpdateReservationRequest>,
        ) -> Result<tonic::Response<super::Reservation>, tonic::Status>;
        #[doc = " Lists all the capacity commitments for the admin project."]
        async fn list_capacity_commitments(
            &self,
            request: tonic::Request<super::ListCapacityCommitmentsRequest>,
        ) -> Result<tonic::Response<super::ListCapacityCommitmentsResponse>, tonic::Status>;
        #[doc = " Returns information about the capacity commitment."]
        async fn get_capacity_commitment(
            &self,
            request: tonic::Request<super::GetCapacityCommitmentRequest>,
        ) -> Result<tonic::Response<super::CapacityCommitment>, tonic::Status>;
        #[doc = " Deletes a capacity commitment. Attempting to delete capacity commitment"]
        #[doc = " before its commitment_end_time will fail with the error code"]
        #[doc = " `google.rpc.Code.FAILED_PRECONDITION`."]
        async fn delete_capacity_commitment(
            &self,
            request: tonic::Request<super::DeleteCapacityCommitmentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Updates an existing capacity commitment."]
        #[doc = ""]
        #[doc = " Only renewal_plan field can be updated."]
        async fn update_capacity_commitment(
            &self,
            request: tonic::Request<super::UpdateCapacityCommitmentRequest>,
        ) -> Result<tonic::Response<super::CapacityCommitment>, tonic::Status>;
        #[doc = " Splits capacity commitment to two commitments of the same plan and"]
        #[doc = " commitment_end_time. A common use case to do that is to perform a downgrade"]
        #[doc = " e.g., in order to downgrade from 10000 slots to 8000, one might split 10000"]
        #[doc = " capacity commitment to 2000 and 8000, change the plan of the first one to"]
        #[doc = " flex and then delete it."]
        async fn split_capacity_commitment(
            &self,
            request: tonic::Request<super::SplitCapacityCommitmentRequest>,
        ) -> Result<tonic::Response<super::SplitCapacityCommitmentResponse>, tonic::Status>;
        #[doc = " Merges capacity commitments of the same plan into one. Resulting capacity"]
        #[doc = " commitment has the longer commitment_end_time out of the two. Attempting to"]
        #[doc = " merge capacity commitments of different plan will fail with the error code"]
        #[doc = " `google.rpc.Code.FAILED_PRECONDITION`."]
        async fn merge_capacity_commitments(
            &self,
            request: tonic::Request<super::MergeCapacityCommitmentsRequest>,
        ) -> Result<tonic::Response<super::CapacityCommitment>, tonic::Status>;
        #[doc = " Returns `google.rpc.Code.PERMISSION_DENIED` if user does not have"]
        #[doc = " 'bigquery.admin' permissions on the project using the reservation"]
        #[doc = " and the project that owns this reservation."]
        #[doc = " Returns `google.rpc.Code.INVALID_ARGUMENT` when location of the assignment"]
        #[doc = " does not match location of the reservation."]
        async fn create_assignment(
            &self,
            request: tonic::Request<super::CreateAssignmentRequest>,
        ) -> Result<tonic::Response<super::Assignment>, tonic::Status>;
        #[doc = " Lists assignments."]
        #[doc = " Only explicitly created assignments will be returned. E.g:"]
        #[doc = " organizationA contains project1 and project2. Reservation res1 exists."]
        #[doc = " CreateAssignment was invoked previously and following assignments were"]
        #[doc = " created explicitly:"]
        #[doc = "   <organizationA, res1>"]
        #[doc = "   <project1, res1>"]
        #[doc = " Then this API will just return the above two assignments for reservation"]
        #[doc = " res1, and no expansion/merge will happen. Wildcard \"-\" can be used for"]
        #[doc = " reservations in the request. In that case all assignments belongs to the"]
        #[doc = " specified project and location will be listed. Note"]
        #[doc = " \"-\" cannot be used for projects nor locations."]
        async fn list_assignments(
            &self,
            request: tonic::Request<super::ListAssignmentsRequest>,
        ) -> Result<tonic::Response<super::ListAssignmentsResponse>, tonic::Status>;
        #[doc = " Deletes a assignment. No expansion will happen."]
        #[doc = " E.g:"]
        #[doc = " organizationA contains project1 and project2. Reservation res1 exists."]
        #[doc = " CreateAssignment was invoked previously and following assignments were"]
        #[doc = " created explicitly:"]
        #[doc = "   <organizationA, res1>"]
        #[doc = "   <project1, res1>"]
        #[doc = " Then deletion of <organizationA, res1> won't affect <project1, res1>. After"]
        #[doc = " deletion of <organizationA, res1>, queries from project1 will still use"]
        #[doc = " res1, while queries from project2 will use on-demand mode."]
        async fn delete_assignment(
            &self,
            request: tonic::Request<super::DeleteAssignmentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Looks up assignments for a specified resource for a particular region."]
        #[doc = " If the request is about a project:"]
        #[doc = "   1) Assignments created on the project will be returned if they exist."]
        #[doc = "   2) Otherwise assignments created on the closest ancestor will be"]
        #[doc = "   returned. 3) Assignments for different JobTypes will all be returned."]
        #[doc = " Same logic applies if the request is about a folder."]
        #[doc = " If the request is about an organization, then assignments created on the"]
        #[doc = " organization will be returned (organization doesn't have ancestors)."]
        #[doc = " Comparing to ListAssignments, there are some behavior"]
        #[doc = " differences:"]
        #[doc = "   1) permission on the assignee will be verified in this API."]
        #[doc = "   2) Hierarchy lookup (project->folder->organization) happens in this API."]
        #[doc = "   3) Parent here is projects/*/locations/*, instead of"]
        #[doc = "   projects/*/locations/*reservations/*."]
        #[doc = " Note \"-\" cannot be used for projects"]
        #[doc = " nor locations."]
        async fn search_assignments(
            &self,
            request: tonic::Request<super::SearchAssignmentsRequest>,
        ) -> Result<tonic::Response<super::SearchAssignmentsResponse>, tonic::Status>;
        #[doc = " Moves a assignment under a new reservation. Customers can do this by"]
        #[doc = " deleting the existing assignment followed by creating another assignment"]
        #[doc = " under the new reservation, but this method provides a transactional way to"]
        #[doc = " do so, to make sure the assignee always has an associated reservation."]
        #[doc = " Without the method customers might see some queries run on-demand which"]
        #[doc = " might be unexpected."]
        async fn move_assignment(
            &self,
            request: tonic::Request<super::MoveAssignmentRequest>,
        ) -> Result<tonic::Response<super::Assignment>, tonic::Status>;
        #[doc = " Retrieves a BI reservation."]
        async fn get_bi_reservation(
            &self,
            request: tonic::Request<super::GetBiReservationRequest>,
        ) -> Result<tonic::Response<super::BiReservation>, tonic::Status>;
        #[doc = " Updates a BI reservation."]
        #[doc = " Only fields specified in the field_mask are updated."]
        #[doc = " Singleton BI reservation always exists with default size 0."]
        #[doc = " In order to reserve BI capacity it needs to be updated to an amount"]
        #[doc = " greater than 0. In order to release BI capacity reservation size"]
        #[doc = " must be set to 0."]
        async fn update_bi_reservation(
            &self,
            request: tonic::Request<super::UpdateBiReservationRequest>,
        ) -> Result<tonic::Response<super::BiReservation>, tonic::Status>;
    }
    #[doc = " This API allows users to manage their flat-rate BigQuery reservations."]
    #[doc = ""]
    #[doc = " A reservation provides computational resource guarantees, in the form of"]
    #[doc = " [slots](https://cloud.google.com/bigquery/docs/slots), to users. A slot is a"]
    #[doc = " unit of computational power in BigQuery, and serves as the basic unit of"]
    #[doc = " parallelism. In a scan of a multi-partitioned table, a single slot operates"]
    #[doc = " on a single partition of the table. A reservation resource exists as a child"]
    #[doc = " resource of the admin project and location, e.g.:"]
    #[doc = "   projects/myproject/locations/US/reservations/reservationName."]
    #[doc = ""]
    #[doc = " A capacity commitment is a way to purchase compute capacity for BigQuery jobs"]
    #[doc = " (in the form of slots) with some committed period of usage. A capacity"]
    #[doc = " commitment resource exists as a child resource of the admin project and"]
    #[doc = " location, e.g.:"]
    #[doc = "   projects/myproject/locations/US/capacityCommitments/id."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct ReservationServiceServer<T: ReservationService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ReservationService> ReservationServiceServer<T> {
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
    impl<T, B> Service<http::Request<B>> for ReservationServiceServer<T>
    where
        T: ReservationService,
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
            match req . uri ( ) . path ( ) { "/google.cloud.bigquery.reservation.v1beta1.ReservationService/CreateReservation" => { # [ allow ( non_camel_case_types ) ] struct CreateReservationSvc < T : ReservationService > ( pub Arc < T > ) ; impl < T : ReservationService > tonic :: server :: UnaryService < super :: CreateReservationRequest > for CreateReservationSvc < T > { type Response = super :: Reservation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: CreateReservationRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . create_reservation ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = CreateReservationSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.reservation.v1beta1.ReservationService/ListReservations" => { # [ allow ( non_camel_case_types ) ] struct ListReservationsSvc < T : ReservationService > ( pub Arc < T > ) ; impl < T : ReservationService > tonic :: server :: UnaryService < super :: ListReservationsRequest > for ListReservationsSvc < T > { type Response = super :: ListReservationsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListReservationsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . list_reservations ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListReservationsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.reservation.v1beta1.ReservationService/GetReservation" => { # [ allow ( non_camel_case_types ) ] struct GetReservationSvc < T : ReservationService > ( pub Arc < T > ) ; impl < T : ReservationService > tonic :: server :: UnaryService < super :: GetReservationRequest > for GetReservationSvc < T > { type Response = super :: Reservation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetReservationRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_reservation ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetReservationSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.reservation.v1beta1.ReservationService/DeleteReservation" => { # [ allow ( non_camel_case_types ) ] struct DeleteReservationSvc < T : ReservationService > ( pub Arc < T > ) ; impl < T : ReservationService > tonic :: server :: UnaryService < super :: DeleteReservationRequest > for DeleteReservationSvc < T > { type Response = ( ) ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: DeleteReservationRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . delete_reservation ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = DeleteReservationSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.reservation.v1beta1.ReservationService/UpdateReservation" => { # [ allow ( non_camel_case_types ) ] struct UpdateReservationSvc < T : ReservationService > ( pub Arc < T > ) ; impl < T : ReservationService > tonic :: server :: UnaryService < super :: UpdateReservationRequest > for UpdateReservationSvc < T > { type Response = super :: Reservation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: UpdateReservationRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . update_reservation ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = UpdateReservationSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.reservation.v1beta1.ReservationService/ListCapacityCommitments" => { # [ allow ( non_camel_case_types ) ] struct ListCapacityCommitmentsSvc < T : ReservationService > ( pub Arc < T > ) ; impl < T : ReservationService > tonic :: server :: UnaryService < super :: ListCapacityCommitmentsRequest > for ListCapacityCommitmentsSvc < T > { type Response = super :: ListCapacityCommitmentsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListCapacityCommitmentsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . list_capacity_commitments ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListCapacityCommitmentsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.reservation.v1beta1.ReservationService/GetCapacityCommitment" => { # [ allow ( non_camel_case_types ) ] struct GetCapacityCommitmentSvc < T : ReservationService > ( pub Arc < T > ) ; impl < T : ReservationService > tonic :: server :: UnaryService < super :: GetCapacityCommitmentRequest > for GetCapacityCommitmentSvc < T > { type Response = super :: CapacityCommitment ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetCapacityCommitmentRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_capacity_commitment ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetCapacityCommitmentSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.reservation.v1beta1.ReservationService/DeleteCapacityCommitment" => { # [ allow ( non_camel_case_types ) ] struct DeleteCapacityCommitmentSvc < T : ReservationService > ( pub Arc < T > ) ; impl < T : ReservationService > tonic :: server :: UnaryService < super :: DeleteCapacityCommitmentRequest > for DeleteCapacityCommitmentSvc < T > { type Response = ( ) ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: DeleteCapacityCommitmentRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . delete_capacity_commitment ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = DeleteCapacityCommitmentSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.reservation.v1beta1.ReservationService/UpdateCapacityCommitment" => { # [ allow ( non_camel_case_types ) ] struct UpdateCapacityCommitmentSvc < T : ReservationService > ( pub Arc < T > ) ; impl < T : ReservationService > tonic :: server :: UnaryService < super :: UpdateCapacityCommitmentRequest > for UpdateCapacityCommitmentSvc < T > { type Response = super :: CapacityCommitment ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: UpdateCapacityCommitmentRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . update_capacity_commitment ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = UpdateCapacityCommitmentSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.reservation.v1beta1.ReservationService/SplitCapacityCommitment" => { # [ allow ( non_camel_case_types ) ] struct SplitCapacityCommitmentSvc < T : ReservationService > ( pub Arc < T > ) ; impl < T : ReservationService > tonic :: server :: UnaryService < super :: SplitCapacityCommitmentRequest > for SplitCapacityCommitmentSvc < T > { type Response = super :: SplitCapacityCommitmentResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: SplitCapacityCommitmentRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . split_capacity_commitment ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = SplitCapacityCommitmentSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.reservation.v1beta1.ReservationService/MergeCapacityCommitments" => { # [ allow ( non_camel_case_types ) ] struct MergeCapacityCommitmentsSvc < T : ReservationService > ( pub Arc < T > ) ; impl < T : ReservationService > tonic :: server :: UnaryService < super :: MergeCapacityCommitmentsRequest > for MergeCapacityCommitmentsSvc < T > { type Response = super :: CapacityCommitment ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: MergeCapacityCommitmentsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . merge_capacity_commitments ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = MergeCapacityCommitmentsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.reservation.v1beta1.ReservationService/CreateAssignment" => { # [ allow ( non_camel_case_types ) ] struct CreateAssignmentSvc < T : ReservationService > ( pub Arc < T > ) ; impl < T : ReservationService > tonic :: server :: UnaryService < super :: CreateAssignmentRequest > for CreateAssignmentSvc < T > { type Response = super :: Assignment ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: CreateAssignmentRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . create_assignment ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = CreateAssignmentSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.reservation.v1beta1.ReservationService/ListAssignments" => { # [ allow ( non_camel_case_types ) ] struct ListAssignmentsSvc < T : ReservationService > ( pub Arc < T > ) ; impl < T : ReservationService > tonic :: server :: UnaryService < super :: ListAssignmentsRequest > for ListAssignmentsSvc < T > { type Response = super :: ListAssignmentsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: ListAssignmentsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . list_assignments ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = ListAssignmentsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.reservation.v1beta1.ReservationService/DeleteAssignment" => { # [ allow ( non_camel_case_types ) ] struct DeleteAssignmentSvc < T : ReservationService > ( pub Arc < T > ) ; impl < T : ReservationService > tonic :: server :: UnaryService < super :: DeleteAssignmentRequest > for DeleteAssignmentSvc < T > { type Response = ( ) ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: DeleteAssignmentRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . delete_assignment ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = DeleteAssignmentSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.reservation.v1beta1.ReservationService/SearchAssignments" => { # [ allow ( non_camel_case_types ) ] struct SearchAssignmentsSvc < T : ReservationService > ( pub Arc < T > ) ; impl < T : ReservationService > tonic :: server :: UnaryService < super :: SearchAssignmentsRequest > for SearchAssignmentsSvc < T > { type Response = super :: SearchAssignmentsResponse ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: SearchAssignmentsRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . search_assignments ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = SearchAssignmentsSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.reservation.v1beta1.ReservationService/MoveAssignment" => { # [ allow ( non_camel_case_types ) ] struct MoveAssignmentSvc < T : ReservationService > ( pub Arc < T > ) ; impl < T : ReservationService > tonic :: server :: UnaryService < super :: MoveAssignmentRequest > for MoveAssignmentSvc < T > { type Response = super :: Assignment ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: MoveAssignmentRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . move_assignment ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = MoveAssignmentSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.reservation.v1beta1.ReservationService/GetBiReservation" => { # [ allow ( non_camel_case_types ) ] struct GetBiReservationSvc < T : ReservationService > ( pub Arc < T > ) ; impl < T : ReservationService > tonic :: server :: UnaryService < super :: GetBiReservationRequest > for GetBiReservationSvc < T > { type Response = super :: BiReservation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: GetBiReservationRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . get_bi_reservation ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = GetBiReservationSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } "/google.cloud.bigquery.reservation.v1beta1.ReservationService/UpdateBiReservation" => { # [ allow ( non_camel_case_types ) ] struct UpdateBiReservationSvc < T : ReservationService > ( pub Arc < T > ) ; impl < T : ReservationService > tonic :: server :: UnaryService < super :: UpdateBiReservationRequest > for UpdateBiReservationSvc < T > { type Response = super :: BiReservation ; type Future = BoxFuture < tonic :: Response < Self :: Response > , tonic :: Status > ; fn call ( & mut self , request : tonic :: Request < super :: UpdateBiReservationRequest > ) -> Self :: Future { let inner = self . 0 . clone ( ) ; let fut = async move { inner . update_bi_reservation ( request ) . await } ; Box :: pin ( fut ) } } let inner = self . inner . clone ( ) ; let fut = async move { let interceptor = inner . 1 . clone ( ) ; let inner = inner . 0 ; let method = UpdateBiReservationSvc ( inner ) ; let codec = tonic :: codec :: ProstCodec :: default ( ) ; let mut grpc = if let Some ( interceptor ) = interceptor { tonic :: server :: Grpc :: with_interceptor ( codec , interceptor ) } else { tonic :: server :: Grpc :: new ( codec ) } ; let res = grpc . unary ( method , req ) . await ; Ok ( res ) } ; Box :: pin ( fut ) } _ => Box :: pin ( async move { Ok ( http :: Response :: builder ( ) . status ( 200 ) . header ( "grpc-status" , "12" ) . body ( tonic :: body :: BoxBody :: empty ( ) ) . unwrap ( ) ) } ) , }
        }
    }
    impl<T: ReservationService> Clone for ReservationServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ReservationService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ReservationService> tonic::transport::NamedService for ReservationServiceServer<T> {
        const NAME: &'static str = "google.cloud.bigquery.reservation.v1beta1.ReservationService";
    }
}
