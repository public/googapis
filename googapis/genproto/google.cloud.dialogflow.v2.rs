/// Represents a single validation error.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidationError {
    /// The severity of the error.
    #[prost(enumeration = "validation_error::Severity", tag = "1")]
    pub severity: i32,
    /// The names of the entries that the error is associated with.
    /// Format:
    ///
    /// - "projects/<Project ID>/agent", if the error is associated with the entire
    /// agent.
    /// - "projects/<Project ID>/agent/intents/<Intent ID>", if the error is
    /// associated with certain intents.
    /// - "projects/<Project
    /// ID>/agent/intents/<Intent Id>/trainingPhrases/<Training Phrase ID>", if the
    /// error is associated with certain intent training phrases.
    /// - "projects/<Project ID>/agent/intents/<Intent Id>/parameters/<Parameter
    /// ID>", if the error is associated with certain intent parameters.
    /// - "projects/<Project ID>/agent/entities/<Entity ID>", if the error is
    /// associated with certain entities.
    #[prost(string, repeated, tag = "3")]
    pub entries: ::std::vec::Vec<std::string::String>,
    /// The detailed error messsage.
    #[prost(string, tag = "4")]
    pub error_message: std::string::String,
}
pub mod validation_error {
    /// Represents a level of severity.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Severity {
        /// Not specified. This value should never be used.
        Unspecified = 0,
        /// The agent doesn't follow Dialogflow best practicies.
        Info = 1,
        /// The agent may not behave as expected.
        Warning = 2,
        /// The agent may experience partial failures.
        Error = 3,
        /// The agent may completely fail.
        Critical = 4,
    }
}
/// Represents the output of agent validation.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidationResult {
    /// Contains all validation errors.
    #[prost(message, repeated, tag = "1")]
    pub validation_errors: ::std::vec::Vec<ValidationError>,
}
/// Represents a conversational agent.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Agent {
    /// Required. The project of this agent.
    /// Format: `projects/<Project ID>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The name of this agent.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// Required. The default language of the agent as a language tag. See
    /// [Language
    /// Support](https://cloud.google.com/dialogflow/docs/reference/language)
    /// for a list of the currently supported language codes. This field cannot be
    /// set by the `Update` method.
    #[prost(string, tag = "3")]
    pub default_language_code: std::string::String,
    /// Optional. The list of all languages supported by this agent (except for the
    /// `default_language_code`).
    #[prost(string, repeated, tag = "4")]
    pub supported_language_codes: ::std::vec::Vec<std::string::String>,
    /// Required. The time zone of this agent from the
    /// [time zone database](https://www.iana.org/time-zones), e.g.,
    /// America/New_York, Europe/Paris.
    #[prost(string, tag = "5")]
    pub time_zone: std::string::String,
    /// Optional. The description of this agent.
    /// The maximum length is 500 characters. If exceeded, the request is rejected.
    #[prost(string, tag = "6")]
    pub description: std::string::String,
    /// Optional. The URI of the agent's avatar.
    /// Avatars are used throughout the Dialogflow console and in the self-hosted
    /// [Web
    /// Demo](https://cloud.google.com/dialogflow/docs/integrations/web-demo)
    /// integration.
    #[prost(string, tag = "7")]
    pub avatar_uri: std::string::String,
    /// Optional. Determines whether this agent should log conversation queries.
    #[prost(bool, tag = "8")]
    pub enable_logging: bool,
    /// Optional. Determines how intents are detected from user queries.
    #[prost(enumeration = "agent::MatchMode", tag = "9")]
    pub match_mode: i32,
    /// Optional. To filter out false positive results and still get variety in
    /// matched natural language inputs for your agent, you can tune the machine
    /// learning classification threshold. If the returned score value is less than
    /// the threshold value, then a fallback intent will be triggered or, if there
    /// are no fallback intents defined, no intent will be triggered. The score
    /// values range from 0.0 (completely uncertain) to 1.0 (completely certain).
    /// If set to 0.0, the default of 0.3 is used.
    #[prost(float, tag = "10")]
    pub classification_threshold: f32,
    /// Optional. API version displayed in Dialogflow console. If not specified,
    /// V2 API is assumed. Clients are free to query different service endpoints
    /// for different API versions. However, bots connectors and webhook calls will
    /// follow the specified API version.
    #[prost(enumeration = "agent::ApiVersion", tag = "14")]
    pub api_version: i32,
    /// Optional. The agent tier. If not specified, TIER_STANDARD is assumed.
    #[prost(enumeration = "agent::Tier", tag = "15")]
    pub tier: i32,
}
pub mod agent {
    /// Match mode determines how intents are detected from user queries.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MatchMode {
        /// Not specified.
        Unspecified = 0,
        /// Best for agents with a small number of examples in intents and/or wide
        /// use of templates syntax and composite entities.
        Hybrid = 1,
        /// Can be used for agents with a large number of examples in intents,
        /// especially the ones using @sys.any or very large custom entities.
        MlOnly = 2,
    }
    /// API version for the agent.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ApiVersion {
        /// Not specified.
        Unspecified = 0,
        /// Legacy V1 API.
        V1 = 1,
        /// V2 API.
        V2 = 2,
        /// V2beta1 API.
        V2Beta1 = 3,
    }
    /// Represents the agent tier.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Tier {
        /// Not specified. This value should never be used.
        Unspecified = 0,
        /// Standard tier.
        Standard = 1,
        /// Enterprise tier (Essentials).
        Enterprise = 2,
        /// Enterprise tier (Plus).
        EnterprisePlus = 3,
    }
}
/// The request message for [Agents.GetAgent][google.cloud.dialogflow.v2.Agents.GetAgent].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetAgentRequest {
    /// Required. The project that the agent to fetch is associated with.
    /// Format: `projects/<Project ID>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
}
/// The request message for [Agents.SetAgent][google.cloud.dialogflow.v2.Agents.SetAgent].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetAgentRequest {
    /// Required. The agent to update.
    #[prost(message, optional, tag = "1")]
    pub agent: ::std::option::Option<Agent>,
    /// Optional. The mask to control which fields get updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// The request message for [Agents.DeleteAgent][google.cloud.dialogflow.v2.Agents.DeleteAgent].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAgentRequest {
    /// Required. The project that the agent to delete is associated with.
    /// Format: `projects/<Project ID>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
}
/// The request message for [Agents.SearchAgents][google.cloud.dialogflow.v2.Agents.SearchAgents].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAgentsRequest {
    /// Required. The project to list agents from.
    /// Format: `projects/<Project ID or '-'>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The maximum number of items to return in a single page. By
    /// default 100 and at most 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// The response message for [Agents.SearchAgents][google.cloud.dialogflow.v2.Agents.SearchAgents].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchAgentsResponse {
    /// The list of agents. There will be a maximum number of items returned based
    /// on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub agents: ::std::vec::Vec<Agent>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The request message for [Agents.TrainAgent][google.cloud.dialogflow.v2.Agents.TrainAgent].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrainAgentRequest {
    /// Required. The project that the agent to train is associated with.
    /// Format: `projects/<Project ID>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
}
/// The request message for [Agents.ExportAgent][google.cloud.dialogflow.v2.Agents.ExportAgent].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportAgentRequest {
    /// Required. The project that the agent to export is associated with.
    /// Format: `projects/<Project ID>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The [Google Cloud Storage](https://cloud.google.com/storage/docs/)
    /// URI to export the agent to.
    /// The format of this URI must be `gs://<bucket-name>/<object-name>`.
    /// If left unspecified, the serialized agent is returned inline.
    #[prost(string, tag = "2")]
    pub agent_uri: std::string::String,
}
/// The response message for [Agents.ExportAgent][google.cloud.dialogflow.v2.Agents.ExportAgent].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportAgentResponse {
    /// The exported agent.
    #[prost(oneof = "export_agent_response::Agent", tags = "1, 2")]
    pub agent: ::std::option::Option<export_agent_response::Agent>,
}
pub mod export_agent_response {
    /// The exported agent.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Agent {
        /// The URI to a file containing the exported agent. This field is populated
        /// only if `agent_uri` is specified in `ExportAgentRequest`.
        #[prost(string, tag = "1")]
        AgentUri(std::string::String),
        /// Zip compressed raw byte content for agent.
        #[prost(bytes, tag = "2")]
        AgentContent(std::vec::Vec<u8>),
    }
}
/// The request message for [Agents.ImportAgent][google.cloud.dialogflow.v2.Agents.ImportAgent].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportAgentRequest {
    /// Required. The project that the agent to import is associated with.
    /// Format: `projects/<Project ID>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The agent to import.
    #[prost(oneof = "import_agent_request::Agent", tags = "2, 3")]
    pub agent: ::std::option::Option<import_agent_request::Agent>,
}
pub mod import_agent_request {
    /// Required. The agent to import.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Agent {
        /// The URI to a Google Cloud Storage file containing the agent to import.
        /// Note: The URI must start with "gs://".
        #[prost(string, tag = "2")]
        AgentUri(std::string::String),
        /// Zip compressed raw byte content for agent.
        #[prost(bytes, tag = "3")]
        AgentContent(std::vec::Vec<u8>),
    }
}
/// The request message for [Agents.RestoreAgent][google.cloud.dialogflow.v2.Agents.RestoreAgent].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestoreAgentRequest {
    /// Required. The project that the agent to restore is associated with.
    /// Format: `projects/<Project ID>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The agent to restore.
    #[prost(oneof = "restore_agent_request::Agent", tags = "2, 3")]
    pub agent: ::std::option::Option<restore_agent_request::Agent>,
}
pub mod restore_agent_request {
    /// Required. The agent to restore.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Agent {
        /// The URI to a Google Cloud Storage file containing the agent to restore.
        /// Note: The URI must start with "gs://".
        #[prost(string, tag = "2")]
        AgentUri(std::string::String),
        /// Zip compressed raw byte content for agent.
        #[prost(bytes, tag = "3")]
        AgentContent(std::vec::Vec<u8>),
    }
}
/// The request message for [Agents.GetValidationResult][google.cloud.dialogflow.v2.Agents.GetValidationResult].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValidationResultRequest {
    /// Required. The project that the agent is associated with.
    /// Format: `projects/<Project ID>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The language for which you want a validation result. If not
    /// specified, the agent's default language is used. [Many
    /// languages](https://cloud.google.com/dialogflow/docs/reference/language)
    /// are supported. Note: languages must be enabled in the agent before they can
    /// be used.
    #[prost(string, tag = "3")]
    pub language_code: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod agents_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Agents are best described as Natural Language Understanding (NLU) modules"]
    #[doc = " that transform user requests into actionable data. You can include agents"]
    #[doc = " in your app, product, or service to determine user intent and respond to the"]
    #[doc = " user in a natural way."]
    #[doc = ""]
    #[doc = " After you create an agent, you can add [Intents][google.cloud.dialogflow.v2.Intents], [Contexts][google.cloud.dialogflow.v2.Contexts],"]
    #[doc = " [Entity Types][google.cloud.dialogflow.v2.EntityTypes], [Webhooks][google.cloud.dialogflow.v2.WebhookRequest], and so on to"]
    #[doc = " manage the flow of a conversation and match user input to predefined intents"]
    #[doc = " and actions."]
    #[doc = ""]
    #[doc = " You can create an agent using both Dialogflow Standard Edition and"]
    #[doc = " Dialogflow Enterprise Edition. For details, see"]
    #[doc = " [Dialogflow"]
    #[doc = " Editions](https://cloud.google.com/dialogflow/docs/editions)."]
    #[doc = ""]
    #[doc = " You can save your agent for backup or versioning by exporting the agent by"]
    #[doc = " using the [ExportAgent][google.cloud.dialogflow.v2.Agents.ExportAgent] method. You can import a saved"]
    #[doc = " agent by using the [ImportAgent][google.cloud.dialogflow.v2.Agents.ImportAgent] method."]
    #[doc = ""]
    #[doc = " Dialogflow provides several"]
    #[doc = " [prebuilt"]
    #[doc = " agents](https://cloud.google.com/dialogflow/docs/agents-prebuilt)"]
    #[doc = " for common conversation scenarios such as determining a date and time,"]
    #[doc = " converting currency, and so on."]
    #[doc = ""]
    #[doc = " For more information about agents, see the"]
    #[doc = " [Dialogflow"]
    #[doc = " documentation](https://cloud.google.com/dialogflow/docs/agents-overview)."]
    pub struct AgentsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AgentsClient<tonic::transport::Channel> {
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
    impl<T> AgentsClient<T>
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
        #[doc = " Retrieves the specified agent."]
        pub async fn get_agent(
            &mut self,
            request: impl tonic::IntoRequest<super::GetAgentRequest>,
        ) -> Result<tonic::Response<super::Agent>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.cloud.dialogflow.v2.Agents/GetAgent");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates/updates the specified agent."]
        pub async fn set_agent(
            &mut self,
            request: impl tonic::IntoRequest<super::SetAgentRequest>,
        ) -> Result<tonic::Response<super::Agent>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/google.cloud.dialogflow.v2.Agents/SetAgent");
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified agent."]
        pub async fn delete_agent(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAgentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.v2.Agents/DeleteAgent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Returns the list of agents."]
        #[doc = ""]
        #[doc = " Since there is at most one conversational agent per project, this method is"]
        #[doc = " useful primarily for listing all agents across projects the caller has"]
        #[doc = " access to. One can achieve that with a wildcard project collection id \"-\"."]
        #[doc = " Refer to [List"]
        #[doc = " Sub-Collections](https://cloud.google.com/apis/design/design_patterns#list_sub-collections)."]
        pub async fn search_agents(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchAgentsRequest>,
        ) -> Result<tonic::Response<super::SearchAgentsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.v2.Agents/SearchAgents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Trains the specified agent."]
        #[doc = ""]
        #[doc = " Operation <response: [google.protobuf.Empty][google.protobuf.Empty]>"]
        pub async fn train_agent(
            &mut self,
            request: impl tonic::IntoRequest<super::TrainAgentRequest>,
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
                "/google.cloud.dialogflow.v2.Agents/TrainAgent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Exports the specified agent to a ZIP file."]
        #[doc = ""]
        #[doc = " Operation <response: [ExportAgentResponse][google.cloud.dialogflow.v2.ExportAgentResponse]>"]
        pub async fn export_agent(
            &mut self,
            request: impl tonic::IntoRequest<super::ExportAgentRequest>,
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
                "/google.cloud.dialogflow.v2.Agents/ExportAgent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Imports the specified agent from a ZIP file."]
        #[doc = ""]
        #[doc = " Uploads new intents and entity types without deleting the existing ones."]
        #[doc = " Intents and entity types with the same name are replaced with the new"]
        #[doc = " versions from ImportAgentRequest."]
        #[doc = ""]
        #[doc = " Operation <response: [google.protobuf.Empty][google.protobuf.Empty]>"]
        pub async fn import_agent(
            &mut self,
            request: impl tonic::IntoRequest<super::ImportAgentRequest>,
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
                "/google.cloud.dialogflow.v2.Agents/ImportAgent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Restores the specified agent from a ZIP file."]
        #[doc = ""]
        #[doc = " Replaces the current agent version with a new one. All the intents and"]
        #[doc = " entity types in the older version are deleted."]
        #[doc = ""]
        #[doc = " Operation <response: [google.protobuf.Empty][google.protobuf.Empty]>"]
        pub async fn restore_agent(
            &mut self,
            request: impl tonic::IntoRequest<super::RestoreAgentRequest>,
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
                "/google.cloud.dialogflow.v2.Agents/RestoreAgent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Gets agent validation result. Agent validation is performed during"]
        #[doc = " training time and is updated automatically when training is completed."]
        pub async fn get_validation_result(
            &mut self,
            request: impl tonic::IntoRequest<super::GetValidationResultRequest>,
        ) -> Result<tonic::Response<super::ValidationResult>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.v2.Agents/GetValidationResult",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for AgentsClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for AgentsClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "AgentsClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod agents_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with AgentsServer."]
    #[async_trait]
    pub trait Agents: Send + Sync + 'static {
        #[doc = " Retrieves the specified agent."]
        async fn get_agent(
            &self,
            request: tonic::Request<super::GetAgentRequest>,
        ) -> Result<tonic::Response<super::Agent>, tonic::Status>;
        #[doc = " Creates/updates the specified agent."]
        async fn set_agent(
            &self,
            request: tonic::Request<super::SetAgentRequest>,
        ) -> Result<tonic::Response<super::Agent>, tonic::Status>;
        #[doc = " Deletes the specified agent."]
        async fn delete_agent(
            &self,
            request: tonic::Request<super::DeleteAgentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Returns the list of agents."]
        #[doc = ""]
        #[doc = " Since there is at most one conversational agent per project, this method is"]
        #[doc = " useful primarily for listing all agents across projects the caller has"]
        #[doc = " access to. One can achieve that with a wildcard project collection id \"-\"."]
        #[doc = " Refer to [List"]
        #[doc = " Sub-Collections](https://cloud.google.com/apis/design/design_patterns#list_sub-collections)."]
        async fn search_agents(
            &self,
            request: tonic::Request<super::SearchAgentsRequest>,
        ) -> Result<tonic::Response<super::SearchAgentsResponse>, tonic::Status>;
        #[doc = " Trains the specified agent."]
        #[doc = ""]
        #[doc = " Operation <response: [google.protobuf.Empty][google.protobuf.Empty]>"]
        async fn train_agent(
            &self,
            request: tonic::Request<super::TrainAgentRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Exports the specified agent to a ZIP file."]
        #[doc = ""]
        #[doc = " Operation <response: [ExportAgentResponse][google.cloud.dialogflow.v2.ExportAgentResponse]>"]
        async fn export_agent(
            &self,
            request: tonic::Request<super::ExportAgentRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Imports the specified agent from a ZIP file."]
        #[doc = ""]
        #[doc = " Uploads new intents and entity types without deleting the existing ones."]
        #[doc = " Intents and entity types with the same name are replaced with the new"]
        #[doc = " versions from ImportAgentRequest."]
        #[doc = ""]
        #[doc = " Operation <response: [google.protobuf.Empty][google.protobuf.Empty]>"]
        async fn import_agent(
            &self,
            request: tonic::Request<super::ImportAgentRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Restores the specified agent from a ZIP file."]
        #[doc = ""]
        #[doc = " Replaces the current agent version with a new one. All the intents and"]
        #[doc = " entity types in the older version are deleted."]
        #[doc = ""]
        #[doc = " Operation <response: [google.protobuf.Empty][google.protobuf.Empty]>"]
        async fn restore_agent(
            &self,
            request: tonic::Request<super::RestoreAgentRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Gets agent validation result. Agent validation is performed during"]
        #[doc = " training time and is updated automatically when training is completed."]
        async fn get_validation_result(
            &self,
            request: tonic::Request<super::GetValidationResultRequest>,
        ) -> Result<tonic::Response<super::ValidationResult>, tonic::Status>;
    }
    #[doc = " Agents are best described as Natural Language Understanding (NLU) modules"]
    #[doc = " that transform user requests into actionable data. You can include agents"]
    #[doc = " in your app, product, or service to determine user intent and respond to the"]
    #[doc = " user in a natural way."]
    #[doc = ""]
    #[doc = " After you create an agent, you can add [Intents][google.cloud.dialogflow.v2.Intents], [Contexts][google.cloud.dialogflow.v2.Contexts],"]
    #[doc = " [Entity Types][google.cloud.dialogflow.v2.EntityTypes], [Webhooks][google.cloud.dialogflow.v2.WebhookRequest], and so on to"]
    #[doc = " manage the flow of a conversation and match user input to predefined intents"]
    #[doc = " and actions."]
    #[doc = ""]
    #[doc = " You can create an agent using both Dialogflow Standard Edition and"]
    #[doc = " Dialogflow Enterprise Edition. For details, see"]
    #[doc = " [Dialogflow"]
    #[doc = " Editions](https://cloud.google.com/dialogflow/docs/editions)."]
    #[doc = ""]
    #[doc = " You can save your agent for backup or versioning by exporting the agent by"]
    #[doc = " using the [ExportAgent][google.cloud.dialogflow.v2.Agents.ExportAgent] method. You can import a saved"]
    #[doc = " agent by using the [ImportAgent][google.cloud.dialogflow.v2.Agents.ImportAgent] method."]
    #[doc = ""]
    #[doc = " Dialogflow provides several"]
    #[doc = " [prebuilt"]
    #[doc = " agents](https://cloud.google.com/dialogflow/docs/agents-prebuilt)"]
    #[doc = " for common conversation scenarios such as determining a date and time,"]
    #[doc = " converting currency, and so on."]
    #[doc = ""]
    #[doc = " For more information about agents, see the"]
    #[doc = " [Dialogflow"]
    #[doc = " documentation](https://cloud.google.com/dialogflow/docs/agents-overview)."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct AgentsServer<T: Agents> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Agents> AgentsServer<T> {
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
    impl<T, B> Service<http::Request<B>> for AgentsServer<T>
    where
        T: Agents,
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
                "/google.cloud.dialogflow.v2.Agents/GetAgent" => {
                    #[allow(non_camel_case_types)]
                    struct GetAgentSvc<T: Agents>(pub Arc<T>);
                    impl<T: Agents> tonic::server::UnaryService<super::GetAgentRequest> for GetAgentSvc<T> {
                        type Response = super::Agent;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetAgentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_agent(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetAgentSvc(inner);
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
                "/google.cloud.dialogflow.v2.Agents/SetAgent" => {
                    #[allow(non_camel_case_types)]
                    struct SetAgentSvc<T: Agents>(pub Arc<T>);
                    impl<T: Agents> tonic::server::UnaryService<super::SetAgentRequest> for SetAgentSvc<T> {
                        type Response = super::Agent;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetAgentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.set_agent(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SetAgentSvc(inner);
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
                "/google.cloud.dialogflow.v2.Agents/DeleteAgent" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteAgentSvc<T: Agents>(pub Arc<T>);
                    impl<T: Agents> tonic::server::UnaryService<super::DeleteAgentRequest> for DeleteAgentSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteAgentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_agent(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteAgentSvc(inner);
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
                "/google.cloud.dialogflow.v2.Agents/SearchAgents" => {
                    #[allow(non_camel_case_types)]
                    struct SearchAgentsSvc<T: Agents>(pub Arc<T>);
                    impl<T: Agents> tonic::server::UnaryService<super::SearchAgentsRequest> for SearchAgentsSvc<T> {
                        type Response = super::SearchAgentsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SearchAgentsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.search_agents(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = SearchAgentsSvc(inner);
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
                "/google.cloud.dialogflow.v2.Agents/TrainAgent" => {
                    #[allow(non_camel_case_types)]
                    struct TrainAgentSvc<T: Agents>(pub Arc<T>);
                    impl<T: Agents> tonic::server::UnaryService<super::TrainAgentRequest> for TrainAgentSvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::TrainAgentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.train_agent(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = TrainAgentSvc(inner);
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
                "/google.cloud.dialogflow.v2.Agents/ExportAgent" => {
                    #[allow(non_camel_case_types)]
                    struct ExportAgentSvc<T: Agents>(pub Arc<T>);
                    impl<T: Agents> tonic::server::UnaryService<super::ExportAgentRequest> for ExportAgentSvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ExportAgentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.export_agent(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ExportAgentSvc(inner);
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
                "/google.cloud.dialogflow.v2.Agents/ImportAgent" => {
                    #[allow(non_camel_case_types)]
                    struct ImportAgentSvc<T: Agents>(pub Arc<T>);
                    impl<T: Agents> tonic::server::UnaryService<super::ImportAgentRequest> for ImportAgentSvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ImportAgentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.import_agent(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ImportAgentSvc(inner);
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
                "/google.cloud.dialogflow.v2.Agents/RestoreAgent" => {
                    #[allow(non_camel_case_types)]
                    struct RestoreAgentSvc<T: Agents>(pub Arc<T>);
                    impl<T: Agents> tonic::server::UnaryService<super::RestoreAgentRequest> for RestoreAgentSvc<T> {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RestoreAgentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.restore_agent(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = RestoreAgentSvc(inner);
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
                "/google.cloud.dialogflow.v2.Agents/GetValidationResult" => {
                    #[allow(non_camel_case_types)]
                    struct GetValidationResultSvc<T: Agents>(pub Arc<T>);
                    impl<T: Agents> tonic::server::UnaryService<super::GetValidationResultRequest>
                        for GetValidationResultSvc<T>
                    {
                        type Response = super::ValidationResult;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetValidationResultRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_validation_result(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetValidationResultSvc(inner);
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
    impl<T: Agents> Clone for AgentsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Agents> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Agents> tonic::transport::NamedService for AgentsServer<T> {
        const NAME: &'static str = "google.cloud.dialogflow.v2.Agents";
    }
}
/// Hints for the speech recognizer to help with recognition in a specific
/// conversation state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeechContext {
    /// Optional. A list of strings containing words and phrases that the speech
    /// recognizer should recognize with higher likelihood.
    ///
    /// This list can be used to:
    /// * improve accuracy for words and phrases you expect the user to say,
    ///   e.g. typical commands for your Dialogflow agent
    /// * add additional words to the speech recognizer vocabulary
    /// * ...
    ///
    /// See the [Cloud Speech
    /// documentation](https://cloud.google.com/speech-to-text/quotas) for usage
    /// limits.
    #[prost(string, repeated, tag = "1")]
    pub phrases: ::std::vec::Vec<std::string::String>,
    /// Optional. Boost for this context compared to other contexts:
    ///
    /// * If the boost is positive, Dialogflow will increase the probability that
    ///   the phrases in this context are recognized over similar sounding phrases.
    /// * If the boost is unspecified or non-positive, Dialogflow will not apply
    ///   any boost.
    ///
    /// Dialogflow recommends that you use boosts in the range (0, 20] and that you
    /// find a value that fits your use case with binary search.
    #[prost(float, tag = "2")]
    pub boost: f32,
}
/// Information for a word recognized by the speech recognizer.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SpeechWordInfo {
    /// The word this info is for.
    #[prost(string, tag = "3")]
    pub word: std::string::String,
    /// Time offset relative to the beginning of the audio that corresponds to the
    /// start of the spoken word. This is an experimental feature and the accuracy
    /// of the time offset can vary.
    #[prost(message, optional, tag = "1")]
    pub start_offset: ::std::option::Option<::prost_types::Duration>,
    /// Time offset relative to the beginning of the audio that corresponds to the
    /// end of the spoken word. This is an experimental feature and the accuracy of
    /// the time offset can vary.
    #[prost(message, optional, tag = "2")]
    pub end_offset: ::std::option::Option<::prost_types::Duration>,
    /// The Speech confidence between 0.0 and 1.0 for this word. A higher number
    /// indicates an estimated greater likelihood that the recognized word is
    /// correct. The default of 0.0 is a sentinel value indicating that confidence
    /// was not set.
    ///
    /// This field is not guaranteed to be fully stable over time for the same
    /// audio input. Users should also not rely on it to always be provided.
    #[prost(float, tag = "4")]
    pub confidence: f32,
}
/// Instructs the speech recognizer how to process the audio content.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InputAudioConfig {
    /// Required. Audio encoding of the audio content to process.
    #[prost(enumeration = "AudioEncoding", tag = "1")]
    pub audio_encoding: i32,
    /// Required. Sample rate (in Hertz) of the audio content sent in the query.
    /// Refer to
    /// [Cloud Speech API
    /// documentation](https://cloud.google.com/speech-to-text/docs/basics) for
    /// more details.
    #[prost(int32, tag = "2")]
    pub sample_rate_hertz: i32,
    /// Required. The language of the supplied audio. Dialogflow does not do
    /// translations. See [Language
    /// Support](https://cloud.google.com/dialogflow/docs/reference/language)
    /// for a list of the currently supported language codes. Note that queries in
    /// the same session do not necessarily need to specify the same language.
    #[prost(string, tag = "3")]
    pub language_code: std::string::String,
    /// If `true`, Dialogflow returns [SpeechWordInfo][google.cloud.dialogflow.v2.SpeechWordInfo] in
    /// [StreamingRecognitionResult][google.cloud.dialogflow.v2.StreamingRecognitionResult] with information about the recognized speech
    /// words, e.g. start and end time offsets. If false or unspecified, Speech
    /// doesn't return any word-level information.
    #[prost(bool, tag = "13")]
    pub enable_word_info: bool,
    /// A list of strings containing words and phrases that the speech
    /// recognizer should recognize with higher likelihood.
    ///
    /// See [the Cloud Speech
    /// documentation](https://cloud.google.com/speech-to-text/docs/basics#phrase-hints)
    /// for more details.
    ///
    /// This field is deprecated. Please use [speech_contexts]() instead. If you
    /// specify both [phrase_hints]() and [speech_contexts](), Dialogflow will
    /// treat the [phrase_hints]() as a single additional [SpeechContext]().
    #[prost(string, repeated, tag = "4")]
    pub phrase_hints: ::std::vec::Vec<std::string::String>,
    /// Context information to assist speech recognition.
    ///
    /// See [the Cloud Speech
    /// documentation](https://cloud.google.com/speech-to-text/docs/basics#phrase-hints)
    /// for more details.
    #[prost(message, repeated, tag = "11")]
    pub speech_contexts: ::std::vec::Vec<SpeechContext>,
    /// Which Speech model to select for the given request. Select the
    /// model best suited to your domain to get best results. If a model is not
    /// explicitly specified, then we auto-select a model based on the parameters
    /// in the InputAudioConfig.
    /// If enhanced speech model is enabled for the agent and an enhanced
    /// version of the specified model for the language does not exist, then the
    /// speech is recognized using the standard version of the specified model.
    /// Refer to
    /// [Cloud Speech API
    /// documentation](https://cloud.google.com/speech-to-text/docs/basics#select-model)
    /// for more details.
    #[prost(string, tag = "7")]
    pub model: std::string::String,
    /// Which variant of the [Speech model][google.cloud.dialogflow.v2.InputAudioConfig.model] to use.
    #[prost(enumeration = "SpeechModelVariant", tag = "10")]
    pub model_variant: i32,
    /// If `false` (default), recognition does not cease until the
    /// client closes the stream.
    /// If `true`, the recognizer will detect a single spoken utterance in input
    /// audio. Recognition ceases when it detects the audio's voice has
    /// stopped or paused. In this case, once a detected intent is received, the
    /// client should close the stream and start a new request with a new stream as
    /// needed.
    /// Note: This setting is relevant only for streaming methods.
    /// Note: When specified, InputAudioConfig.single_utterance takes precedence
    /// over StreamingDetectIntentRequest.single_utterance.
    #[prost(bool, tag = "8")]
    pub single_utterance: bool,
}
/// Description of which voice to use for speech synthesis.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoiceSelectionParams {
    /// Optional. The name of the voice. If not set, the service will choose a
    /// voice based on the other parameters such as language_code and
    /// [ssml_gender][google.cloud.dialogflow.v2.VoiceSelectionParams.ssml_gender].
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Optional. The preferred gender of the voice. If not set, the service will
    /// choose a voice based on the other parameters such as language_code and
    /// [name][google.cloud.dialogflow.v2.VoiceSelectionParams.name]. Note that this is only a preference, not requirement. If a
    /// voice of the appropriate gender is not available, the synthesizer should
    /// substitute a voice with a different gender rather than failing the request.
    #[prost(enumeration = "SsmlVoiceGender", tag = "2")]
    pub ssml_gender: i32,
}
/// Configuration of how speech should be synthesized.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SynthesizeSpeechConfig {
    /// Optional. Speaking rate/speed, in the range [0.25, 4.0]. 1.0 is the normal
    /// native speed supported by the specific voice. 2.0 is twice as fast, and
    /// 0.5 is half as fast. If unset(0.0), defaults to the native 1.0 speed. Any
    /// other values < 0.25 or > 4.0 will return an error.
    #[prost(double, tag = "1")]
    pub speaking_rate: f64,
    /// Optional. Speaking pitch, in the range [-20.0, 20.0]. 20 means increase 20
    /// semitones from the original pitch. -20 means decrease 20 semitones from the
    /// original pitch.
    #[prost(double, tag = "2")]
    pub pitch: f64,
    /// Optional. Volume gain (in dB) of the normal native volume supported by the
    /// specific voice, in the range [-96.0, 16.0]. If unset, or set to a value of
    /// 0.0 (dB), will play at normal native signal amplitude. A value of -6.0 (dB)
    /// will play at approximately half the amplitude of the normal native signal
    /// amplitude. A value of +6.0 (dB) will play at approximately twice the
    /// amplitude of the normal native signal amplitude. We strongly recommend not
    /// to exceed +10 (dB) as there's usually no effective increase in loudness for
    /// any value greater than that.
    #[prost(double, tag = "3")]
    pub volume_gain_db: f64,
    /// Optional. An identifier which selects 'audio effects' profiles that are
    /// applied on (post synthesized) text to speech. Effects are applied on top of
    /// each other in the order they are given.
    #[prost(string, repeated, tag = "5")]
    pub effects_profile_id: ::std::vec::Vec<std::string::String>,
    /// Optional. The desired voice of the synthesized audio.
    #[prost(message, optional, tag = "4")]
    pub voice: ::std::option::Option<VoiceSelectionParams>,
}
/// Instructs the speech synthesizer on how to generate the output audio content.
/// If this audio config is supplied in a request, it overrides all existing
/// text-to-speech settings applied to the agent.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutputAudioConfig {
    /// Required. Audio encoding of the synthesized audio content.
    #[prost(enumeration = "OutputAudioEncoding", tag = "1")]
    pub audio_encoding: i32,
    /// The synthesis sample rate (in hertz) for this audio. If not
    /// provided, then the synthesizer will use the default sample rate based on
    /// the audio encoding. If this is different from the voice's natural sample
    /// rate, then the synthesizer will honor this request by converting to the
    /// desired sample rate (which might result in worse audio quality).
    #[prost(int32, tag = "2")]
    pub sample_rate_hertz: i32,
    /// Configuration of how speech should be synthesized.
    #[prost(message, optional, tag = "3")]
    pub synthesize_speech_config: ::std::option::Option<SynthesizeSpeechConfig>,
}
/// Audio encoding of the audio content sent in the conversational query request.
/// Refer to the
/// [Cloud Speech API
/// documentation](https://cloud.google.com/speech-to-text/docs/basics) for more
/// details.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AudioEncoding {
    /// Not specified.
    Unspecified = 0,
    /// Uncompressed 16-bit signed little-endian samples (Linear PCM).
    Linear16 = 1,
    /// [`FLAC`](https://xiph.org/flac/documentation.html) (Free Lossless Audio
    /// Codec) is the recommended encoding because it is lossless (therefore
    /// recognition is not compromised) and requires only about half the
    /// bandwidth of `LINEAR16`. `FLAC` stream encoding supports 16-bit and
    /// 24-bit samples, however, not all fields in `STREAMINFO` are supported.
    Flac = 2,
    /// 8-bit samples that compand 14-bit audio samples using G.711 PCMU/mu-law.
    Mulaw = 3,
    /// Adaptive Multi-Rate Narrowband codec. `sample_rate_hertz` must be 8000.
    Amr = 4,
    /// Adaptive Multi-Rate Wideband codec. `sample_rate_hertz` must be 16000.
    AmrWb = 5,
    /// Opus encoded audio frames in Ogg container
    /// ([OggOpus](https://wiki.xiph.org/OggOpus)).
    /// `sample_rate_hertz` must be 16000.
    OggOpus = 6,
    /// Although the use of lossy encodings is not recommended, if a very low
    /// bitrate encoding is required, `OGG_OPUS` is highly preferred over
    /// Speex encoding. The [Speex](https://speex.org/) encoding supported by
    /// Dialogflow API has a header byte in each block, as in MIME type
    /// `audio/x-speex-with-header-byte`.
    /// It is a variant of the RTP Speex encoding defined in
    /// [RFC 5574](https://tools.ietf.org/html/rfc5574).
    /// The stream is a sequence of blocks, one block per RTP packet. Each block
    /// starts with a byte containing the length of the block, in bytes, followed
    /// by one or more frames of Speex data, padded to an integral number of
    /// bytes (octets) as specified in RFC 5574. In other words, each RTP header
    /// is replaced with a single byte containing the block length. Only Speex
    /// wideband is supported. `sample_rate_hertz` must be 16000.
    SpeexWithHeaderByte = 7,
}
/// Variant of the specified [Speech model][google.cloud.dialogflow.v2.InputAudioConfig.model] to use.
///
/// See the [Cloud Speech
/// documentation](https://cloud.google.com/speech-to-text/docs/enhanced-models)
/// for which models have different variants. For example, the "phone_call" model
/// has both a standard and an enhanced variant. When you use an enhanced model,
/// you will generally receive higher quality results than for a standard model.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SpeechModelVariant {
    /// No model variant specified. In this case Dialogflow defaults to
    /// USE_BEST_AVAILABLE.
    Unspecified = 0,
    /// Use the best available variant of the [Speech
    /// model][InputAudioConfig.model] that the caller is eligible for.
    ///
    /// Please see the [Dialogflow
    /// docs](https://cloud.google.com/dialogflow/docs/data-logging) for
    /// how to make your project eligible for enhanced models.
    UseBestAvailable = 1,
    /// Use standard model variant even if an enhanced model is available.  See the
    /// [Cloud Speech
    /// documentation](https://cloud.google.com/speech-to-text/docs/enhanced-models)
    /// for details about enhanced models.
    UseStandard = 2,
    /// Use an enhanced model variant:
    ///
    /// * If an enhanced variant does not exist for the given
    ///   [model][google.cloud.dialogflow.v2.InputAudioConfig.model] and request language, Dialogflow falls
    ///   back to the standard variant.
    ///
    ///   The [Cloud Speech
    ///   documentation](https://cloud.google.com/speech-to-text/docs/enhanced-models)
    ///   describes which models have enhanced variants.
    ///
    /// * If the API caller isn't eligible for enhanced models, Dialogflow returns
    ///   an error. Please see the [Dialogflow
    ///   docs](https://cloud.google.com/dialogflow/docs/data-logging)
    ///   for how to make your project eligible.
    UseEnhanced = 3,
}
/// Gender of the voice as described in
/// [SSML voice element](https://www.w3.org/TR/speech-synthesis11/#edef_voice).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SsmlVoiceGender {
    /// An unspecified gender, which means that the client doesn't care which
    /// gender the selected voice will have.
    Unspecified = 0,
    /// A male voice.
    Male = 1,
    /// A female voice.
    Female = 2,
    /// A gender-neutral voice.
    Neutral = 3,
}
/// Audio encoding of the output audio format in Text-To-Speech.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum OutputAudioEncoding {
    /// Not specified.
    Unspecified = 0,
    /// Uncompressed 16-bit signed little-endian samples (Linear PCM).
    /// Audio content returned as LINEAR16 also contains a WAV header.
    Linear16 = 1,
    /// MP3 audio at 32kbps.
    Mp3 = 2,
    /// Opus encoded audio wrapped in an ogg container. The result will be a
    /// file which can be played natively on Android, and in browsers (at least
    /// Chrome and Firefox). The quality of the encoding is considerably higher
    /// than MP3 while using approximately the same bitrate.
    OggOpus = 3,
}
/// Represents a context.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Context {
    /// Required. The unique identifier of the context. Format:
    /// `projects/<Project ID>/agent/sessions/<Session ID>/contexts/<Context ID>`,
    /// or `projects/<Project ID>/agent/environments/<Environment ID>/users/<User
    /// ID>/sessions/<Session ID>/contexts/<Context ID>`.
    ///
    /// The `Context ID` is always converted to lowercase, may only contain
    /// characters in a-zA-Z0-9_-% and may be at most 250 bytes long.
    ///
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment. If `User ID` is not specified, we assume default '-' user.
    ///
    /// The following context names are reserved for internal use by Dialogflow.
    /// You should not use these contexts or create contexts with these names:
    ///
    /// * `__system_counters__`
    /// * `*_id_dialog_context`
    /// * `*_dialog_params_size`
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Optional. The number of conversational query requests after which the
    /// context expires. The default is `0`. If set to `0`, the context expires
    /// immediately. Contexts expire automatically after 20 minutes if there
    /// are no matching queries.
    #[prost(int32, tag = "2")]
    pub lifespan_count: i32,
    /// Optional. The collection of parameters associated with this context.
    ///
    /// Depending on your protocol or client library language, this is a
    /// map, associative array, symbol table, dictionary, or JSON object
    /// composed of a collection of (MapKey, MapValue) pairs:
    ///
    /// -   MapKey type: string
    /// -   MapKey value: parameter name
    /// -   MapValue type:
    ///     -   If parameter's entity type is a composite entity: map
    ///     -   Else: string or number, depending on parameter value type
    /// -   MapValue value:
    ///     -   If parameter's entity type is a composite entity:
    ///         map from composite entity property names to property values
    ///     -   Else: parameter value
    #[prost(message, optional, tag = "3")]
    pub parameters: ::std::option::Option<::prost_types::Struct>,
}
/// The request message for [Contexts.ListContexts][google.cloud.dialogflow.v2.Contexts.ListContexts].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListContextsRequest {
    /// Required. The session to list all contexts from.
    /// Format: `projects/<Project ID>/agent/sessions/<Session ID>` or
    /// `projects/<Project ID>/agent/environments/<Environment ID>/users/<User
    /// ID>/sessions/<Session ID>`.
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment. If `User ID` is not specified, we assume default '-' user.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The maximum number of items to return in a single page. By
    /// default 100 and at most 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// The response message for [Contexts.ListContexts][google.cloud.dialogflow.v2.Contexts.ListContexts].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListContextsResponse {
    /// The list of contexts. There will be a maximum number of items
    /// returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub contexts: ::std::vec::Vec<Context>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The request message for [Contexts.GetContext][google.cloud.dialogflow.v2.Contexts.GetContext].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetContextRequest {
    /// Required. The name of the context. Format:
    /// `projects/<Project ID>/agent/sessions/<Session ID>/contexts/<Context ID>`
    /// or `projects/<Project ID>/agent/environments/<Environment ID>/users/<User
    /// ID>/sessions/<Session ID>/contexts/<Context ID>`.
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment. If `User ID` is not specified, we assume default '-' user.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The request message for [Contexts.CreateContext][google.cloud.dialogflow.v2.Contexts.CreateContext].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateContextRequest {
    /// Required. The session to create a context for.
    /// Format: `projects/<Project ID>/agent/sessions/<Session ID>` or
    /// `projects/<Project ID>/agent/environments/<Environment ID>/users/<User
    /// ID>/sessions/<Session ID>`.
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment. If `User ID` is not specified, we assume default '-' user.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The context to create.
    #[prost(message, optional, tag = "2")]
    pub context: ::std::option::Option<Context>,
}
/// The request message for [Contexts.UpdateContext][google.cloud.dialogflow.v2.Contexts.UpdateContext].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateContextRequest {
    /// Required. The context to update.
    #[prost(message, optional, tag = "1")]
    pub context: ::std::option::Option<Context>,
    /// Optional. The mask to control which fields get updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// The request message for [Contexts.DeleteContext][google.cloud.dialogflow.v2.Contexts.DeleteContext].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteContextRequest {
    /// Required. The name of the context to delete. Format:
    /// `projects/<Project ID>/agent/sessions/<Session ID>/contexts/<Context ID>`
    /// or `projects/<Project ID>/agent/environments/<Environment ID>/users/<User
    /// ID>/sessions/<Session ID>/contexts/<Context ID>`.
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment. If `User ID` is not specified, we assume default '-' user.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The request message for [Contexts.DeleteAllContexts][google.cloud.dialogflow.v2.Contexts.DeleteAllContexts].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAllContextsRequest {
    /// Required. The name of the session to delete all contexts from. Format:
    /// `projects/<Project ID>/agent/sessions/<Session ID>` or `projects/<Project
    /// ID>/agent/environments/<Environment ID>/users/<User ID>/sessions/<Session
    /// ID>`.
    /// If `Environment ID` is not specified we assume default 'draft' environment.
    /// If `User ID` is not specified, we assume default '-' user.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod contexts_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " A context represents additional information included with user input or with"]
    #[doc = " an intent returned by the Dialogflow API. Contexts are helpful for"]
    #[doc = " differentiating user input which may be vague or have a different meaning"]
    #[doc = " depending on additional details from your application such as user setting"]
    #[doc = " and preferences, previous user input, where the user is in your application,"]
    #[doc = " geographic location, and so on."]
    #[doc = ""]
    #[doc = " You can include contexts as input parameters of a"]
    #[doc = " [DetectIntent][google.cloud.dialogflow.v2.Sessions.DetectIntent] (or"]
    #[doc = " [StreamingDetectIntent][google.cloud.dialogflow.v2.Sessions.StreamingDetectIntent]) request,"]
    #[doc = " or as output contexts included in the returned intent."]
    #[doc = " Contexts expire when an intent is matched, after the number of `DetectIntent`"]
    #[doc = " requests specified by the `lifespan_count` parameter, or after 20 minutes"]
    #[doc = " if no intents are matched for a `DetectIntent` request."]
    #[doc = ""]
    #[doc = " For more information about contexts, see the"]
    #[doc = " [Dialogflow"]
    #[doc = " documentation](https://cloud.google.com/dialogflow/docs/contexts-overview)."]
    pub struct ContextsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ContextsClient<tonic::transport::Channel> {
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
    impl<T> ContextsClient<T>
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
        #[doc = " Returns the list of all contexts in the specified session."]
        pub async fn list_contexts(
            &mut self,
            request: impl tonic::IntoRequest<super::ListContextsRequest>,
        ) -> Result<tonic::Response<super::ListContextsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.v2.Contexts/ListContexts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the specified context."]
        pub async fn get_context(
            &mut self,
            request: impl tonic::IntoRequest<super::GetContextRequest>,
        ) -> Result<tonic::Response<super::Context>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.v2.Contexts/GetContext",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a context."]
        #[doc = ""]
        #[doc = " If the specified context already exists, overrides the context."]
        pub async fn create_context(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateContextRequest>,
        ) -> Result<tonic::Response<super::Context>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.v2.Contexts/CreateContext",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified context."]
        pub async fn update_context(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateContextRequest>,
        ) -> Result<tonic::Response<super::Context>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.v2.Contexts/UpdateContext",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified context."]
        pub async fn delete_context(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteContextRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.v2.Contexts/DeleteContext",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes all active contexts in the specified session."]
        pub async fn delete_all_contexts(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteAllContextsRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.v2.Contexts/DeleteAllContexts",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ContextsClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ContextsClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ContextsClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod contexts_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ContextsServer."]
    #[async_trait]
    pub trait Contexts: Send + Sync + 'static {
        #[doc = " Returns the list of all contexts in the specified session."]
        async fn list_contexts(
            &self,
            request: tonic::Request<super::ListContextsRequest>,
        ) -> Result<tonic::Response<super::ListContextsResponse>, tonic::Status>;
        #[doc = " Retrieves the specified context."]
        async fn get_context(
            &self,
            request: tonic::Request<super::GetContextRequest>,
        ) -> Result<tonic::Response<super::Context>, tonic::Status>;
        #[doc = " Creates a context."]
        #[doc = ""]
        #[doc = " If the specified context already exists, overrides the context."]
        async fn create_context(
            &self,
            request: tonic::Request<super::CreateContextRequest>,
        ) -> Result<tonic::Response<super::Context>, tonic::Status>;
        #[doc = " Updates the specified context."]
        async fn update_context(
            &self,
            request: tonic::Request<super::UpdateContextRequest>,
        ) -> Result<tonic::Response<super::Context>, tonic::Status>;
        #[doc = " Deletes the specified context."]
        async fn delete_context(
            &self,
            request: tonic::Request<super::DeleteContextRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Deletes all active contexts in the specified session."]
        async fn delete_all_contexts(
            &self,
            request: tonic::Request<super::DeleteAllContextsRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
    }
    #[doc = " A context represents additional information included with user input or with"]
    #[doc = " an intent returned by the Dialogflow API. Contexts are helpful for"]
    #[doc = " differentiating user input which may be vague or have a different meaning"]
    #[doc = " depending on additional details from your application such as user setting"]
    #[doc = " and preferences, previous user input, where the user is in your application,"]
    #[doc = " geographic location, and so on."]
    #[doc = ""]
    #[doc = " You can include contexts as input parameters of a"]
    #[doc = " [DetectIntent][google.cloud.dialogflow.v2.Sessions.DetectIntent] (or"]
    #[doc = " [StreamingDetectIntent][google.cloud.dialogflow.v2.Sessions.StreamingDetectIntent]) request,"]
    #[doc = " or as output contexts included in the returned intent."]
    #[doc = " Contexts expire when an intent is matched, after the number of `DetectIntent`"]
    #[doc = " requests specified by the `lifespan_count` parameter, or after 20 minutes"]
    #[doc = " if no intents are matched for a `DetectIntent` request."]
    #[doc = ""]
    #[doc = " For more information about contexts, see the"]
    #[doc = " [Dialogflow"]
    #[doc = " documentation](https://cloud.google.com/dialogflow/docs/contexts-overview)."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct ContextsServer<T: Contexts> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Contexts> ContextsServer<T> {
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
    impl<T, B> Service<http::Request<B>> for ContextsServer<T>
    where
        T: Contexts,
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
                "/google.cloud.dialogflow.v2.Contexts/ListContexts" => {
                    #[allow(non_camel_case_types)]
                    struct ListContextsSvc<T: Contexts>(pub Arc<T>);
                    impl<T: Contexts> tonic::server::UnaryService<super::ListContextsRequest> for ListContextsSvc<T> {
                        type Response = super::ListContextsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListContextsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_contexts(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListContextsSvc(inner);
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
                "/google.cloud.dialogflow.v2.Contexts/GetContext" => {
                    #[allow(non_camel_case_types)]
                    struct GetContextSvc<T: Contexts>(pub Arc<T>);
                    impl<T: Contexts> tonic::server::UnaryService<super::GetContextRequest> for GetContextSvc<T> {
                        type Response = super::Context;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetContextRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_context(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetContextSvc(inner);
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
                "/google.cloud.dialogflow.v2.Contexts/CreateContext" => {
                    #[allow(non_camel_case_types)]
                    struct CreateContextSvc<T: Contexts>(pub Arc<T>);
                    impl<T: Contexts> tonic::server::UnaryService<super::CreateContextRequest> for CreateContextSvc<T> {
                        type Response = super::Context;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateContextRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_context(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateContextSvc(inner);
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
                "/google.cloud.dialogflow.v2.Contexts/UpdateContext" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateContextSvc<T: Contexts>(pub Arc<T>);
                    impl<T: Contexts> tonic::server::UnaryService<super::UpdateContextRequest> for UpdateContextSvc<T> {
                        type Response = super::Context;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateContextRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_context(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateContextSvc(inner);
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
                "/google.cloud.dialogflow.v2.Contexts/DeleteContext" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteContextSvc<T: Contexts>(pub Arc<T>);
                    impl<T: Contexts> tonic::server::UnaryService<super::DeleteContextRequest> for DeleteContextSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteContextRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_context(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteContextSvc(inner);
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
                "/google.cloud.dialogflow.v2.Contexts/DeleteAllContexts" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteAllContextsSvc<T: Contexts>(pub Arc<T>);
                    impl<T: Contexts> tonic::server::UnaryService<super::DeleteAllContextsRequest>
                        for DeleteAllContextsSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteAllContextsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_all_contexts(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteAllContextsSvc(inner);
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
    impl<T: Contexts> Clone for ContextsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Contexts> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Contexts> tonic::transport::NamedService for ContextsServer<T> {
        const NAME: &'static str = "google.cloud.dialogflow.v2.Contexts";
    }
}
/// Represents an entity type.
/// Entity types serve as a tool for extracting parameter values from natural
/// language queries.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityType {
    /// The unique identifier of the entity type.
    /// Required for [EntityTypes.UpdateEntityType][google.cloud.dialogflow.v2.EntityTypes.UpdateEntityType] and
    /// [EntityTypes.BatchUpdateEntityTypes][google.cloud.dialogflow.v2.EntityTypes.BatchUpdateEntityTypes] methods.
    /// Format: `projects/<Project ID>/agent/entityTypes/<Entity Type ID>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The name of the entity type.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// Required. Indicates the kind of entity type.
    #[prost(enumeration = "entity_type::Kind", tag = "3")]
    pub kind: i32,
    /// Optional. Indicates whether the entity type can be automatically
    /// expanded.
    #[prost(enumeration = "entity_type::AutoExpansionMode", tag = "4")]
    pub auto_expansion_mode: i32,
    /// Optional. The collection of entity entries associated with the entity type.
    #[prost(message, repeated, tag = "6")]
    pub entities: ::std::vec::Vec<entity_type::Entity>,
    /// Optional. Enables fuzzy entity extraction during classification.
    #[prost(bool, tag = "7")]
    pub enable_fuzzy_extraction: bool,
}
pub mod entity_type {
    /// An **entity entry** for an associated entity type.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Entity {
        /// Required. The primary value associated with this entity entry.
        /// For example, if the entity type is *vegetable*, the value could be
        /// *scallions*.
        ///
        /// For `KIND_MAP` entity types:
        ///
        /// *   A reference value to be used in place of synonyms.
        ///
        /// For `KIND_LIST` entity types:
        ///
        /// *   A string that can contain references to other entity types (with or
        ///     without aliases).
        #[prost(string, tag = "1")]
        pub value: std::string::String,
        /// Required. A collection of value synonyms. For example, if the entity type
        /// is *vegetable*, and `value` is *scallions*, a synonym could be *green
        /// onions*.
        ///
        /// For `KIND_LIST` entity types:
        ///
        /// *   This collection must contain exactly one synonym equal to `value`.
        #[prost(string, repeated, tag = "2")]
        pub synonyms: ::std::vec::Vec<std::string::String>,
    }
    /// Represents kinds of entities.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Kind {
        /// Not specified. This value should be never used.
        Unspecified = 0,
        /// Map entity types allow mapping of a group of synonyms to a reference
        /// value.
        Map = 1,
        /// List entity types contain a set of entries that do not map to reference
        /// values. However, list entity types can contain references to other entity
        /// types (with or without aliases).
        List = 2,
        /// Regexp entity types allow to specify regular expressions in entries
        /// values.
        Regexp = 3,
    }
    /// Represents different entity type expansion modes. Automated expansion
    /// allows an agent to recognize values that have not been explicitly listed in
    /// the entity (for example, new kinds of shopping list items).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum AutoExpansionMode {
        /// Auto expansion disabled for the entity.
        Unspecified = 0,
        /// Allows an agent to recognize values that have not been explicitly
        /// listed in the entity.
        Default = 1,
    }
}
/// The request message for [EntityTypes.ListEntityTypes][google.cloud.dialogflow.v2.EntityTypes.ListEntityTypes].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntityTypesRequest {
    /// Required. The agent to list all entity types from.
    /// Format: `projects/<Project ID>/agent`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The language used to access language-specific data.
    /// If not specified, the agent's default language is used.
    /// For more information, see
    /// [Multilingual intent and entity
    /// data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity).
    #[prost(string, tag = "2")]
    pub language_code: std::string::String,
    /// Optional. The maximum number of items to return in a single page. By
    /// default 100 and at most 1000.
    #[prost(int32, tag = "3")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous list request.
    #[prost(string, tag = "4")]
    pub page_token: std::string::String,
}
/// The response message for [EntityTypes.ListEntityTypes][google.cloud.dialogflow.v2.EntityTypes.ListEntityTypes].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEntityTypesResponse {
    /// The list of agent entity types. There will be a maximum number of items
    /// returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub entity_types: ::std::vec::Vec<EntityType>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The request message for [EntityTypes.GetEntityType][google.cloud.dialogflow.v2.EntityTypes.GetEntityType].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetEntityTypeRequest {
    /// Required. The name of the entity type.
    /// Format: `projects/<Project ID>/agent/entityTypes/<EntityType ID>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Optional. The language used to access language-specific data.
    /// If not specified, the agent's default language is used.
    /// For more information, see
    /// [Multilingual intent and entity
    /// data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity).
    #[prost(string, tag = "2")]
    pub language_code: std::string::String,
}
/// The request message for [EntityTypes.CreateEntityType][google.cloud.dialogflow.v2.EntityTypes.CreateEntityType].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEntityTypeRequest {
    /// Required. The agent to create a entity type for.
    /// Format: `projects/<Project ID>/agent`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The entity type to create.
    #[prost(message, optional, tag = "2")]
    pub entity_type: ::std::option::Option<EntityType>,
    /// Optional. The language used to access language-specific data.
    /// If not specified, the agent's default language is used.
    /// For more information, see
    /// [Multilingual intent and entity
    /// data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity).
    #[prost(string, tag = "3")]
    pub language_code: std::string::String,
}
/// The request message for [EntityTypes.UpdateEntityType][google.cloud.dialogflow.v2.EntityTypes.UpdateEntityType].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateEntityTypeRequest {
    /// Required. The entity type to update.
    #[prost(message, optional, tag = "1")]
    pub entity_type: ::std::option::Option<EntityType>,
    /// Optional. The language used to access language-specific data.
    /// If not specified, the agent's default language is used.
    /// For more information, see
    /// [Multilingual intent and entity
    /// data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity).
    #[prost(string, tag = "2")]
    pub language_code: std::string::String,
    /// Optional. The mask to control which fields get updated.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// The request message for [EntityTypes.DeleteEntityType][google.cloud.dialogflow.v2.EntityTypes.DeleteEntityType].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteEntityTypeRequest {
    /// Required. The name of the entity type to delete.
    /// Format: `projects/<Project ID>/agent/entityTypes/<EntityType ID>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The request message for [EntityTypes.BatchUpdateEntityTypes][google.cloud.dialogflow.v2.EntityTypes.BatchUpdateEntityTypes].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateEntityTypesRequest {
    /// Required. The name of the agent to update or create entity types in.
    /// Format: `projects/<Project ID>/agent`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The language used to access language-specific data.
    /// If not specified, the agent's default language is used.
    /// For more information, see
    /// [Multilingual intent and entity
    /// data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity).
    #[prost(string, tag = "4")]
    pub language_code: std::string::String,
    /// Optional. The mask to control which fields get updated.
    #[prost(message, optional, tag = "5")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The source of the entity type batch.
    ///
    /// For each entity type in the batch:
    ///
    /// *   If `name` is specified, we update an existing entity type.
    /// *   If `name` is not specified, we create a new entity type.
    #[prost(
        oneof = "batch_update_entity_types_request::EntityTypeBatch",
        tags = "2, 3"
    )]
    pub entity_type_batch:
        ::std::option::Option<batch_update_entity_types_request::EntityTypeBatch>,
}
pub mod batch_update_entity_types_request {
    /// The source of the entity type batch.
    ///
    /// For each entity type in the batch:
    ///
    /// *   If `name` is specified, we update an existing entity type.
    /// *   If `name` is not specified, we create a new entity type.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum EntityTypeBatch {
        /// The URI to a Google Cloud Storage file containing entity types to update
        /// or create. The file format can either be a serialized proto (of
        /// EntityBatch type) or a JSON object. Note: The URI must start with
        /// "gs://".
        #[prost(string, tag = "2")]
        EntityTypeBatchUri(std::string::String),
        /// The collection of entity types to update or create.
        #[prost(message, tag = "3")]
        EntityTypeBatchInline(super::EntityTypeBatch),
    }
}
/// The response message for [EntityTypes.BatchUpdateEntityTypes][google.cloud.dialogflow.v2.EntityTypes.BatchUpdateEntityTypes].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateEntityTypesResponse {
    /// The collection of updated or created entity types.
    #[prost(message, repeated, tag = "1")]
    pub entity_types: ::std::vec::Vec<EntityType>,
}
/// The request message for [EntityTypes.BatchDeleteEntityTypes][google.cloud.dialogflow.v2.EntityTypes.BatchDeleteEntityTypes].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteEntityTypesRequest {
    /// Required. The name of the agent to delete all entities types for. Format:
    /// `projects/<Project ID>/agent`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The names entity types to delete. All names must point to the
    /// same agent as `parent`.
    #[prost(string, repeated, tag = "2")]
    pub entity_type_names: ::std::vec::Vec<std::string::String>,
}
/// The request message for [EntityTypes.BatchCreateEntities][google.cloud.dialogflow.v2.EntityTypes.BatchCreateEntities].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchCreateEntitiesRequest {
    /// Required. The name of the entity type to create entities in. Format:
    /// `projects/<Project ID>/agent/entityTypes/<Entity Type ID>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The entities to create.
    #[prost(message, repeated, tag = "2")]
    pub entities: ::std::vec::Vec<entity_type::Entity>,
    /// Optional. The language used to access language-specific data.
    /// If not specified, the agent's default language is used.
    /// For more information, see
    /// [Multilingual intent and entity
    /// data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity).
    #[prost(string, tag = "3")]
    pub language_code: std::string::String,
}
/// The request message for [EntityTypes.BatchUpdateEntities][google.cloud.dialogflow.v2.EntityTypes.BatchUpdateEntities].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateEntitiesRequest {
    /// Required. The name of the entity type to update or create entities in.
    /// Format: `projects/<Project ID>/agent/entityTypes/<Entity Type ID>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The entities to update or create.
    #[prost(message, repeated, tag = "2")]
    pub entities: ::std::vec::Vec<entity_type::Entity>,
    /// Optional. The language used to access language-specific data.
    /// If not specified, the agent's default language is used.
    /// For more information, see
    /// [Multilingual intent and entity
    /// data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity).
    #[prost(string, tag = "3")]
    pub language_code: std::string::String,
    /// Optional. The mask to control which fields get updated.
    #[prost(message, optional, tag = "4")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// The request message for [EntityTypes.BatchDeleteEntities][google.cloud.dialogflow.v2.EntityTypes.BatchDeleteEntities].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteEntitiesRequest {
    /// Required. The name of the entity type to delete entries for. Format:
    /// `projects/<Project ID>/agent/entityTypes/<Entity Type ID>`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The reference `values` of the entities to delete. Note that
    /// these are not fully-qualified names, i.e. they don't start with
    /// `projects/<Project ID>`.
    #[prost(string, repeated, tag = "2")]
    pub entity_values: ::std::vec::Vec<std::string::String>,
    /// Optional. The language used to access language-specific data.
    /// If not specified, the agent's default language is used.
    /// For more information, see
    /// [Multilingual intent and entity
    /// data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity).
    #[prost(string, tag = "3")]
    pub language_code: std::string::String,
}
/// This message is a wrapper around a collection of entity types.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityTypeBatch {
    /// A collection of entity types.
    #[prost(message, repeated, tag = "1")]
    pub entity_types: ::std::vec::Vec<EntityType>,
}
#[doc = r" Generated client implementations."]
pub mod entity_types_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Entities are extracted from user input and represent parameters that are"]
    #[doc = " meaningful to your application. For example, a date range, a proper name"]
    #[doc = " such as a geographic location or landmark, and so on. Entities represent"]
    #[doc = " actionable data for your application."]
    #[doc = ""]
    #[doc = " When you define an entity, you can also include synonyms that all map to"]
    #[doc = " that entity. For example, \"soft drink\", \"soda\", \"pop\", and so on."]
    #[doc = ""]
    #[doc = " There are three types of entities:"]
    #[doc = ""]
    #[doc = " *   **System** - entities that are defined by the Dialogflow API for common"]
    #[doc = "     data types such as date, time, currency, and so on. A system entity is"]
    #[doc = "     represented by the `EntityType` type."]
    #[doc = ""]
    #[doc = " *   **Custom** - entities that are defined by you that represent"]
    #[doc = "     actionable data that is meaningful to your application. For example,"]
    #[doc = "     you could define a `pizza.sauce` entity for red or white pizza sauce,"]
    #[doc = "     a `pizza.cheese` entity for the different types of cheese on a pizza,"]
    #[doc = "     a `pizza.topping` entity for different toppings, and so on. A custom"]
    #[doc = "     entity is represented by the `EntityType` type."]
    #[doc = ""]
    #[doc = " *   **User** - entities that are built for an individual user such as"]
    #[doc = "     favorites, preferences, playlists, and so on. A user entity is"]
    #[doc = "     represented by the [SessionEntityType][google.cloud.dialogflow.v2.SessionEntityType] type."]
    #[doc = ""]
    #[doc = " For more information about entity types, see the"]
    #[doc = " [Dialogflow"]
    #[doc = " documentation](https://cloud.google.com/dialogflow/docs/entities-overview)."]
    pub struct EntityTypesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl EntityTypesClient<tonic::transport::Channel> {
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
    impl<T> EntityTypesClient<T>
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
        #[doc = " Returns the list of all entity types in the specified agent."]
        pub async fn list_entity_types(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEntityTypesRequest>,
        ) -> Result<tonic::Response<super::ListEntityTypesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.v2.EntityTypes/ListEntityTypes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the specified entity type."]
        pub async fn get_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::GetEntityTypeRequest>,
        ) -> Result<tonic::Response<super::EntityType>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.v2.EntityTypes/GetEntityType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an entity type in the specified agent."]
        pub async fn create_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateEntityTypeRequest>,
        ) -> Result<tonic::Response<super::EntityType>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.v2.EntityTypes/CreateEntityType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified entity type."]
        pub async fn update_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateEntityTypeRequest>,
        ) -> Result<tonic::Response<super::EntityType>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.v2.EntityTypes/UpdateEntityType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified entity type."]
        pub async fn delete_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteEntityTypeRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.v2.EntityTypes/DeleteEntityType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates/Creates multiple entity types in the specified agent."]
        #[doc = ""]
        #[doc = " Operation <response: [BatchUpdateEntityTypesResponse][google.cloud.dialogflow.v2.BatchUpdateEntityTypesResponse]>"]
        pub async fn batch_update_entity_types(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchUpdateEntityTypesRequest>,
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
                "/google.cloud.dialogflow.v2.EntityTypes/BatchUpdateEntityTypes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes entity types in the specified agent."]
        #[doc = ""]
        #[doc = " Operation <response: [google.protobuf.Empty][google.protobuf.Empty]>"]
        pub async fn batch_delete_entity_types(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchDeleteEntityTypesRequest>,
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
                "/google.cloud.dialogflow.v2.EntityTypes/BatchDeleteEntityTypes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates multiple new entities in the specified entity type."]
        #[doc = ""]
        #[doc = " Operation <response: [google.protobuf.Empty][google.protobuf.Empty]>"]
        pub async fn batch_create_entities(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchCreateEntitiesRequest>,
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
                "/google.cloud.dialogflow.v2.EntityTypes/BatchCreateEntities",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates or creates multiple entities in the specified entity type. This"]
        #[doc = " method does not affect entities in the entity type that aren't explicitly"]
        #[doc = " specified in the request."]
        #[doc = ""]
        #[doc = ""]
        #[doc = " Operation <response: [google.protobuf.Empty][google.protobuf.Empty]>"]
        pub async fn batch_update_entities(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchUpdateEntitiesRequest>,
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
                "/google.cloud.dialogflow.v2.EntityTypes/BatchUpdateEntities",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes entities in the specified entity type."]
        #[doc = ""]
        #[doc = ""]
        #[doc = " Operation <response: [google.protobuf.Empty][google.protobuf.Empty]>"]
        pub async fn batch_delete_entities(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchDeleteEntitiesRequest>,
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
                "/google.cloud.dialogflow.v2.EntityTypes/BatchDeleteEntities",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for EntityTypesClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for EntityTypesClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "EntityTypesClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod entity_types_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with EntityTypesServer."]
    #[async_trait]
    pub trait EntityTypes: Send + Sync + 'static {
        #[doc = " Returns the list of all entity types in the specified agent."]
        async fn list_entity_types(
            &self,
            request: tonic::Request<super::ListEntityTypesRequest>,
        ) -> Result<tonic::Response<super::ListEntityTypesResponse>, tonic::Status>;
        #[doc = " Retrieves the specified entity type."]
        async fn get_entity_type(
            &self,
            request: tonic::Request<super::GetEntityTypeRequest>,
        ) -> Result<tonic::Response<super::EntityType>, tonic::Status>;
        #[doc = " Creates an entity type in the specified agent."]
        async fn create_entity_type(
            &self,
            request: tonic::Request<super::CreateEntityTypeRequest>,
        ) -> Result<tonic::Response<super::EntityType>, tonic::Status>;
        #[doc = " Updates the specified entity type."]
        async fn update_entity_type(
            &self,
            request: tonic::Request<super::UpdateEntityTypeRequest>,
        ) -> Result<tonic::Response<super::EntityType>, tonic::Status>;
        #[doc = " Deletes the specified entity type."]
        async fn delete_entity_type(
            &self,
            request: tonic::Request<super::DeleteEntityTypeRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Updates/Creates multiple entity types in the specified agent."]
        #[doc = ""]
        #[doc = " Operation <response: [BatchUpdateEntityTypesResponse][google.cloud.dialogflow.v2.BatchUpdateEntityTypesResponse]>"]
        async fn batch_update_entity_types(
            &self,
            request: tonic::Request<super::BatchUpdateEntityTypesRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Deletes entity types in the specified agent."]
        #[doc = ""]
        #[doc = " Operation <response: [google.protobuf.Empty][google.protobuf.Empty]>"]
        async fn batch_delete_entity_types(
            &self,
            request: tonic::Request<super::BatchDeleteEntityTypesRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Creates multiple new entities in the specified entity type."]
        #[doc = ""]
        #[doc = " Operation <response: [google.protobuf.Empty][google.protobuf.Empty]>"]
        async fn batch_create_entities(
            &self,
            request: tonic::Request<super::BatchCreateEntitiesRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Updates or creates multiple entities in the specified entity type. This"]
        #[doc = " method does not affect entities in the entity type that aren't explicitly"]
        #[doc = " specified in the request."]
        #[doc = ""]
        #[doc = ""]
        #[doc = " Operation <response: [google.protobuf.Empty][google.protobuf.Empty]>"]
        async fn batch_update_entities(
            &self,
            request: tonic::Request<super::BatchUpdateEntitiesRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Deletes entities in the specified entity type."]
        #[doc = ""]
        #[doc = ""]
        #[doc = " Operation <response: [google.protobuf.Empty][google.protobuf.Empty]>"]
        async fn batch_delete_entities(
            &self,
            request: tonic::Request<super::BatchDeleteEntitiesRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
    }
    #[doc = " Entities are extracted from user input and represent parameters that are"]
    #[doc = " meaningful to your application. For example, a date range, a proper name"]
    #[doc = " such as a geographic location or landmark, and so on. Entities represent"]
    #[doc = " actionable data for your application."]
    #[doc = ""]
    #[doc = " When you define an entity, you can also include synonyms that all map to"]
    #[doc = " that entity. For example, \"soft drink\", \"soda\", \"pop\", and so on."]
    #[doc = ""]
    #[doc = " There are three types of entities:"]
    #[doc = ""]
    #[doc = " *   **System** - entities that are defined by the Dialogflow API for common"]
    #[doc = "     data types such as date, time, currency, and so on. A system entity is"]
    #[doc = "     represented by the `EntityType` type."]
    #[doc = ""]
    #[doc = " *   **Custom** - entities that are defined by you that represent"]
    #[doc = "     actionable data that is meaningful to your application. For example,"]
    #[doc = "     you could define a `pizza.sauce` entity for red or white pizza sauce,"]
    #[doc = "     a `pizza.cheese` entity for the different types of cheese on a pizza,"]
    #[doc = "     a `pizza.topping` entity for different toppings, and so on. A custom"]
    #[doc = "     entity is represented by the `EntityType` type."]
    #[doc = ""]
    #[doc = " *   **User** - entities that are built for an individual user such as"]
    #[doc = "     favorites, preferences, playlists, and so on. A user entity is"]
    #[doc = "     represented by the [SessionEntityType][google.cloud.dialogflow.v2.SessionEntityType] type."]
    #[doc = ""]
    #[doc = " For more information about entity types, see the"]
    #[doc = " [Dialogflow"]
    #[doc = " documentation](https://cloud.google.com/dialogflow/docs/entities-overview)."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct EntityTypesServer<T: EntityTypes> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: EntityTypes> EntityTypesServer<T> {
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
    impl<T, B> Service<http::Request<B>> for EntityTypesServer<T>
    where
        T: EntityTypes,
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
                "/google.cloud.dialogflow.v2.EntityTypes/ListEntityTypes" => {
                    #[allow(non_camel_case_types)]
                    struct ListEntityTypesSvc<T: EntityTypes>(pub Arc<T>);
                    impl<T: EntityTypes> tonic::server::UnaryService<super::ListEntityTypesRequest>
                        for ListEntityTypesSvc<T>
                    {
                        type Response = super::ListEntityTypesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListEntityTypesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_entity_types(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListEntityTypesSvc(inner);
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
                "/google.cloud.dialogflow.v2.EntityTypes/GetEntityType" => {
                    #[allow(non_camel_case_types)]
                    struct GetEntityTypeSvc<T: EntityTypes>(pub Arc<T>);
                    impl<T: EntityTypes> tonic::server::UnaryService<super::GetEntityTypeRequest>
                        for GetEntityTypeSvc<T>
                    {
                        type Response = super::EntityType;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetEntityTypeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_entity_type(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetEntityTypeSvc(inner);
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
                "/google.cloud.dialogflow.v2.EntityTypes/CreateEntityType" => {
                    #[allow(non_camel_case_types)]
                    struct CreateEntityTypeSvc<T: EntityTypes>(pub Arc<T>);
                    impl<T: EntityTypes> tonic::server::UnaryService<super::CreateEntityTypeRequest>
                        for CreateEntityTypeSvc<T>
                    {
                        type Response = super::EntityType;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateEntityTypeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_entity_type(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateEntityTypeSvc(inner);
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
                "/google.cloud.dialogflow.v2.EntityTypes/UpdateEntityType" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateEntityTypeSvc<T: EntityTypes>(pub Arc<T>);
                    impl<T: EntityTypes> tonic::server::UnaryService<super::UpdateEntityTypeRequest>
                        for UpdateEntityTypeSvc<T>
                    {
                        type Response = super::EntityType;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateEntityTypeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_entity_type(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateEntityTypeSvc(inner);
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
                "/google.cloud.dialogflow.v2.EntityTypes/DeleteEntityType" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteEntityTypeSvc<T: EntityTypes>(pub Arc<T>);
                    impl<T: EntityTypes> tonic::server::UnaryService<super::DeleteEntityTypeRequest>
                        for DeleteEntityTypeSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteEntityTypeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_entity_type(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteEntityTypeSvc(inner);
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
                "/google.cloud.dialogflow.v2.EntityTypes/BatchUpdateEntityTypes" => {
                    #[allow(non_camel_case_types)]
                    struct BatchUpdateEntityTypesSvc<T: EntityTypes>(pub Arc<T>);
                    impl<T: EntityTypes>
                        tonic::server::UnaryService<super::BatchUpdateEntityTypesRequest>
                        for BatchUpdateEntityTypesSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchUpdateEntityTypesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.batch_update_entity_types(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = BatchUpdateEntityTypesSvc(inner);
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
                "/google.cloud.dialogflow.v2.EntityTypes/BatchDeleteEntityTypes" => {
                    #[allow(non_camel_case_types)]
                    struct BatchDeleteEntityTypesSvc<T: EntityTypes>(pub Arc<T>);
                    impl<T: EntityTypes>
                        tonic::server::UnaryService<super::BatchDeleteEntityTypesRequest>
                        for BatchDeleteEntityTypesSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchDeleteEntityTypesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.batch_delete_entity_types(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = BatchDeleteEntityTypesSvc(inner);
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
                "/google.cloud.dialogflow.v2.EntityTypes/BatchCreateEntities" => {
                    #[allow(non_camel_case_types)]
                    struct BatchCreateEntitiesSvc<T: EntityTypes>(pub Arc<T>);
                    impl<T: EntityTypes>
                        tonic::server::UnaryService<super::BatchCreateEntitiesRequest>
                        for BatchCreateEntitiesSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchCreateEntitiesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.batch_create_entities(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = BatchCreateEntitiesSvc(inner);
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
                "/google.cloud.dialogflow.v2.EntityTypes/BatchUpdateEntities" => {
                    #[allow(non_camel_case_types)]
                    struct BatchUpdateEntitiesSvc<T: EntityTypes>(pub Arc<T>);
                    impl<T: EntityTypes>
                        tonic::server::UnaryService<super::BatchUpdateEntitiesRequest>
                        for BatchUpdateEntitiesSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchUpdateEntitiesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.batch_update_entities(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = BatchUpdateEntitiesSvc(inner);
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
                "/google.cloud.dialogflow.v2.EntityTypes/BatchDeleteEntities" => {
                    #[allow(non_camel_case_types)]
                    struct BatchDeleteEntitiesSvc<T: EntityTypes>(pub Arc<T>);
                    impl<T: EntityTypes>
                        tonic::server::UnaryService<super::BatchDeleteEntitiesRequest>
                        for BatchDeleteEntitiesSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchDeleteEntitiesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.batch_delete_entities(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = BatchDeleteEntitiesSvc(inner);
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
    impl<T: EntityTypes> Clone for EntityTypesServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: EntityTypes> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: EntityTypes> tonic::transport::NamedService for EntityTypesServer<T> {
        const NAME: &'static str = "google.cloud.dialogflow.v2.EntityTypes";
    }
}
/// Represents an agent environment.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Environment {
    /// Output only. The unique identifier of this agent environment.
    /// Format: `projects/<Project ID>/agent/environments/<Environment ID>`.
    /// For Environment ID, "-" is reserved for 'draft' environment.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Optional. The developer-provided description for this environment.
    /// The maximum length is 500 characters. If exceeded, the request is rejected.
    #[prost(string, tag = "2")]
    pub description: std::string::String,
    /// Optional. The agent version loaded into this environment.
    /// Format: `projects/<Project ID>/agent/versions/<Version ID>`.
    #[prost(string, tag = "3")]
    pub agent_version: std::string::String,
    /// Output only. The state of this environment. This field is read-only, i.e., it cannot be
    /// set by create and update methods.
    #[prost(enumeration = "environment::State", tag = "4")]
    pub state: i32,
    /// Output only. The last update time of this environment. This field is read-only, i.e., it
    /// cannot be set by create and update methods.
    #[prost(message, optional, tag = "5")]
    pub update_time: ::std::option::Option<::prost_types::Timestamp>,
}
pub mod environment {
    /// Represents an environment state. When a environment is pointed to a new
    /// agent version, the environment is temporarily set to the `LOADING` state.
    /// During that time, the environment keeps on serving the previous version of
    /// the agent. After the new agent version is done loading, the environment is
    /// set back to the `RUNNING` state.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum State {
        /// Not specified. This value is not used.
        Unspecified = 0,
        /// Stopped.
        Stopped = 1,
        /// Loading.
        Loading = 2,
        /// Running.
        Running = 3,
    }
}
/// The request message for [Environments.ListEnvironments][google.cloud.dialogflow.v2.Environments.ListEnvironments].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEnvironmentsRequest {
    /// Required. The agent to list all environments from.
    /// Format: `projects/<Project ID>/agent`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The maximum number of items to return in a single page. By default 100 and
    /// at most 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// The response message for [Environments.ListEnvironments][google.cloud.dialogflow.v2.Environments.ListEnvironments].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListEnvironmentsResponse {
    /// The list of agent environments. There will be a maximum number of items
    /// returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub environments: ::std::vec::Vec<Environment>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod environments_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Manages agent environments."]
    pub struct EnvironmentsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl EnvironmentsClient<tonic::transport::Channel> {
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
    impl<T> EnvironmentsClient<T>
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
        #[doc = " Returns the list of all non-draft environments of the specified agent."]
        pub async fn list_environments(
            &mut self,
            request: impl tonic::IntoRequest<super::ListEnvironmentsRequest>,
        ) -> Result<tonic::Response<super::ListEnvironmentsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.v2.Environments/ListEnvironments",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for EnvironmentsClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for EnvironmentsClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "EnvironmentsClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod environments_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with EnvironmentsServer."]
    #[async_trait]
    pub trait Environments: Send + Sync + 'static {
        #[doc = " Returns the list of all non-draft environments of the specified agent."]
        async fn list_environments(
            &self,
            request: tonic::Request<super::ListEnvironmentsRequest>,
        ) -> Result<tonic::Response<super::ListEnvironmentsResponse>, tonic::Status>;
    }
    #[doc = " Manages agent environments."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct EnvironmentsServer<T: Environments> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Environments> EnvironmentsServer<T> {
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
    impl<T, B> Service<http::Request<B>> for EnvironmentsServer<T>
    where
        T: Environments,
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
                "/google.cloud.dialogflow.v2.Environments/ListEnvironments" => {
                    #[allow(non_camel_case_types)]
                    struct ListEnvironmentsSvc<T: Environments>(pub Arc<T>);
                    impl<T: Environments>
                        tonic::server::UnaryService<super::ListEnvironmentsRequest>
                        for ListEnvironmentsSvc<T>
                    {
                        type Response = super::ListEnvironmentsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListEnvironmentsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_environments(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListEnvironmentsSvc(inner);
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
    impl<T: Environments> Clone for EnvironmentsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Environments> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Environments> tonic::transport::NamedService for EnvironmentsServer<T> {
        const NAME: &'static str = "google.cloud.dialogflow.v2.Environments";
    }
}
/// Represents an intent.
/// Intents convert a number of user expressions or patterns into an action. An
/// action is an extraction of a user command or sentence semantics.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Intent {
    /// Optional. The unique identifier of this intent.
    /// Required for [Intents.UpdateIntent][google.cloud.dialogflow.v2.Intents.UpdateIntent] and [Intents.BatchUpdateIntents][google.cloud.dialogflow.v2.Intents.BatchUpdateIntents]
    /// methods.
    /// Format: `projects/<Project ID>/agent/intents/<Intent ID>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. The name of this intent.
    #[prost(string, tag = "2")]
    pub display_name: std::string::String,
    /// Optional. Indicates whether webhooks are enabled for the intent.
    #[prost(enumeration = "intent::WebhookState", tag = "6")]
    pub webhook_state: i32,
    /// Optional. The priority of this intent. Higher numbers represent higher
    /// priorities.
    ///
    /// - If the supplied value is unspecified or 0, the service
    ///   translates the value to 500,000, which corresponds to the
    ///   `Normal` priority in the console.
    /// - If the supplied value is negative, the intent is ignored
    ///   in runtime detect intent requests.
    #[prost(int32, tag = "3")]
    pub priority: i32,
    /// Optional. Indicates whether this is a fallback intent.
    #[prost(bool, tag = "4")]
    pub is_fallback: bool,
    /// Optional. Indicates whether Machine Learning is disabled for the intent.
    /// Note: If `ml_disabled` setting is set to true, then this intent is not
    /// taken into account during inference in `ML ONLY` match mode. Also,
    /// auto-markup in the UI is turned off.
    #[prost(bool, tag = "19")]
    pub ml_disabled: bool,
    /// Optional. The list of context names required for this intent to be
    /// triggered.
    /// Format: `projects/<Project ID>/agent/sessions/-/contexts/<Context ID>`.
    #[prost(string, repeated, tag = "7")]
    pub input_context_names: ::std::vec::Vec<std::string::String>,
    /// Optional. The collection of event names that trigger the intent.
    /// If the collection of input contexts is not empty, all of the contexts must
    /// be present in the active user session for an event to trigger this intent.
    /// Event names are limited to 150 characters.
    #[prost(string, repeated, tag = "8")]
    pub events: ::std::vec::Vec<std::string::String>,
    /// Optional. The collection of examples that the agent is
    /// trained on.
    #[prost(message, repeated, tag = "9")]
    pub training_phrases: ::std::vec::Vec<intent::TrainingPhrase>,
    /// Optional. The name of the action associated with the intent.
    /// Note: The action name must not contain whitespaces.
    #[prost(string, tag = "10")]
    pub action: std::string::String,
    /// Optional. The collection of contexts that are activated when the intent
    /// is matched. Context messages in this collection should not set the
    /// parameters field. Setting the `lifespan_count` to 0 will reset the context
    /// when the intent is matched.
    /// Format: `projects/<Project ID>/agent/sessions/-/contexts/<Context ID>`.
    #[prost(message, repeated, tag = "11")]
    pub output_contexts: ::std::vec::Vec<Context>,
    /// Optional. Indicates whether to delete all contexts in the current
    /// session when this intent is matched.
    #[prost(bool, tag = "12")]
    pub reset_contexts: bool,
    /// Optional. The collection of parameters associated with the intent.
    #[prost(message, repeated, tag = "13")]
    pub parameters: ::std::vec::Vec<intent::Parameter>,
    /// Optional. The collection of rich messages corresponding to the
    /// `Response` field in the Dialogflow console.
    #[prost(message, repeated, tag = "14")]
    pub messages: ::std::vec::Vec<intent::Message>,
    /// Optional. The list of platforms for which the first responses will be
    /// copied from the messages in PLATFORM_UNSPECIFIED (i.e. default platform).
    #[prost(
        enumeration = "intent::message::Platform",
        repeated,
        packed = "false",
        tag = "15"
    )]
    pub default_response_platforms: ::std::vec::Vec<i32>,
    /// Read-only. The unique identifier of the root intent in the chain of
    /// followup intents. It identifies the correct followup intents chain for
    /// this intent. We populate this field only in the output.
    ///
    /// Format: `projects/<Project ID>/agent/intents/<Intent ID>`.
    #[prost(string, tag = "16")]
    pub root_followup_intent_name: std::string::String,
    /// Read-only after creation. The unique identifier of the parent intent in the
    /// chain of followup intents. You can set this field when creating an intent,
    /// for example with [CreateIntent][google.cloud.dialogflow.v2.Intents.CreateIntent] or
    /// [BatchUpdateIntents][google.cloud.dialogflow.v2.Intents.BatchUpdateIntents], in order to make this
    /// intent a followup intent.
    ///
    /// It identifies the parent followup intent.
    /// Format: `projects/<Project ID>/agent/intents/<Intent ID>`.
    #[prost(string, tag = "17")]
    pub parent_followup_intent_name: std::string::String,
    /// Read-only. Information about all followup intents that have this intent as
    /// a direct or indirect parent. We populate this field only in the output.
    #[prost(message, repeated, tag = "18")]
    pub followup_intent_info: ::std::vec::Vec<intent::FollowupIntentInfo>,
}
pub mod intent {
    /// Represents an example that the agent is trained on.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TrainingPhrase {
        /// Output only. The unique identifier of this training phrase.
        #[prost(string, tag = "1")]
        pub name: std::string::String,
        /// Required. The type of the training phrase.
        #[prost(enumeration = "training_phrase::Type", tag = "2")]
        pub r#type: i32,
        /// Required. The ordered list of training phrase parts.
        /// The parts are concatenated in order to form the training phrase.
        ///
        /// Note: The API does not automatically annotate training phrases like the
        /// Dialogflow Console does.
        ///
        /// Note: Do not forget to include whitespace at part boundaries,
        /// so the training phrase is well formatted when the parts are concatenated.
        ///
        /// If the training phrase does not need to be annotated with parameters,
        /// you just need a single part with only the [Part.text][google.cloud.dialogflow.v2.Intent.TrainingPhrase.Part.text] field set.
        ///
        /// If you want to annotate the training phrase, you must create multiple
        /// parts, where the fields of each part are populated in one of two ways:
        ///
        /// -   `Part.text` is set to a part of the phrase that has no parameters.
        /// -   `Part.text` is set to a part of the phrase that you want to annotate,
        ///     and the `entity_type`, `alias`, and `user_defined` fields are all
        ///     set.
        #[prost(message, repeated, tag = "3")]
        pub parts: ::std::vec::Vec<training_phrase::Part>,
        /// Optional. Indicates how many times this example was added to
        /// the intent. Each time a developer adds an existing sample by editing an
        /// intent or training, this counter is increased.
        #[prost(int32, tag = "4")]
        pub times_added_count: i32,
    }
    pub mod training_phrase {
        /// Represents a part of a training phrase.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Part {
            /// Required. The text for this part.
            #[prost(string, tag = "1")]
            pub text: std::string::String,
            /// Optional. The entity type name prefixed with `@`.
            /// This field is required for annotated parts of the training phrase.
            #[prost(string, tag = "2")]
            pub entity_type: std::string::String,
            /// Optional. The parameter name for the value extracted from the
            /// annotated part of the example.
            /// This field is required for annotated parts of the training phrase.
            #[prost(string, tag = "3")]
            pub alias: std::string::String,
            /// Optional. Indicates whether the text was manually annotated.
            /// This field is set to true when the Dialogflow Console is used to
            /// manually annotate the part. When creating an annotated part with the
            /// API, you must set this to true.
            #[prost(bool, tag = "4")]
            pub user_defined: bool,
        }
        /// Represents different types of training phrases.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Type {
            /// Not specified. This value should never be used.
            Unspecified = 0,
            /// Examples do not contain @-prefixed entity type names, but example parts
            /// can be annotated with entity types.
            Example = 1,
            /// Templates are not annotated with entity types, but they can contain
            /// @-prefixed entity type names as substrings.
            /// Template mode has been deprecated. Example mode is the only supported
            /// way to create new training phrases. If you have existing training
            /// phrases that you've created in template mode, those will continue to
            /// work.
            Template = 2,
        }
    }
    /// Represents intent parameters.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Parameter {
        /// The unique identifier of this parameter.
        #[prost(string, tag = "1")]
        pub name: std::string::String,
        /// Required. The name of the parameter.
        #[prost(string, tag = "2")]
        pub display_name: std::string::String,
        /// Optional. The definition of the parameter value. It can be:
        /// - a constant string,
        /// - a parameter value defined as `$parameter_name`,
        /// - an original parameter value defined as `$parameter_name.original`,
        /// - a parameter value from some context defined as
        ///   `#context_name.parameter_name`.
        #[prost(string, tag = "3")]
        pub value: std::string::String,
        /// Optional. The default value to use when the `value` yields an empty
        /// result.
        /// Default values can be extracted from contexts by using the following
        /// syntax: `#context_name.parameter_name`.
        #[prost(string, tag = "4")]
        pub default_value: std::string::String,
        /// Optional. The name of the entity type, prefixed with `@`, that
        /// describes values of the parameter. If the parameter is
        /// required, this must be provided.
        #[prost(string, tag = "5")]
        pub entity_type_display_name: std::string::String,
        /// Optional. Indicates whether the parameter is required. That is,
        /// whether the intent cannot be completed without collecting the parameter
        /// value.
        #[prost(bool, tag = "6")]
        pub mandatory: bool,
        /// Optional. The collection of prompts that the agent can present to the
        /// user in order to collect a value for the parameter.
        #[prost(string, repeated, tag = "7")]
        pub prompts: ::std::vec::Vec<std::string::String>,
        /// Optional. Indicates whether the parameter represents a list of values.
        #[prost(bool, tag = "8")]
        pub is_list: bool,
    }
    /// A rich response message.
    /// Corresponds to the intent `Response` field in the Dialogflow console.
    /// For more information, see
    /// [Rich response
    /// messages](https://cloud.google.com/dialogflow/docs/intents-rich-messages).
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Message {
        /// Optional. The platform that this message is intended for.
        #[prost(enumeration = "message::Platform", tag = "6")]
        pub platform: i32,
        /// Required. The rich response message.
        #[prost(
            oneof = "message::Message",
            tags = "1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 22, 23, 24"
        )]
        pub message: ::std::option::Option<message::Message>,
    }
    pub mod message {
        /// The text response message.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Text {
            /// Optional. The collection of the agent's responses.
            #[prost(string, repeated, tag = "1")]
            pub text: ::std::vec::Vec<std::string::String>,
        }
        /// The image response message.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Image {
            /// Optional. The public URI to an image file.
            #[prost(string, tag = "1")]
            pub image_uri: std::string::String,
            /// Optional. A text description of the image to be used for accessibility,
            /// e.g., screen readers.
            #[prost(string, tag = "2")]
            pub accessibility_text: std::string::String,
        }
        /// The quick replies response message.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct QuickReplies {
            /// Optional. The title of the collection of quick replies.
            #[prost(string, tag = "1")]
            pub title: std::string::String,
            /// Optional. The collection of quick replies.
            #[prost(string, repeated, tag = "2")]
            pub quick_replies: ::std::vec::Vec<std::string::String>,
        }
        /// The card response message.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Card {
            /// Optional. The title of the card.
            #[prost(string, tag = "1")]
            pub title: std::string::String,
            /// Optional. The subtitle of the card.
            #[prost(string, tag = "2")]
            pub subtitle: std::string::String,
            /// Optional. The public URI to an image file for the card.
            #[prost(string, tag = "3")]
            pub image_uri: std::string::String,
            /// Optional. The collection of card buttons.
            #[prost(message, repeated, tag = "4")]
            pub buttons: ::std::vec::Vec<card::Button>,
        }
        pub mod card {
            /// Contains information about a button.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Button {
                /// Optional. The text to show on the button.
                #[prost(string, tag = "1")]
                pub text: std::string::String,
                /// Optional. The text to send back to the Dialogflow API or a URI to
                /// open.
                #[prost(string, tag = "2")]
                pub postback: std::string::String,
            }
        }
        /// The simple response message containing speech or text.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SimpleResponse {
            /// One of text_to_speech or ssml must be provided. The plain text of the
            /// speech output. Mutually exclusive with ssml.
            #[prost(string, tag = "1")]
            pub text_to_speech: std::string::String,
            /// One of text_to_speech or ssml must be provided. Structured spoken
            /// response to the user in the SSML format. Mutually exclusive with
            /// text_to_speech.
            #[prost(string, tag = "2")]
            pub ssml: std::string::String,
            /// Optional. The text to display.
            #[prost(string, tag = "3")]
            pub display_text: std::string::String,
        }
        /// The collection of simple response candidates.
        /// This message in `QueryResult.fulfillment_messages` and
        /// `WebhookResponse.fulfillment_messages` should contain only one
        /// `SimpleResponse`.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SimpleResponses {
            /// Required. The list of simple responses.
            #[prost(message, repeated, tag = "1")]
            pub simple_responses: ::std::vec::Vec<SimpleResponse>,
        }
        /// The basic card message. Useful for displaying information.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct BasicCard {
            /// Optional. The title of the card.
            #[prost(string, tag = "1")]
            pub title: std::string::String,
            /// Optional. The subtitle of the card.
            #[prost(string, tag = "2")]
            pub subtitle: std::string::String,
            /// Required, unless image is present. The body text of the card.
            #[prost(string, tag = "3")]
            pub formatted_text: std::string::String,
            /// Optional. The image for the card.
            #[prost(message, optional, tag = "4")]
            pub image: ::std::option::Option<Image>,
            /// Optional. The collection of card buttons.
            #[prost(message, repeated, tag = "5")]
            pub buttons: ::std::vec::Vec<basic_card::Button>,
        }
        pub mod basic_card {
            /// The button object that appears at the bottom of a card.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Button {
                /// Required. The title of the button.
                #[prost(string, tag = "1")]
                pub title: std::string::String,
                /// Required. Action to take when a user taps on the button.
                #[prost(message, optional, tag = "2")]
                pub open_uri_action: ::std::option::Option<button::OpenUriAction>,
            }
            pub mod button {
                /// Opens the given URI.
                #[derive(Clone, PartialEq, ::prost::Message)]
                pub struct OpenUriAction {
                    /// Required. The HTTP or HTTPS scheme URI.
                    #[prost(string, tag = "1")]
                    pub uri: std::string::String,
                }
            }
        }
        /// The suggestion chip message that the user can tap to quickly post a reply
        /// to the conversation.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Suggestion {
            /// Required. The text shown the in the suggestion chip.
            #[prost(string, tag = "1")]
            pub title: std::string::String,
        }
        /// The collection of suggestions.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct Suggestions {
            /// Required. The list of suggested replies.
            #[prost(message, repeated, tag = "1")]
            pub suggestions: ::std::vec::Vec<Suggestion>,
        }
        /// The suggestion chip message that allows the user to jump out to the app
        /// or website associated with this agent.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct LinkOutSuggestion {
            /// Required. The name of the app or site this chip is linking to.
            #[prost(string, tag = "1")]
            pub destination_name: std::string::String,
            /// Required. The URI of the app or site to open when the user taps the
            /// suggestion chip.
            #[prost(string, tag = "2")]
            pub uri: std::string::String,
        }
        /// The card for presenting a list of options to select from.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ListSelect {
            /// Optional. The overall title of the list.
            #[prost(string, tag = "1")]
            pub title: std::string::String,
            /// Required. List items.
            #[prost(message, repeated, tag = "2")]
            pub items: ::std::vec::Vec<list_select::Item>,
            /// Optional. Subtitle of the list.
            #[prost(string, tag = "3")]
            pub subtitle: std::string::String,
        }
        pub mod list_select {
            /// An item in the list.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Item {
                /// Required. Additional information about this option.
                #[prost(message, optional, tag = "1")]
                pub info: ::std::option::Option<super::SelectItemInfo>,
                /// Required. The title of the list item.
                #[prost(string, tag = "2")]
                pub title: std::string::String,
                /// Optional. The main text describing the item.
                #[prost(string, tag = "3")]
                pub description: std::string::String,
                /// Optional. The image to display.
                #[prost(message, optional, tag = "4")]
                pub image: ::std::option::Option<super::Image>,
            }
        }
        /// The card for presenting a carousel of options to select from.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CarouselSelect {
            /// Required. Carousel items.
            #[prost(message, repeated, tag = "1")]
            pub items: ::std::vec::Vec<carousel_select::Item>,
        }
        pub mod carousel_select {
            /// An item in the carousel.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct Item {
                /// Required. Additional info about the option item.
                #[prost(message, optional, tag = "1")]
                pub info: ::std::option::Option<super::SelectItemInfo>,
                /// Required. Title of the carousel item.
                #[prost(string, tag = "2")]
                pub title: std::string::String,
                /// Optional. The body text of the card.
                #[prost(string, tag = "3")]
                pub description: std::string::String,
                /// Optional. The image to display.
                #[prost(message, optional, tag = "4")]
                pub image: ::std::option::Option<super::Image>,
            }
        }
        /// Additional info about the select item for when it is triggered in a
        /// dialog.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SelectItemInfo {
            /// Required. A unique key that will be sent back to the agent if this
            /// response is given.
            #[prost(string, tag = "1")]
            pub key: std::string::String,
            /// Optional. A list of synonyms that can also be used to trigger this
            /// item in dialog.
            #[prost(string, repeated, tag = "2")]
            pub synonyms: ::std::vec::Vec<std::string::String>,
        }
        /// The media content card for Actions on Google.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct MediaContent {
            /// Optional. What type of media is the content (ie "audio").
            #[prost(enumeration = "media_content::ResponseMediaType", tag = "1")]
            pub media_type: i32,
            /// Required. List of media objects.
            #[prost(message, repeated, tag = "2")]
            pub media_objects: ::std::vec::Vec<media_content::ResponseMediaObject>,
        }
        pub mod media_content {
            /// Response media object for media content card.
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct ResponseMediaObject {
                /// Required. Name of media card.
                #[prost(string, tag = "1")]
                pub name: std::string::String,
                /// Optional. Description of media card.
                #[prost(string, tag = "2")]
                pub description: std::string::String,
                /// Required. Url where the media is stored.
                #[prost(string, tag = "5")]
                pub content_url: std::string::String,
                /// Image to show with the media card.
                #[prost(oneof = "response_media_object::Image", tags = "3, 4")]
                pub image: ::std::option::Option<response_media_object::Image>,
            }
            pub mod response_media_object {
                /// Image to show with the media card.
                #[derive(Clone, PartialEq, ::prost::Oneof)]
                pub enum Image {
                    /// Optional. Image to display above media content.
                    #[prost(message, tag = "3")]
                    LargeImage(super::super::Image),
                    /// Optional. Icon to display above media content.
                    #[prost(message, tag = "4")]
                    Icon(super::super::Image),
                }
            }
            /// Format of response media type.
            #[derive(
                Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
            )]
            #[repr(i32)]
            pub enum ResponseMediaType {
                /// Unspecified.
                Unspecified = 0,
                /// Response media type is audio.
                Audio = 1,
            }
        }
        /// Browse Carousel Card for Actions on Google.
        /// https://developers.google.com/actions/assistant/responses#browsing_carousel
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct BrowseCarouselCard {
            /// Required. List of items in the Browse Carousel Card. Minimum of two
            /// items, maximum of ten.
            #[prost(message, repeated, tag = "1")]
            pub items: ::std::vec::Vec<browse_carousel_card::BrowseCarouselCardItem>,
            /// Optional. Settings for displaying the image. Applies to every image in
            /// [items][google.cloud.dialogflow.v2.Intent.Message.BrowseCarouselCard.items].
            #[prost(enumeration = "browse_carousel_card::ImageDisplayOptions", tag = "2")]
            pub image_display_options: i32,
        }
        pub mod browse_carousel_card {
            /// Browsing carousel tile
            #[derive(Clone, PartialEq, ::prost::Message)]
            pub struct BrowseCarouselCardItem {
                /// Required. Action to present to the user.
                #[prost(message, optional, tag = "1")]
                pub open_uri_action:
                    ::std::option::Option<browse_carousel_card_item::OpenUrlAction>,
                /// Required. Title of the carousel item. Maximum of two lines of text.
                #[prost(string, tag = "2")]
                pub title: std::string::String,
                /// Optional. Description of the carousel item. Maximum of four lines of
                /// text.
                #[prost(string, tag = "3")]
                pub description: std::string::String,
                /// Optional. Hero image for the carousel item.
                #[prost(message, optional, tag = "4")]
                pub image: ::std::option::Option<super::Image>,
                /// Optional. Text that appears at the bottom of the Browse Carousel
                /// Card. Maximum of one line of text.
                #[prost(string, tag = "5")]
                pub footer: std::string::String,
            }
            pub mod browse_carousel_card_item {
                /// Actions on Google action to open a given url.
                #[derive(Clone, PartialEq, ::prost::Message)]
                pub struct OpenUrlAction {
                    /// Required. URL
                    #[prost(string, tag = "1")]
                    pub url: std::string::String,
                    /// Optional. Specifies the type of viewer that is used when opening
                    /// the URL. Defaults to opening via web browser.
                    #[prost(enumeration = "open_url_action::UrlTypeHint", tag = "3")]
                    pub url_type_hint: i32,
                }
                pub mod open_url_action {
                    /// Type of the URI.
                    #[derive(
                        Clone,
                        Copy,
                        Debug,
                        PartialEq,
                        Eq,
                        Hash,
                        PartialOrd,
                        Ord,
                        ::prost::Enumeration,
                    )]
                    #[repr(i32)]
                    pub enum UrlTypeHint {
                        /// Unspecified
                        Unspecified = 0,
                        /// Url would be an amp action
                        AmpAction = 1,
                        /// URL that points directly to AMP content, or to a canonical URL
                        /// which refers to AMP content via <link rel="amphtml">.
                        AmpContent = 2,
                    }
                }
            }
            /// Image display options for Actions on Google. This should be used for
            /// when the image's aspect ratio does not match the image container's
            /// aspect ratio.
            #[derive(
                Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
            )]
            #[repr(i32)]
            pub enum ImageDisplayOptions {
                /// Fill the gaps between the image and the image container with gray
                /// bars.
                Unspecified = 0,
                /// Fill the gaps between the image and the image container with gray
                /// bars.
                Gray = 1,
                /// Fill the gaps between the image and the image container with white
                /// bars.
                White = 2,
                /// Image is scaled such that the image width and height match or exceed
                /// the container dimensions. This may crop the top and bottom of the
                /// image if the scaled image height is greater than the container
                /// height, or crop the left and right of the image if the scaled image
                /// width is greater than the container width. This is similar to "Zoom
                /// Mode" on a widescreen TV when playing a 4:3 video.
                Cropped = 3,
                /// Pad the gaps between image and image frame with a blurred copy of the
                /// same image.
                BlurredBackground = 4,
            }
        }
        /// Table card for Actions on Google.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TableCard {
            /// Required. Title of the card.
            #[prost(string, tag = "1")]
            pub title: std::string::String,
            /// Optional. Subtitle to the title.
            #[prost(string, tag = "2")]
            pub subtitle: std::string::String,
            /// Optional. Image which should be displayed on the card.
            #[prost(message, optional, tag = "3")]
            pub image: ::std::option::Option<Image>,
            /// Optional. Display properties for the columns in this table.
            #[prost(message, repeated, tag = "4")]
            pub column_properties: ::std::vec::Vec<ColumnProperties>,
            /// Optional. Rows in this table of data.
            #[prost(message, repeated, tag = "5")]
            pub rows: ::std::vec::Vec<TableCardRow>,
            /// Optional. List of buttons for the card.
            #[prost(message, repeated, tag = "6")]
            pub buttons: ::std::vec::Vec<basic_card::Button>,
        }
        /// Column properties for [TableCard][google.cloud.dialogflow.v2.Intent.Message.TableCard].
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ColumnProperties {
            /// Required. Column heading.
            #[prost(string, tag = "1")]
            pub header: std::string::String,
            /// Optional. Defines text alignment for all cells in this column.
            #[prost(enumeration = "column_properties::HorizontalAlignment", tag = "2")]
            pub horizontal_alignment: i32,
        }
        pub mod column_properties {
            /// Text alignments within a cell.
            #[derive(
                Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
            )]
            #[repr(i32)]
            pub enum HorizontalAlignment {
                /// Text is aligned to the leading edge of the column.
                Unspecified = 0,
                /// Text is aligned to the leading edge of the column.
                Leading = 1,
                /// Text is centered in the column.
                Center = 2,
                /// Text is aligned to the trailing edge of the column.
                Trailing = 3,
            }
        }
        /// Row of [TableCard][google.cloud.dialogflow.v2.Intent.Message.TableCard].
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TableCardRow {
            /// Optional. List of cells that make up this row.
            #[prost(message, repeated, tag = "1")]
            pub cells: ::std::vec::Vec<TableCardCell>,
            /// Optional. Whether to add a visual divider after this row.
            #[prost(bool, tag = "2")]
            pub divider_after: bool,
        }
        /// Cell of [TableCardRow][google.cloud.dialogflow.v2.Intent.Message.TableCardRow].
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct TableCardCell {
            /// Required. Text in this cell.
            #[prost(string, tag = "1")]
            pub text: std::string::String,
        }
        /// The rich response message integration platform. See
        /// [Integrations](https://cloud.google.com/dialogflow/docs/integrations).
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum Platform {
            /// Default platform.
            Unspecified = 0,
            /// Facebook.
            Facebook = 1,
            /// Slack.
            Slack = 2,
            /// Telegram.
            Telegram = 3,
            /// Kik.
            Kik = 4,
            /// Skype.
            Skype = 5,
            /// Line.
            Line = 6,
            /// Viber.
            Viber = 7,
            /// Google Assistant
            /// See [Dialogflow webhook
            /// format](https://developers.google.com/assistant/actions/build/json/dialogflow-webhook-json)
            ActionsOnGoogle = 8,
            /// Google Hangouts.
            GoogleHangouts = 11,
        }
        /// Required. The rich response message.
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Message {
            /// The text response.
            #[prost(message, tag = "1")]
            Text(Text),
            /// The image response.
            #[prost(message, tag = "2")]
            Image(Image),
            /// The quick replies response.
            #[prost(message, tag = "3")]
            QuickReplies(QuickReplies),
            /// The card response.
            #[prost(message, tag = "4")]
            Card(Card),
            /// A custom platform-specific response.
            #[prost(message, tag = "5")]
            Payload(::prost_types::Struct),
            /// The voice and text-only responses for Actions on Google.
            #[prost(message, tag = "7")]
            SimpleResponses(SimpleResponses),
            /// The basic card response for Actions on Google.
            #[prost(message, tag = "8")]
            BasicCard(BasicCard),
            /// The suggestion chips for Actions on Google.
            #[prost(message, tag = "9")]
            Suggestions(Suggestions),
            /// The link out suggestion chip for Actions on Google.
            #[prost(message, tag = "10")]
            LinkOutSuggestion(LinkOutSuggestion),
            /// The list card response for Actions on Google.
            #[prost(message, tag = "11")]
            ListSelect(ListSelect),
            /// The carousel card response for Actions on Google.
            #[prost(message, tag = "12")]
            CarouselSelect(CarouselSelect),
            /// Browse carousel card for Actions on Google.
            #[prost(message, tag = "22")]
            BrowseCarouselCard(BrowseCarouselCard),
            /// Table card for Actions on Google.
            #[prost(message, tag = "23")]
            TableCard(TableCard),
            /// The media content card for Actions on Google.
            #[prost(message, tag = "24")]
            MediaContent(MediaContent),
        }
    }
    /// Represents a single followup intent in the chain.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct FollowupIntentInfo {
        /// The unique identifier of the followup intent.
        /// Format: `projects/<Project ID>/agent/intents/<Intent ID>`.
        #[prost(string, tag = "1")]
        pub followup_intent_name: std::string::String,
        /// The unique identifier of the followup intent's parent.
        /// Format: `projects/<Project ID>/agent/intents/<Intent ID>`.
        #[prost(string, tag = "2")]
        pub parent_followup_intent_name: std::string::String,
    }
    /// Represents the different states that webhooks can be in.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum WebhookState {
        /// Webhook is disabled in the agent and in the intent.
        Unspecified = 0,
        /// Webhook is enabled in the agent and in the intent.
        Enabled = 1,
        /// Webhook is enabled in the agent and in the intent. Also, each slot
        /// filling prompt is forwarded to the webhook.
        EnabledForSlotFilling = 2,
    }
}
/// The request message for [Intents.ListIntents][google.cloud.dialogflow.v2.Intents.ListIntents].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIntentsRequest {
    /// Required. The agent to list all intents from.
    /// Format: `projects/<Project ID>/agent`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The language used to access language-specific data.
    /// If not specified, the agent's default language is used.
    /// For more information, see
    /// [Multilingual intent and entity
    /// data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity).
    #[prost(string, tag = "2")]
    pub language_code: std::string::String,
    /// Optional. The resource view to apply to the returned intent.
    #[prost(enumeration = "IntentView", tag = "3")]
    pub intent_view: i32,
    /// Optional. The maximum number of items to return in a single page. By
    /// default 100 and at most 1000.
    #[prost(int32, tag = "4")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous list request.
    #[prost(string, tag = "5")]
    pub page_token: std::string::String,
}
/// The response message for [Intents.ListIntents][google.cloud.dialogflow.v2.Intents.ListIntents].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListIntentsResponse {
    /// The list of agent intents. There will be a maximum number of items
    /// returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub intents: ::std::vec::Vec<Intent>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The request message for [Intents.GetIntent][google.cloud.dialogflow.v2.Intents.GetIntent].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetIntentRequest {
    /// Required. The name of the intent.
    /// Format: `projects/<Project ID>/agent/intents/<Intent ID>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Optional. The language used to access language-specific data.
    /// If not specified, the agent's default language is used.
    /// For more information, see
    /// [Multilingual intent and entity
    /// data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity).
    #[prost(string, tag = "2")]
    pub language_code: std::string::String,
    /// Optional. The resource view to apply to the returned intent.
    #[prost(enumeration = "IntentView", tag = "3")]
    pub intent_view: i32,
}
/// The request message for [Intents.CreateIntent][google.cloud.dialogflow.v2.Intents.CreateIntent].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateIntentRequest {
    /// Required. The agent to create a intent for.
    /// Format: `projects/<Project ID>/agent`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The intent to create.
    #[prost(message, optional, tag = "2")]
    pub intent: ::std::option::Option<Intent>,
    /// Optional. The language used to access language-specific data.
    /// If not specified, the agent's default language is used.
    /// For more information, see
    /// [Multilingual intent and entity
    /// data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity).
    #[prost(string, tag = "3")]
    pub language_code: std::string::String,
    /// Optional. The resource view to apply to the returned intent.
    #[prost(enumeration = "IntentView", tag = "4")]
    pub intent_view: i32,
}
/// The request message for [Intents.UpdateIntent][google.cloud.dialogflow.v2.Intents.UpdateIntent].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateIntentRequest {
    /// Required. The intent to update.
    #[prost(message, optional, tag = "1")]
    pub intent: ::std::option::Option<Intent>,
    /// Optional. The language used to access language-specific data.
    /// If not specified, the agent's default language is used.
    /// For more information, see
    /// [Multilingual intent and entity
    /// data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity).
    #[prost(string, tag = "2")]
    pub language_code: std::string::String,
    /// Optional. The mask to control which fields get updated.
    #[prost(message, optional, tag = "3")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// Optional. The resource view to apply to the returned intent.
    #[prost(enumeration = "IntentView", tag = "4")]
    pub intent_view: i32,
}
/// The request message for [Intents.DeleteIntent][google.cloud.dialogflow.v2.Intents.DeleteIntent].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteIntentRequest {
    /// Required. The name of the intent to delete. If this intent has direct or
    /// indirect followup intents, we also delete them.
    /// Format: `projects/<Project ID>/agent/intents/<Intent ID>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateIntentsRequest {
    /// Required. The name of the agent to update or create intents in.
    /// Format: `projects/<Project ID>/agent`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The language used to access language-specific data.
    /// If not specified, the agent's default language is used.
    /// For more information, see
    /// [Multilingual intent and entity
    /// data](https://cloud.google.com/dialogflow/docs/agents-multilingual#intent-entity).
    #[prost(string, tag = "4")]
    pub language_code: std::string::String,
    /// Optional. The mask to control which fields get updated.
    #[prost(message, optional, tag = "5")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// Optional. The resource view to apply to the returned intent.
    #[prost(enumeration = "IntentView", tag = "6")]
    pub intent_view: i32,
    /// The source of the intent batch.
    #[prost(oneof = "batch_update_intents_request::IntentBatch", tags = "2, 3")]
    pub intent_batch: ::std::option::Option<batch_update_intents_request::IntentBatch>,
}
pub mod batch_update_intents_request {
    /// The source of the intent batch.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum IntentBatch {
        /// The URI to a Google Cloud Storage file containing intents to update or
        /// create. The file format can either be a serialized proto (of IntentBatch
        /// type) or JSON object. Note: The URI must start with "gs://".
        #[prost(string, tag = "2")]
        IntentBatchUri(std::string::String),
        /// The collection of intents to update or create.
        #[prost(message, tag = "3")]
        IntentBatchInline(super::IntentBatch),
    }
}
/// The response message for [Intents.BatchUpdateIntents][google.cloud.dialogflow.v2.Intents.BatchUpdateIntents].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchUpdateIntentsResponse {
    /// The collection of updated or created intents.
    #[prost(message, repeated, tag = "1")]
    pub intents: ::std::vec::Vec<Intent>,
}
/// The request message for [Intents.BatchDeleteIntents][google.cloud.dialogflow.v2.Intents.BatchDeleteIntents].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchDeleteIntentsRequest {
    /// Required. The name of the agent to delete all entities types for. Format:
    /// `projects/<Project ID>/agent`.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The collection of intents to delete. Only intent `name` must be
    /// filled in.
    #[prost(message, repeated, tag = "2")]
    pub intents: ::std::vec::Vec<Intent>,
}
/// This message is a wrapper around a collection of intents.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IntentBatch {
    /// A collection of intents.
    #[prost(message, repeated, tag = "1")]
    pub intents: ::std::vec::Vec<Intent>,
}
/// Represents the options for views of an intent.
/// An intent can be a sizable object. Therefore, we provide a resource view that
/// does not return training phrases in the response by default.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IntentView {
    /// Training phrases field is not populated in the response.
    Unspecified = 0,
    /// All fields are populated.
    Full = 1,
}
#[doc = r" Generated client implementations."]
pub mod intents_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " An intent represents a mapping between input from a user and an action to"]
    #[doc = " be taken by your application. When you pass user input to the"]
    #[doc = " [DetectIntent][google.cloud.dialogflow.v2.Sessions.DetectIntent] (or"]
    #[doc = " [StreamingDetectIntent][google.cloud.dialogflow.v2.Sessions.StreamingDetectIntent]) method, the"]
    #[doc = " Dialogflow API analyzes the input and searches"]
    #[doc = " for a matching intent. If no match is found, the Dialogflow API returns a"]
    #[doc = " fallback intent (`is_fallback` = true)."]
    #[doc = ""]
    #[doc = " You can provide additional information for the Dialogflow API to use to"]
    #[doc = " match user input to an intent by adding the following to your intent."]
    #[doc = ""]
    #[doc = " *   **Contexts** - provide additional context for intent analysis. For"]
    #[doc = "     example, if an intent is related to an object in your application that"]
    #[doc = "     plays music, you can provide a context to determine when to match the"]
    #[doc = "     intent if the user input is \"turn it off\". You can include a context"]
    #[doc = "     that matches the intent when there is previous user input of"]
    #[doc = "     \"play music\", and not when there is previous user input of"]
    #[doc = "     \"turn on the light\"."]
    #[doc = ""]
    #[doc = " *   **Events** - allow for matching an intent by using an event name"]
    #[doc = "     instead of user input. Your application can provide an event name and"]
    #[doc = "     related parameters to the Dialogflow API to match an intent. For"]
    #[doc = "     example, when your application starts, you can send a welcome event"]
    #[doc = "     with a user name parameter to the Dialogflow API to match an intent with"]
    #[doc = "     a personalized welcome message for the user."]
    #[doc = ""]
    #[doc = " *   **Training phrases** - provide examples of user input to train the"]
    #[doc = "     Dialogflow API agent to better match intents."]
    #[doc = ""]
    #[doc = " For more information about intents, see the"]
    #[doc = " [Dialogflow"]
    #[doc = " documentation](https://cloud.google.com/dialogflow/docs/intents-overview)."]
    pub struct IntentsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl IntentsClient<tonic::transport::Channel> {
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
    impl<T> IntentsClient<T>
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
        #[doc = " Returns the list of all intents in the specified agent."]
        pub async fn list_intents(
            &mut self,
            request: impl tonic::IntoRequest<super::ListIntentsRequest>,
        ) -> Result<tonic::Response<super::ListIntentsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.v2.Intents/ListIntents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the specified intent."]
        pub async fn get_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::GetIntentRequest>,
        ) -> Result<tonic::Response<super::Intent>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.v2.Intents/GetIntent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates an intent in the specified agent."]
        pub async fn create_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateIntentRequest>,
        ) -> Result<tonic::Response<super::Intent>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.v2.Intents/CreateIntent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified intent."]
        pub async fn update_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateIntentRequest>,
        ) -> Result<tonic::Response<super::Intent>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.v2.Intents/UpdateIntent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified intent and its direct or indirect followup intents."]
        pub async fn delete_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteIntentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.v2.Intents/DeleteIntent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates/Creates multiple intents in the specified agent."]
        #[doc = ""]
        #[doc = " Operation <response: [BatchUpdateIntentsResponse][google.cloud.dialogflow.v2.BatchUpdateIntentsResponse]>"]
        pub async fn batch_update_intents(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchUpdateIntentsRequest>,
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
                "/google.cloud.dialogflow.v2.Intents/BatchUpdateIntents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes intents in the specified agent."]
        #[doc = ""]
        #[doc = " Operation <response: [google.protobuf.Empty][google.protobuf.Empty]>"]
        pub async fn batch_delete_intents(
            &mut self,
            request: impl tonic::IntoRequest<super::BatchDeleteIntentsRequest>,
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
                "/google.cloud.dialogflow.v2.Intents/BatchDeleteIntents",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for IntentsClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for IntentsClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "IntentsClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod intents_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with IntentsServer."]
    #[async_trait]
    pub trait Intents: Send + Sync + 'static {
        #[doc = " Returns the list of all intents in the specified agent."]
        async fn list_intents(
            &self,
            request: tonic::Request<super::ListIntentsRequest>,
        ) -> Result<tonic::Response<super::ListIntentsResponse>, tonic::Status>;
        #[doc = " Retrieves the specified intent."]
        async fn get_intent(
            &self,
            request: tonic::Request<super::GetIntentRequest>,
        ) -> Result<tonic::Response<super::Intent>, tonic::Status>;
        #[doc = " Creates an intent in the specified agent."]
        async fn create_intent(
            &self,
            request: tonic::Request<super::CreateIntentRequest>,
        ) -> Result<tonic::Response<super::Intent>, tonic::Status>;
        #[doc = " Updates the specified intent."]
        async fn update_intent(
            &self,
            request: tonic::Request<super::UpdateIntentRequest>,
        ) -> Result<tonic::Response<super::Intent>, tonic::Status>;
        #[doc = " Deletes the specified intent and its direct or indirect followup intents."]
        async fn delete_intent(
            &self,
            request: tonic::Request<super::DeleteIntentRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
        #[doc = " Updates/Creates multiple intents in the specified agent."]
        #[doc = ""]
        #[doc = " Operation <response: [BatchUpdateIntentsResponse][google.cloud.dialogflow.v2.BatchUpdateIntentsResponse]>"]
        async fn batch_update_intents(
            &self,
            request: tonic::Request<super::BatchUpdateIntentsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
        #[doc = " Deletes intents in the specified agent."]
        #[doc = ""]
        #[doc = " Operation <response: [google.protobuf.Empty][google.protobuf.Empty]>"]
        async fn batch_delete_intents(
            &self,
            request: tonic::Request<super::BatchDeleteIntentsRequest>,
        ) -> Result<
            tonic::Response<super::super::super::super::longrunning::Operation>,
            tonic::Status,
        >;
    }
    #[doc = " An intent represents a mapping between input from a user and an action to"]
    #[doc = " be taken by your application. When you pass user input to the"]
    #[doc = " [DetectIntent][google.cloud.dialogflow.v2.Sessions.DetectIntent] (or"]
    #[doc = " [StreamingDetectIntent][google.cloud.dialogflow.v2.Sessions.StreamingDetectIntent]) method, the"]
    #[doc = " Dialogflow API analyzes the input and searches"]
    #[doc = " for a matching intent. If no match is found, the Dialogflow API returns a"]
    #[doc = " fallback intent (`is_fallback` = true)."]
    #[doc = ""]
    #[doc = " You can provide additional information for the Dialogflow API to use to"]
    #[doc = " match user input to an intent by adding the following to your intent."]
    #[doc = ""]
    #[doc = " *   **Contexts** - provide additional context for intent analysis. For"]
    #[doc = "     example, if an intent is related to an object in your application that"]
    #[doc = "     plays music, you can provide a context to determine when to match the"]
    #[doc = "     intent if the user input is \"turn it off\". You can include a context"]
    #[doc = "     that matches the intent when there is previous user input of"]
    #[doc = "     \"play music\", and not when there is previous user input of"]
    #[doc = "     \"turn on the light\"."]
    #[doc = ""]
    #[doc = " *   **Events** - allow for matching an intent by using an event name"]
    #[doc = "     instead of user input. Your application can provide an event name and"]
    #[doc = "     related parameters to the Dialogflow API to match an intent. For"]
    #[doc = "     example, when your application starts, you can send a welcome event"]
    #[doc = "     with a user name parameter to the Dialogflow API to match an intent with"]
    #[doc = "     a personalized welcome message for the user."]
    #[doc = ""]
    #[doc = " *   **Training phrases** - provide examples of user input to train the"]
    #[doc = "     Dialogflow API agent to better match intents."]
    #[doc = ""]
    #[doc = " For more information about intents, see the"]
    #[doc = " [Dialogflow"]
    #[doc = " documentation](https://cloud.google.com/dialogflow/docs/intents-overview)."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct IntentsServer<T: Intents> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Intents> IntentsServer<T> {
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
    impl<T, B> Service<http::Request<B>> for IntentsServer<T>
    where
        T: Intents,
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
                "/google.cloud.dialogflow.v2.Intents/ListIntents" => {
                    #[allow(non_camel_case_types)]
                    struct ListIntentsSvc<T: Intents>(pub Arc<T>);
                    impl<T: Intents> tonic::server::UnaryService<super::ListIntentsRequest> for ListIntentsSvc<T> {
                        type Response = super::ListIntentsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListIntentsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_intents(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListIntentsSvc(inner);
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
                "/google.cloud.dialogflow.v2.Intents/GetIntent" => {
                    #[allow(non_camel_case_types)]
                    struct GetIntentSvc<T: Intents>(pub Arc<T>);
                    impl<T: Intents> tonic::server::UnaryService<super::GetIntentRequest> for GetIntentSvc<T> {
                        type Response = super::Intent;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetIntentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_intent(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetIntentSvc(inner);
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
                "/google.cloud.dialogflow.v2.Intents/CreateIntent" => {
                    #[allow(non_camel_case_types)]
                    struct CreateIntentSvc<T: Intents>(pub Arc<T>);
                    impl<T: Intents> tonic::server::UnaryService<super::CreateIntentRequest> for CreateIntentSvc<T> {
                        type Response = super::Intent;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateIntentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.create_intent(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateIntentSvc(inner);
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
                "/google.cloud.dialogflow.v2.Intents/UpdateIntent" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateIntentSvc<T: Intents>(pub Arc<T>);
                    impl<T: Intents> tonic::server::UnaryService<super::UpdateIntentRequest> for UpdateIntentSvc<T> {
                        type Response = super::Intent;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateIntentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.update_intent(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateIntentSvc(inner);
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
                "/google.cloud.dialogflow.v2.Intents/DeleteIntent" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteIntentSvc<T: Intents>(pub Arc<T>);
                    impl<T: Intents> tonic::server::UnaryService<super::DeleteIntentRequest> for DeleteIntentSvc<T> {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteIntentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.delete_intent(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteIntentSvc(inner);
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
                "/google.cloud.dialogflow.v2.Intents/BatchUpdateIntents" => {
                    #[allow(non_camel_case_types)]
                    struct BatchUpdateIntentsSvc<T: Intents>(pub Arc<T>);
                    impl<T: Intents> tonic::server::UnaryService<super::BatchUpdateIntentsRequest>
                        for BatchUpdateIntentsSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchUpdateIntentsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.batch_update_intents(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = BatchUpdateIntentsSvc(inner);
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
                "/google.cloud.dialogflow.v2.Intents/BatchDeleteIntents" => {
                    #[allow(non_camel_case_types)]
                    struct BatchDeleteIntentsSvc<T: Intents>(pub Arc<T>);
                    impl<T: Intents> tonic::server::UnaryService<super::BatchDeleteIntentsRequest>
                        for BatchDeleteIntentsSvc<T>
                    {
                        type Response = super::super::super::super::longrunning::Operation;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::BatchDeleteIntentsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.batch_delete_intents(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = BatchDeleteIntentsSvc(inner);
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
    impl<T: Intents> Clone for IntentsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Intents> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Intents> tonic::transport::NamedService for IntentsServer<T> {
        const NAME: &'static str = "google.cloud.dialogflow.v2.Intents";
    }
}
/// Represents a session entity type.
///
/// Extends or replaces a custom entity type at the user session level (we
/// refer to the entity types defined at the agent level as "custom entity
/// types").
///
/// Note: session entity types apply to all queries, regardless of the language.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SessionEntityType {
    /// Required. The unique identifier of this session entity type. Format:
    /// `projects/<Project ID>/agent/sessions/<Session ID>/entityTypes/<Entity Type
    /// Display Name>`, or `projects/<Project ID>/agent/environments/<Environment
    /// ID>/users/<User ID>/sessions/<Session ID>/entityTypes/<Entity Type Display
    /// Name>`.
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment. If `User ID` is not specified, we assume default '-' user.
    ///
    /// `<Entity Type Display Name>` must be the display name of an existing entity
    /// type in the same agent that will be overridden or supplemented.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Required. Indicates whether the additional data should override or
    /// supplement the custom entity type definition.
    #[prost(enumeration = "session_entity_type::EntityOverrideMode", tag = "2")]
    pub entity_override_mode: i32,
    /// Required. The collection of entities associated with this session entity
    /// type.
    #[prost(message, repeated, tag = "3")]
    pub entities: ::std::vec::Vec<entity_type::Entity>,
}
pub mod session_entity_type {
    /// The types of modifications for a session entity type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum EntityOverrideMode {
        /// Not specified. This value should be never used.
        Unspecified = 0,
        /// The collection of session entities overrides the collection of entities
        /// in the corresponding custom entity type.
        Override = 1,
        /// The collection of session entities extends the collection of entities in
        /// the corresponding custom entity type.
        ///
        /// Note: Even in this override mode calls to `ListSessionEntityTypes`,
        /// `GetSessionEntityType`, `CreateSessionEntityType` and
        /// `UpdateSessionEntityType` only return the additional entities added in
        /// this session entity type. If you want to get the supplemented list,
        /// please call [EntityTypes.GetEntityType][google.cloud.dialogflow.v2.EntityTypes.GetEntityType] on the custom entity type
        /// and merge.
        Supplement = 2,
    }
}
/// The request message for [SessionEntityTypes.ListSessionEntityTypes][google.cloud.dialogflow.v2.SessionEntityTypes.ListSessionEntityTypes].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSessionEntityTypesRequest {
    /// Required. The session to list all session entity types from.
    /// Format: `projects/<Project ID>/agent/sessions/<Session ID>` or
    /// `projects/<Project ID>/agent/environments/<Environment ID>/users/<User ID>/
    /// sessions/<Session ID>`.
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment. If `User ID` is not specified, we assume default '-' user.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Optional. The maximum number of items to return in a single page. By
    /// default 100 and at most 1000.
    #[prost(int32, tag = "2")]
    pub page_size: i32,
    /// Optional. The next_page_token value returned from a previous list request.
    #[prost(string, tag = "3")]
    pub page_token: std::string::String,
}
/// The response message for [SessionEntityTypes.ListSessionEntityTypes][google.cloud.dialogflow.v2.SessionEntityTypes.ListSessionEntityTypes].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSessionEntityTypesResponse {
    /// The list of session entity types. There will be a maximum number of items
    /// returned based on the page_size field in the request.
    #[prost(message, repeated, tag = "1")]
    pub session_entity_types: ::std::vec::Vec<SessionEntityType>,
    /// Token to retrieve the next page of results, or empty if there are no
    /// more results in the list.
    #[prost(string, tag = "2")]
    pub next_page_token: std::string::String,
}
/// The request message for [SessionEntityTypes.GetSessionEntityType][google.cloud.dialogflow.v2.SessionEntityTypes.GetSessionEntityType].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSessionEntityTypeRequest {
    /// Required. The name of the session entity type. Format:
    /// `projects/<Project ID>/agent/sessions/<Session ID>/entityTypes/<Entity Type
    /// Display Name>` or `projects/<Project ID>/agent/environments/<Environment
    /// ID>/users/<User ID>/sessions/<Session ID>/entityTypes/<Entity Type Display
    /// Name>`.
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment. If `User ID` is not specified, we assume default '-' user.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// The request message for [SessionEntityTypes.CreateSessionEntityType][google.cloud.dialogflow.v2.SessionEntityTypes.CreateSessionEntityType].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSessionEntityTypeRequest {
    /// Required. The session to create a session entity type for.
    /// Format: `projects/<Project ID>/agent/sessions/<Session ID>` or
    /// `projects/<Project ID>/agent/environments/<Environment ID>/users/<User ID>/
    /// sessions/<Session ID>`.
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment. If `User ID` is not specified, we assume default '-' user.
    #[prost(string, tag = "1")]
    pub parent: std::string::String,
    /// Required. The session entity type to create.
    #[prost(message, optional, tag = "2")]
    pub session_entity_type: ::std::option::Option<SessionEntityType>,
}
/// The request message for [SessionEntityTypes.UpdateSessionEntityType][google.cloud.dialogflow.v2.SessionEntityTypes.UpdateSessionEntityType].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateSessionEntityTypeRequest {
    /// Required. The session entity type to update.
    #[prost(message, optional, tag = "1")]
    pub session_entity_type: ::std::option::Option<SessionEntityType>,
    /// Optional. The mask to control which fields get updated.
    #[prost(message, optional, tag = "2")]
    pub update_mask: ::std::option::Option<::prost_types::FieldMask>,
}
/// The request message for [SessionEntityTypes.DeleteSessionEntityType][google.cloud.dialogflow.v2.SessionEntityTypes.DeleteSessionEntityType].
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteSessionEntityTypeRequest {
    /// Required. The name of the entity type to delete. Format:
    /// `projects/<Project ID>/agent/sessions/<Session ID>/entityTypes/<Entity Type
    /// Display Name>` or `projects/<Project ID>/agent/environments/<Environment
    /// ID>/users/<User ID>/sessions/<Session ID>/entityTypes/<Entity Type Display
    /// Name>`.
    /// If `Environment ID` is not specified, we assume default 'draft'
    /// environment. If `User ID` is not specified, we assume default '-' user.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[doc = r" Generated client implementations."]
pub mod session_entity_types_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Entities are extracted from user input and represent parameters that are"]
    #[doc = " meaningful to your application. For example, a date range, a proper name"]
    #[doc = " such as a geographic location or landmark, and so on. Entities represent"]
    #[doc = " actionable data for your application."]
    #[doc = ""]
    #[doc = " Session entity types are referred to as **User** entity types and are"]
    #[doc = " entities that are built for an individual user such as"]
    #[doc = " favorites, preferences, playlists, and so on. You can redefine a session"]
    #[doc = " entity type at the session level."]
    #[doc = ""]
    #[doc = " Session entity methods do not work with Google Assistant integration."]
    #[doc = " Contact Dialogflow support if you need to use session entities"]
    #[doc = " with Google Assistant integration."]
    #[doc = ""]
    #[doc = " For more information about entity types, see the"]
    #[doc = " [Dialogflow"]
    #[doc = " documentation](https://cloud.google.com/dialogflow/docs/entities-overview)."]
    pub struct SessionEntityTypesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SessionEntityTypesClient<tonic::transport::Channel> {
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
    impl<T> SessionEntityTypesClient<T>
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
        #[doc = " Returns the list of all session entity types in the specified session."]
        #[doc = ""]
        #[doc = " This method doesn't work with Google Assistant integration."]
        #[doc = " Contact Dialogflow support if you need to use session entities"]
        #[doc = " with Google Assistant integration."]
        pub async fn list_session_entity_types(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSessionEntityTypesRequest>,
        ) -> Result<tonic::Response<super::ListSessionEntityTypesResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.v2.SessionEntityTypes/ListSessionEntityTypes",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Retrieves the specified session entity type."]
        #[doc = ""]
        #[doc = " This method doesn't work with Google Assistant integration."]
        #[doc = " Contact Dialogflow support if you need to use session entities"]
        #[doc = " with Google Assistant integration."]
        pub async fn get_session_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSessionEntityTypeRequest>,
        ) -> Result<tonic::Response<super::SessionEntityType>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.v2.SessionEntityTypes/GetSessionEntityType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Creates a session entity type."]
        #[doc = ""]
        #[doc = " If the specified session entity type already exists, overrides the session"]
        #[doc = " entity type."]
        #[doc = ""]
        #[doc = " This method doesn't work with Google Assistant integration."]
        #[doc = " Contact Dialogflow support if you need to use session entities"]
        #[doc = " with Google Assistant integration."]
        pub async fn create_session_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSessionEntityTypeRequest>,
        ) -> Result<tonic::Response<super::SessionEntityType>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.v2.SessionEntityTypes/CreateSessionEntityType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Updates the specified session entity type."]
        #[doc = ""]
        #[doc = " This method doesn't work with Google Assistant integration."]
        #[doc = " Contact Dialogflow support if you need to use session entities"]
        #[doc = " with Google Assistant integration."]
        pub async fn update_session_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSessionEntityTypeRequest>,
        ) -> Result<tonic::Response<super::SessionEntityType>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.v2.SessionEntityTypes/UpdateSessionEntityType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Deletes the specified session entity type."]
        #[doc = ""]
        #[doc = " This method doesn't work with Google Assistant integration."]
        #[doc = " Contact Dialogflow support if you need to use session entities"]
        #[doc = " with Google Assistant integration."]
        pub async fn delete_session_entity_type(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteSessionEntityTypeRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.v2.SessionEntityTypes/DeleteSessionEntityType",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for SessionEntityTypesClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for SessionEntityTypesClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SessionEntityTypesClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod session_entity_types_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with SessionEntityTypesServer."]
    #[async_trait]
    pub trait SessionEntityTypes: Send + Sync + 'static {
        #[doc = " Returns the list of all session entity types in the specified session."]
        #[doc = ""]
        #[doc = " This method doesn't work with Google Assistant integration."]
        #[doc = " Contact Dialogflow support if you need to use session entities"]
        #[doc = " with Google Assistant integration."]
        async fn list_session_entity_types(
            &self,
            request: tonic::Request<super::ListSessionEntityTypesRequest>,
        ) -> Result<tonic::Response<super::ListSessionEntityTypesResponse>, tonic::Status>;
        #[doc = " Retrieves the specified session entity type."]
        #[doc = ""]
        #[doc = " This method doesn't work with Google Assistant integration."]
        #[doc = " Contact Dialogflow support if you need to use session entities"]
        #[doc = " with Google Assistant integration."]
        async fn get_session_entity_type(
            &self,
            request: tonic::Request<super::GetSessionEntityTypeRequest>,
        ) -> Result<tonic::Response<super::SessionEntityType>, tonic::Status>;
        #[doc = " Creates a session entity type."]
        #[doc = ""]
        #[doc = " If the specified session entity type already exists, overrides the session"]
        #[doc = " entity type."]
        #[doc = ""]
        #[doc = " This method doesn't work with Google Assistant integration."]
        #[doc = " Contact Dialogflow support if you need to use session entities"]
        #[doc = " with Google Assistant integration."]
        async fn create_session_entity_type(
            &self,
            request: tonic::Request<super::CreateSessionEntityTypeRequest>,
        ) -> Result<tonic::Response<super::SessionEntityType>, tonic::Status>;
        #[doc = " Updates the specified session entity type."]
        #[doc = ""]
        #[doc = " This method doesn't work with Google Assistant integration."]
        #[doc = " Contact Dialogflow support if you need to use session entities"]
        #[doc = " with Google Assistant integration."]
        async fn update_session_entity_type(
            &self,
            request: tonic::Request<super::UpdateSessionEntityTypeRequest>,
        ) -> Result<tonic::Response<super::SessionEntityType>, tonic::Status>;
        #[doc = " Deletes the specified session entity type."]
        #[doc = ""]
        #[doc = " This method doesn't work with Google Assistant integration."]
        #[doc = " Contact Dialogflow support if you need to use session entities"]
        #[doc = " with Google Assistant integration."]
        async fn delete_session_entity_type(
            &self,
            request: tonic::Request<super::DeleteSessionEntityTypeRequest>,
        ) -> Result<tonic::Response<()>, tonic::Status>;
    }
    #[doc = " Entities are extracted from user input and represent parameters that are"]
    #[doc = " meaningful to your application. For example, a date range, a proper name"]
    #[doc = " such as a geographic location or landmark, and so on. Entities represent"]
    #[doc = " actionable data for your application."]
    #[doc = ""]
    #[doc = " Session entity types are referred to as **User** entity types and are"]
    #[doc = " entities that are built for an individual user such as"]
    #[doc = " favorites, preferences, playlists, and so on. You can redefine a session"]
    #[doc = " entity type at the session level."]
    #[doc = ""]
    #[doc = " Session entity methods do not work with Google Assistant integration."]
    #[doc = " Contact Dialogflow support if you need to use session entities"]
    #[doc = " with Google Assistant integration."]
    #[doc = ""]
    #[doc = " For more information about entity types, see the"]
    #[doc = " [Dialogflow"]
    #[doc = " documentation](https://cloud.google.com/dialogflow/docs/entities-overview)."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct SessionEntityTypesServer<T: SessionEntityTypes> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: SessionEntityTypes> SessionEntityTypesServer<T> {
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
    impl<T, B> Service<http::Request<B>> for SessionEntityTypesServer<T>
    where
        T: SessionEntityTypes,
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
                "/google.cloud.dialogflow.v2.SessionEntityTypes/ListSessionEntityTypes" => {
                    #[allow(non_camel_case_types)]
                    struct ListSessionEntityTypesSvc<T: SessionEntityTypes>(pub Arc<T>);
                    impl<T: SessionEntityTypes>
                        tonic::server::UnaryService<super::ListSessionEntityTypesRequest>
                        for ListSessionEntityTypesSvc<T>
                    {
                        type Response = super::ListSessionEntityTypesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListSessionEntityTypesRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.list_session_entity_types(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ListSessionEntityTypesSvc(inner);
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
                "/google.cloud.dialogflow.v2.SessionEntityTypes/GetSessionEntityType" => {
                    #[allow(non_camel_case_types)]
                    struct GetSessionEntityTypeSvc<T: SessionEntityTypes>(pub Arc<T>);
                    impl<T: SessionEntityTypes>
                        tonic::server::UnaryService<super::GetSessionEntityTypeRequest>
                        for GetSessionEntityTypeSvc<T>
                    {
                        type Response = super::SessionEntityType;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetSessionEntityTypeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_session_entity_type(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetSessionEntityTypeSvc(inner);
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
                "/google.cloud.dialogflow.v2.SessionEntityTypes/CreateSessionEntityType" => {
                    #[allow(non_camel_case_types)]
                    struct CreateSessionEntityTypeSvc<T: SessionEntityTypes>(pub Arc<T>);
                    impl<T: SessionEntityTypes>
                        tonic::server::UnaryService<super::CreateSessionEntityTypeRequest>
                        for CreateSessionEntityTypeSvc<T>
                    {
                        type Response = super::SessionEntityType;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateSessionEntityTypeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.create_session_entity_type(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateSessionEntityTypeSvc(inner);
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
                "/google.cloud.dialogflow.v2.SessionEntityTypes/UpdateSessionEntityType" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateSessionEntityTypeSvc<T: SessionEntityTypes>(pub Arc<T>);
                    impl<T: SessionEntityTypes>
                        tonic::server::UnaryService<super::UpdateSessionEntityTypeRequest>
                        for UpdateSessionEntityTypeSvc<T>
                    {
                        type Response = super::SessionEntityType;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateSessionEntityTypeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.update_session_entity_type(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = UpdateSessionEntityTypeSvc(inner);
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
                "/google.cloud.dialogflow.v2.SessionEntityTypes/DeleteSessionEntityType" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteSessionEntityTypeSvc<T: SessionEntityTypes>(pub Arc<T>);
                    impl<T: SessionEntityTypes>
                        tonic::server::UnaryService<super::DeleteSessionEntityTypeRequest>
                        for DeleteSessionEntityTypeSvc<T>
                    {
                        type Response = ();
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteSessionEntityTypeRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { inner.delete_session_entity_type(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DeleteSessionEntityTypeSvc(inner);
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
    impl<T: SessionEntityTypes> Clone for SessionEntityTypesServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: SessionEntityTypes> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: SessionEntityTypes> tonic::transport::NamedService for SessionEntityTypesServer<T> {
        const NAME: &'static str = "google.cloud.dialogflow.v2.SessionEntityTypes";
    }
}
/// The request to detect user's intent.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetectIntentRequest {
    /// Required. The name of the session this query is sent to. Format:
    /// `projects/<Project ID>/agent/sessions/<Session ID>`, or
    /// `projects/<Project ID>/agent/environments/<Environment ID>/users/<User
    /// ID>/sessions/<Session ID>`. If `Environment ID` is not specified, we assume
    /// default 'draft' environment. If `User ID` is not specified, we are using
    /// "-". It's up to the API caller to choose an appropriate `Session ID` and
    /// `User Id`. They can be a random number or some type of user and session
    /// identifiers (preferably hashed). The length of the `Session ID` and
    /// `User ID` must not exceed 36 characters.
    #[prost(string, tag = "1")]
    pub session: std::string::String,
    /// The parameters of this query.
    #[prost(message, optional, tag = "2")]
    pub query_params: ::std::option::Option<QueryParameters>,
    /// Required. The input specification. It can be set to:
    ///
    /// 1.  an audio config
    ///     which instructs the speech recognizer how to process the speech audio,
    ///
    /// 2.  a conversational query in the form of text, or
    ///
    /// 3.  an event that specifies which intent to trigger.
    #[prost(message, optional, tag = "3")]
    pub query_input: ::std::option::Option<QueryInput>,
    /// Instructs the speech synthesizer how to generate the output
    /// audio. If this field is not set and agent-level speech synthesizer is not
    /// configured, no output audio is generated.
    #[prost(message, optional, tag = "4")]
    pub output_audio_config: ::std::option::Option<OutputAudioConfig>,
    /// Mask for [output_audio_config][google.cloud.dialogflow.v2.DetectIntentRequest.output_audio_config] indicating which settings in this
    /// request-level config should override speech synthesizer settings defined at
    /// agent-level.
    ///
    /// If unspecified or empty, [output_audio_config][google.cloud.dialogflow.v2.DetectIntentRequest.output_audio_config] replaces the agent-level
    /// config in its entirety.
    #[prost(message, optional, tag = "7")]
    pub output_audio_config_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The natural language speech audio to be processed. This field
    /// should be populated iff `query_input` is set to an input audio config.
    /// A single request can contain up to 1 minute of speech audio data.
    #[prost(bytes, tag = "5")]
    pub input_audio: std::vec::Vec<u8>,
}
/// The message returned from the DetectIntent method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetectIntentResponse {
    /// The unique identifier of the response. It can be used to
    /// locate a response in the training example set or for reporting issues.
    #[prost(string, tag = "1")]
    pub response_id: std::string::String,
    /// The selected results of the conversational query or event processing.
    /// See `alternative_query_results` for additional potential results.
    #[prost(message, optional, tag = "2")]
    pub query_result: ::std::option::Option<QueryResult>,
    /// Specifies the status of the webhook request.
    #[prost(message, optional, tag = "3")]
    pub webhook_status: ::std::option::Option<super::super::super::rpc::Status>,
    /// The audio data bytes encoded as specified in the request.
    /// Note: The output audio is generated based on the values of default platform
    /// text responses found in the `query_result.fulfillment_messages` field. If
    /// multiple default text responses exist, they will be concatenated when
    /// generating audio. If no default platform text responses exist, the
    /// generated audio content will be empty.
    #[prost(bytes, tag = "4")]
    pub output_audio: std::vec::Vec<u8>,
    /// The config used by the speech synthesizer to generate the output audio.
    #[prost(message, optional, tag = "6")]
    pub output_audio_config: ::std::option::Option<OutputAudioConfig>,
}
/// Represents the parameters of the conversational query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParameters {
    /// The time zone of this conversational query from the
    /// [time zone database](https://www.iana.org/time-zones), e.g.,
    /// America/New_York, Europe/Paris. If not provided, the time zone specified in
    /// agent settings is used.
    #[prost(string, tag = "1")]
    pub time_zone: std::string::String,
    /// The geo location of this conversational query.
    #[prost(message, optional, tag = "2")]
    pub geo_location: ::std::option::Option<super::super::super::r#type::LatLng>,
    /// The collection of contexts to be activated before this query is
    /// executed.
    #[prost(message, repeated, tag = "3")]
    pub contexts: ::std::vec::Vec<Context>,
    /// Specifies whether to delete all contexts in the current session
    /// before the new ones are activated.
    #[prost(bool, tag = "4")]
    pub reset_contexts: bool,
    /// Additional session entity types to replace or extend developer
    /// entity types with. The entity synonyms apply to all languages and persist
    /// for the session of this query.
    #[prost(message, repeated, tag = "5")]
    pub session_entity_types: ::std::vec::Vec<SessionEntityType>,
    /// This field can be used to pass custom data to your webhook.
    /// Arbitrary JSON objects are supported.
    /// If supplied, the value is used to populate the
    /// `WebhookRequest.original_detect_intent_request.payload`
    /// field sent to your webhook.
    #[prost(message, optional, tag = "6")]
    pub payload: ::std::option::Option<::prost_types::Struct>,
    /// Configures the type of sentiment analysis to perform. If not
    /// provided, sentiment analysis is not performed.
    #[prost(message, optional, tag = "10")]
    pub sentiment_analysis_request_config: ::std::option::Option<SentimentAnalysisRequestConfig>,
}
/// Represents the query input. It can contain either:
///
/// 1.  An audio config which
///     instructs the speech recognizer how to process the speech audio.
///
/// 2.  A conversational query in the form of text,.
///
/// 3.  An event that specifies which intent to trigger.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryInput {
    /// Required. The input specification.
    #[prost(oneof = "query_input::Input", tags = "1, 2, 3")]
    pub input: ::std::option::Option<query_input::Input>,
}
pub mod query_input {
    /// Required. The input specification.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Input {
        /// Instructs the speech recognizer how to process the speech audio.
        #[prost(message, tag = "1")]
        AudioConfig(super::InputAudioConfig),
        /// The natural language text to be processed.
        #[prost(message, tag = "2")]
        Text(super::TextInput),
        /// The event to be processed.
        #[prost(message, tag = "3")]
        Event(super::EventInput),
    }
}
/// Represents the result of conversational query or event processing.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResult {
    /// The original conversational query text:
    ///
    /// - If natural language text was provided as input, `query_text` contains
    ///   a copy of the input.
    /// - If natural language speech audio was provided as input, `query_text`
    ///   contains the speech recognition result. If speech recognizer produced
    ///   multiple alternatives, a particular one is picked.
    /// - If automatic spell correction is enabled, `query_text` will contain the
    ///   corrected user input.
    #[prost(string, tag = "1")]
    pub query_text: std::string::String,
    /// The language that was triggered during intent detection.
    /// See [Language
    /// Support](https://cloud.google.com/dialogflow/docs/reference/language)
    /// for a list of the currently supported language codes.
    #[prost(string, tag = "15")]
    pub language_code: std::string::String,
    /// The Speech recognition confidence between 0.0 and 1.0. A higher number
    /// indicates an estimated greater likelihood that the recognized words are
    /// correct. The default of 0.0 is a sentinel value indicating that confidence
    /// was not set.
    ///
    /// This field is not guaranteed to be accurate or set. In particular this
    /// field isn't set for StreamingDetectIntent since the streaming endpoint has
    /// separate confidence estimates per portion of the audio in
    /// StreamingRecognitionResult.
    #[prost(float, tag = "2")]
    pub speech_recognition_confidence: f32,
    /// The action name from the matched intent.
    #[prost(string, tag = "3")]
    pub action: std::string::String,
    /// The collection of extracted parameters.
    ///
    /// Depending on your protocol or client library language, this is a
    /// map, associative array, symbol table, dictionary, or JSON object
    /// composed of a collection of (MapKey, MapValue) pairs:
    ///
    /// -   MapKey type: string
    /// -   MapKey value: parameter name
    /// -   MapValue type:
    ///     -   If parameter's entity type is a composite entity: map
    ///     -   Else: string or number, depending on parameter value type
    /// -   MapValue value:
    ///     -   If parameter's entity type is a composite entity:
    ///         map from composite entity property names to property values
    ///     -   Else: parameter value
    #[prost(message, optional, tag = "4")]
    pub parameters: ::std::option::Option<::prost_types::Struct>,
    /// This field is set to:
    ///
    /// - `false` if the matched intent has required parameters and not all of
    ///    the required parameter values have been collected.
    /// - `true` if all required parameter values have been collected, or if the
    ///    matched intent doesn't contain any required parameters.
    #[prost(bool, tag = "5")]
    pub all_required_params_present: bool,
    /// The text to be pronounced to the user or shown on the screen.
    /// Note: This is a legacy field, `fulfillment_messages` should be preferred.
    #[prost(string, tag = "6")]
    pub fulfillment_text: std::string::String,
    /// The collection of rich messages to present to the user.
    #[prost(message, repeated, tag = "7")]
    pub fulfillment_messages: ::std::vec::Vec<intent::Message>,
    /// If the query was fulfilled by a webhook call, this field is set to the
    /// value of the `source` field returned in the webhook response.
    #[prost(string, tag = "8")]
    pub webhook_source: std::string::String,
    /// If the query was fulfilled by a webhook call, this field is set to the
    /// value of the `payload` field returned in the webhook response.
    #[prost(message, optional, tag = "9")]
    pub webhook_payload: ::std::option::Option<::prost_types::Struct>,
    /// The collection of output contexts. If applicable,
    /// `output_contexts.parameters` contains entries with name
    /// `<parameter name>.original` containing the original parameter values
    /// before the query.
    #[prost(message, repeated, tag = "10")]
    pub output_contexts: ::std::vec::Vec<Context>,
    /// The intent that matched the conversational query. Some, not
    /// all fields are filled in this message, including but not limited to:
    /// `name`, `display_name`, `end_interaction` and `is_fallback`.
    #[prost(message, optional, tag = "11")]
    pub intent: ::std::option::Option<Intent>,
    /// The intent detection confidence. Values range from 0.0
    /// (completely uncertain) to 1.0 (completely certain).
    /// This value is for informational purpose only and is only used to
    /// help match the best intent within the classification threshold.
    /// This value may change for the same end-user expression at any time due to a
    /// model retraining or change in implementation.
    /// If there are `multiple knowledge_answers` messages, this value is set to
    /// the greatest `knowledgeAnswers.match_confidence` value in the list.
    #[prost(float, tag = "12")]
    pub intent_detection_confidence: f32,
    /// Free-form diagnostic information for the associated detect intent request.
    /// The fields of this data can change without notice, so you should not write
    /// code that depends on its structure.
    /// The data may contain:
    ///
    /// - webhook call latency
    /// - webhook errors
    #[prost(message, optional, tag = "14")]
    pub diagnostic_info: ::std::option::Option<::prost_types::Struct>,
    /// The sentiment analysis result, which depends on the
    /// `sentiment_analysis_request_config` specified in the request.
    #[prost(message, optional, tag = "17")]
    pub sentiment_analysis_result: ::std::option::Option<SentimentAnalysisResult>,
}
/// The top-level message sent by the client to the
/// [Sessions.StreamingDetectIntent][google.cloud.dialogflow.v2.Sessions.StreamingDetectIntent] method.
///
/// Multiple request messages should be sent in order:
///
/// 1.  The first message must contain
/// [session][google.cloud.dialogflow.v2.StreamingDetectIntentRequest.session],
///     [query_input][google.cloud.dialogflow.v2.StreamingDetectIntentRequest.query_input] plus optionally
///     [query_params][google.cloud.dialogflow.v2.StreamingDetectIntentRequest.query_params]. If the client
///     wants to receive an audio response, it should also contain
///     [output_audio_config][google.cloud.dialogflow.v2.StreamingDetectIntentRequest.output_audio_config].
///     The message must not contain
///     [input_audio][google.cloud.dialogflow.v2.StreamingDetectIntentRequest.input_audio].
/// 2.  If [query_input][google.cloud.dialogflow.v2.StreamingDetectIntentRequest.query_input] was set to
///     [query_input.audio_config][google.cloud.dialogflow.v2.InputAudioConfig], all subsequent
///     messages must contain
///     [input_audio][google.cloud.dialogflow.v2.StreamingDetectIntentRequest.input_audio] to continue with
///     Speech recognition.
///     If you decide to rather detect an intent from text input after you
///     already started Speech recognition, please send a message with
///     [query_input.text][google.cloud.dialogflow.v2.QueryInput.text].
///
///     However, note that:
///
///     * Dialogflow will bill you for the audio duration so far.
///     * Dialogflow discards all Speech recognition results in favor of the
///       input text.
///     * Dialogflow will use the language code from the first message.
///
/// After you sent all input, you must half-close or abort the request stream.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingDetectIntentRequest {
    /// Required. The name of the session the query is sent to.
    /// Format of the session name:
    /// `projects/<Project ID>/agent/sessions/<Session ID>`, or
    /// `projects/<Project ID>/agent/environments/<Environment ID>/users/<User
    /// ID>/sessions/<Session ID>`. If `Environment ID` is not specified, we assume
    /// default 'draft' environment. If `User ID` is not specified, we are using
    /// "-". It's up to the API caller to choose an appropriate `Session ID` and
    /// `User Id`. They can be a random number or some type of user and session
    /// identifiers (preferably hashed). The length of the `Session ID` and
    /// `User ID` must not exceed 36 characters.
    #[prost(string, tag = "1")]
    pub session: std::string::String,
    /// The parameters of this query.
    #[prost(message, optional, tag = "2")]
    pub query_params: ::std::option::Option<QueryParameters>,
    /// Required. The input specification. It can be set to:
    ///
    /// 1.  an audio config which instructs the speech recognizer how to process
    ///     the speech audio,
    ///
    /// 2.  a conversational query in the form of text, or
    ///
    /// 3.  an event that specifies which intent to trigger.
    #[prost(message, optional, tag = "3")]
    pub query_input: ::std::option::Option<QueryInput>,
    /// Please use [InputAudioConfig.single_utterance][google.cloud.dialogflow.v2.InputAudioConfig.single_utterance] instead.
    /// If `false` (default), recognition does not cease until
    /// the client closes the stream. If `true`, the recognizer will detect a
    /// single spoken utterance in input audio. Recognition ceases when it detects
    /// the audio's voice has stopped or paused. In this case, once a detected
    /// intent is received, the client should close the stream and start a new
    /// request with a new stream as needed.
    /// This setting is ignored when `query_input` is a piece of text or an event.
    #[prost(bool, tag = "4")]
    pub single_utterance: bool,
    /// Instructs the speech synthesizer how to generate the output
    /// audio. If this field is not set and agent-level speech synthesizer is not
    /// configured, no output audio is generated.
    #[prost(message, optional, tag = "5")]
    pub output_audio_config: ::std::option::Option<OutputAudioConfig>,
    /// Mask for [output_audio_config][google.cloud.dialogflow.v2.StreamingDetectIntentRequest.output_audio_config] indicating which settings in this
    /// request-level config should override speech synthesizer settings defined at
    /// agent-level.
    ///
    /// If unspecified or empty, [output_audio_config][google.cloud.dialogflow.v2.StreamingDetectIntentRequest.output_audio_config] replaces the agent-level
    /// config in its entirety.
    #[prost(message, optional, tag = "7")]
    pub output_audio_config_mask: ::std::option::Option<::prost_types::FieldMask>,
    /// The input audio content to be recognized. Must be sent if
    /// `query_input` was set to a streaming input audio config. The complete audio
    /// over all streaming messages must not exceed 1 minute.
    #[prost(bytes, tag = "6")]
    pub input_audio: std::vec::Vec<u8>,
}
/// The top-level message returned from the
/// `StreamingDetectIntent` method.
///
/// Multiple response messages can be returned in order:
///
/// 1.  If the input was set to streaming audio, the first one or more messages
///     contain `recognition_result`. Each `recognition_result` represents a more
///     complete transcript of what the user said. The last `recognition_result`
///     has `is_final` set to `true`.
///
/// 2.  The next message contains `response_id`, `query_result`
///     and optionally `webhook_status` if a WebHook was called.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingDetectIntentResponse {
    /// The unique identifier of the response. It can be used to
    /// locate a response in the training example set or for reporting issues.
    #[prost(string, tag = "1")]
    pub response_id: std::string::String,
    /// The result of speech recognition.
    #[prost(message, optional, tag = "2")]
    pub recognition_result: ::std::option::Option<StreamingRecognitionResult>,
    /// The result of the conversational query or event processing.
    #[prost(message, optional, tag = "3")]
    pub query_result: ::std::option::Option<QueryResult>,
    /// Specifies the status of the webhook request.
    #[prost(message, optional, tag = "4")]
    pub webhook_status: ::std::option::Option<super::super::super::rpc::Status>,
    /// The audio data bytes encoded as specified in the request.
    /// Note: The output audio is generated based on the values of default platform
    /// text responses found in the `query_result.fulfillment_messages` field. If
    /// multiple default text responses exist, they will be concatenated when
    /// generating audio. If no default platform text responses exist, the
    /// generated audio content will be empty.
    #[prost(bytes, tag = "5")]
    pub output_audio: std::vec::Vec<u8>,
    /// The config used by the speech synthesizer to generate the output audio.
    #[prost(message, optional, tag = "6")]
    pub output_audio_config: ::std::option::Option<OutputAudioConfig>,
}
/// Contains a speech recognition result corresponding to a portion of the audio
/// that is currently being processed or an indication that this is the end
/// of the single requested utterance.
///
/// Example:
///
/// 1.  transcript: "tube"
///
/// 2.  transcript: "to be a"
///
/// 3.  transcript: "to be"
///
/// 4.  transcript: "to be or not to be"
///     is_final: true
///
/// 5.  transcript: " that's"
///
/// 6.  transcript: " that is"
///
/// 7.  message_type: `END_OF_SINGLE_UTTERANCE`
///
/// 8.  transcript: " that is the question"
///     is_final: true
///
/// Only two of the responses contain final results (#4 and #8 indicated by
/// `is_final: true`). Concatenating these generates the full transcript: "to be
/// or not to be that is the question".
///
/// In each response we populate:
///
/// *  for `TRANSCRIPT`: `transcript` and possibly `is_final`.
///
/// *  for `END_OF_SINGLE_UTTERANCE`: only `message_type`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamingRecognitionResult {
    /// Type of the result message.
    #[prost(enumeration = "streaming_recognition_result::MessageType", tag = "1")]
    pub message_type: i32,
    /// Transcript text representing the words that the user spoke.
    /// Populated if and only if `message_type` = `TRANSCRIPT`.
    #[prost(string, tag = "2")]
    pub transcript: std::string::String,
    /// If `false`, the `StreamingRecognitionResult` represents an
    /// interim result that may change. If `true`, the recognizer will not return
    /// any further hypotheses about this piece of the audio. May only be populated
    /// for `message_type` = `TRANSCRIPT`.
    #[prost(bool, tag = "3")]
    pub is_final: bool,
    /// The Speech confidence between 0.0 and 1.0 for the current portion of audio.
    /// A higher number indicates an estimated greater likelihood that the
    /// recognized words are correct. The default of 0.0 is a sentinel value
    /// indicating that confidence was not set.
    ///
    /// This field is typically only provided if `is_final` is true and you should
    /// not rely on it being accurate or even set.
    #[prost(float, tag = "4")]
    pub confidence: f32,
    /// Word-specific information for the words recognized by Speech in
    /// [transcript][google.cloud.dialogflow.v2.StreamingRecognitionResult.transcript]. Populated if and only if `message_type` = `TRANSCRIPT` and
    /// [InputAudioConfig.enable_word_info] is set.
    #[prost(message, repeated, tag = "7")]
    pub speech_word_info: ::std::vec::Vec<SpeechWordInfo>,
    /// Time offset of the end of this Speech recognition result relative to the
    /// beginning of the audio. Only populated for `message_type` = `TRANSCRIPT`.
    #[prost(message, optional, tag = "8")]
    pub speech_end_offset: ::std::option::Option<::prost_types::Duration>,
}
pub mod streaming_recognition_result {
    /// Type of the response message.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MessageType {
        /// Not specified. Should never be used.
        Unspecified = 0,
        /// Message contains a (possibly partial) transcript.
        Transcript = 1,
        /// Event indicates that the server has detected the end of the user's speech
        /// utterance and expects no additional inputs.
        /// Therefore, the server will not process additional audio (although it may subsequently return additional results). The
        /// client should stop sending additional audio data, half-close the gRPC
        /// connection, and wait for any additional results until the server closes
        /// the gRPC connection. This message is only sent if `single_utterance` was
        /// set to `true`, and is not used otherwise.
        EndOfSingleUtterance = 2,
    }
}
/// Represents the natural language text to be processed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextInput {
    /// Required. The UTF-8 encoded natural language text to be processed.
    /// Text length must not exceed 256 characters.
    #[prost(string, tag = "1")]
    pub text: std::string::String,
    /// Required. The language of this conversational query. See [Language
    /// Support](https://cloud.google.com/dialogflow/docs/reference/language)
    /// for a list of the currently supported language codes. Note that queries in
    /// the same session do not necessarily need to specify the same language.
    #[prost(string, tag = "2")]
    pub language_code: std::string::String,
}
/// Events allow for matching intents by event name instead of the natural
/// language input. For instance, input `<event: { name: "welcome_event",
/// parameters: { name: "Sam" } }>` can trigger a personalized welcome response.
/// The parameter `name` may be used by the agent in the response:
/// `"Hello #welcome_event.name! What can I do for you today?"`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventInput {
    /// Required. The unique identifier of the event.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// The collection of parameters associated with the event.
    ///
    /// Depending on your protocol or client library language, this is a
    /// map, associative array, symbol table, dictionary, or JSON object
    /// composed of a collection of (MapKey, MapValue) pairs:
    ///
    /// -   MapKey type: string
    /// -   MapKey value: parameter name
    /// -   MapValue type:
    ///     -   If parameter's entity type is a composite entity: map
    ///     -   Else: string or number, depending on parameter value type
    /// -   MapValue value:
    ///     -   If parameter's entity type is a composite entity:
    ///         map from composite entity property names to property values
    ///     -   Else: parameter value
    #[prost(message, optional, tag = "2")]
    pub parameters: ::std::option::Option<::prost_types::Struct>,
    /// Required. The language of this query. See [Language
    /// Support](https://cloud.google.com/dialogflow/docs/reference/language)
    /// for a list of the currently supported language codes. Note that queries in
    /// the same session do not necessarily need to specify the same language.
    #[prost(string, tag = "3")]
    pub language_code: std::string::String,
}
/// Configures the types of sentiment analysis to perform.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SentimentAnalysisRequestConfig {
    /// Instructs the service to perform sentiment analysis on
    /// `query_text`. If not provided, sentiment analysis is not performed on
    /// `query_text`.
    #[prost(bool, tag = "1")]
    pub analyze_query_text_sentiment: bool,
}
/// The result of sentiment analysis as configured by
/// `sentiment_analysis_request_config`.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SentimentAnalysisResult {
    /// The sentiment analysis result for `query_text`.
    #[prost(message, optional, tag = "1")]
    pub query_text_sentiment: ::std::option::Option<Sentiment>,
}
/// The sentiment, such as positive/negative feeling or association, for a unit
/// of analysis, such as the query text.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sentiment {
    /// Sentiment score between -1.0 (negative sentiment) and 1.0 (positive
    /// sentiment).
    #[prost(float, tag = "1")]
    pub score: f32,
    /// A non-negative number in the [0, +inf) range, which represents the absolute
    /// magnitude of sentiment, regardless of score (positive or negative).
    #[prost(float, tag = "2")]
    pub magnitude: f32,
}
#[doc = r" Generated client implementations."]
pub mod sessions_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " A session represents an interaction with a user. You retrieve user input"]
    #[doc = " and pass it to the [DetectIntent][google.cloud.dialogflow.v2.Sessions.DetectIntent] (or"]
    #[doc = " [StreamingDetectIntent][google.cloud.dialogflow.v2.Sessions.StreamingDetectIntent]) method to determine"]
    #[doc = " user intent and respond."]
    pub struct SessionsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SessionsClient<tonic::transport::Channel> {
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
    impl<T> SessionsClient<T>
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
        #[doc = " Processes a natural language query and returns structured, actionable data"]
        #[doc = " as a result. This method is not idempotent, because it may cause contexts"]
        #[doc = " and session entity types to be updated, which in turn might affect"]
        #[doc = " results of future queries."]
        pub async fn detect_intent(
            &mut self,
            request: impl tonic::IntoRequest<super::DetectIntentRequest>,
        ) -> Result<tonic::Response<super::DetectIntentResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/google.cloud.dialogflow.v2.Sessions/DetectIntent",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Processes a natural language query in audio format in a streaming fashion"]
        #[doc = " and returns structured, actionable data as a result. This method is only"]
        #[doc = " available via the gRPC API (not REST)."]
        pub async fn streaming_detect_intent(
            &mut self,
            request: impl tonic::IntoStreamingRequest<Message = super::StreamingDetectIntentRequest>,
        ) -> Result<
            tonic::Response<tonic::codec::Streaming<super::StreamingDetectIntentResponse>>,
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
                "/google.cloud.dialogflow.v2.Sessions/StreamingDetectIntent",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
    }
    impl<T: Clone> Clone for SessionsClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for SessionsClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "SessionsClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod sessions_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with SessionsServer."]
    #[async_trait]
    pub trait Sessions: Send + Sync + 'static {
        #[doc = " Processes a natural language query and returns structured, actionable data"]
        #[doc = " as a result. This method is not idempotent, because it may cause contexts"]
        #[doc = " and session entity types to be updated, which in turn might affect"]
        #[doc = " results of future queries."]
        async fn detect_intent(
            &self,
            request: tonic::Request<super::DetectIntentRequest>,
        ) -> Result<tonic::Response<super::DetectIntentResponse>, tonic::Status>;
        #[doc = "Server streaming response type for the StreamingDetectIntent method."]
        type StreamingDetectIntentStream: Stream<Item = Result<super::StreamingDetectIntentResponse, tonic::Status>>
            + Send
            + Sync
            + 'static;
        #[doc = " Processes a natural language query in audio format in a streaming fashion"]
        #[doc = " and returns structured, actionable data as a result. This method is only"]
        #[doc = " available via the gRPC API (not REST)."]
        async fn streaming_detect_intent(
            &self,
            request: tonic::Request<tonic::Streaming<super::StreamingDetectIntentRequest>>,
        ) -> Result<tonic::Response<Self::StreamingDetectIntentStream>, tonic::Status>;
    }
    #[doc = " A session represents an interaction with a user. You retrieve user input"]
    #[doc = " and pass it to the [DetectIntent][google.cloud.dialogflow.v2.Sessions.DetectIntent] (or"]
    #[doc = " [StreamingDetectIntent][google.cloud.dialogflow.v2.Sessions.StreamingDetectIntent]) method to determine"]
    #[doc = " user intent and respond."]
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct SessionsServer<T: Sessions> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: Sessions> SessionsServer<T> {
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
    impl<T, B> Service<http::Request<B>> for SessionsServer<T>
    where
        T: Sessions,
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
                "/google.cloud.dialogflow.v2.Sessions/DetectIntent" => {
                    #[allow(non_camel_case_types)]
                    struct DetectIntentSvc<T: Sessions>(pub Arc<T>);
                    impl<T: Sessions> tonic::server::UnaryService<super::DetectIntentRequest> for DetectIntentSvc<T> {
                        type Response = super::DetectIntentResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DetectIntentRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.detect_intent(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DetectIntentSvc(inner);
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
                "/google.cloud.dialogflow.v2.Sessions/StreamingDetectIntent" => {
                    #[allow(non_camel_case_types)]
                    struct StreamingDetectIntentSvc<T: Sessions>(pub Arc<T>);
                    impl<T: Sessions>
                        tonic::server::StreamingService<super::StreamingDetectIntentRequest>
                        for StreamingDetectIntentSvc<T>
                    {
                        type Response = super::StreamingDetectIntentResponse;
                        type ResponseStream = T::StreamingDetectIntentStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<super::StreamingDetectIntentRequest>,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.streaming_detect_intent(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = StreamingDetectIntentSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.streaming(method, req).await;
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
    impl<T: Sessions> Clone for SessionsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: Sessions> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Sessions> tonic::transport::NamedService for SessionsServer<T> {
        const NAME: &'static str = "google.cloud.dialogflow.v2.Sessions";
    }
}
/// The request message for a webhook call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebhookRequest {
    /// The unique identifier of detectIntent request session.
    /// Can be used to identify end-user inside webhook implementation.
    /// Format: `projects/<Project ID>/agent/sessions/<Session ID>`, or
    /// `projects/<Project ID>/agent/environments/<Environment ID>/users/<User
    /// ID>/sessions/<Session ID>`.
    #[prost(string, tag = "4")]
    pub session: std::string::String,
    /// The unique identifier of the response. Contains the same value as
    /// `[Streaming]DetectIntentResponse.response_id`.
    #[prost(string, tag = "1")]
    pub response_id: std::string::String,
    /// The result of the conversational query or event processing. Contains the
    /// same value as `[Streaming]DetectIntentResponse.query_result`.
    #[prost(message, optional, tag = "2")]
    pub query_result: ::std::option::Option<QueryResult>,
    /// Optional. The contents of the original request that was passed to
    /// `[Streaming]DetectIntent` call.
    #[prost(message, optional, tag = "3")]
    pub original_detect_intent_request: ::std::option::Option<OriginalDetectIntentRequest>,
}
/// The response message for a webhook call.
///
/// This response is validated by the Dialogflow server. If validation fails,
/// an error will be returned in the [QueryResult.diagnostic_info][google.cloud.dialogflow.v2.QueryResult.diagnostic_info] field.
/// Setting JSON fields to an empty value with the wrong type is a common error.
/// To avoid this error:
///
/// - Use `""` for empty strings
/// - Use `{}` or `null` for empty objects
/// - Use `[]` or `null` for empty arrays
///
/// For more information, see the
/// [Protocol Buffers Language
/// Guide](https://developers.google.com/protocol-buffers/docs/proto3#json).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebhookResponse {
    /// Optional. The text response message intended for the end-user.
    /// It is recommended to use `fulfillment_messages.text.text[0]` instead.
    /// When provided, Dialogflow uses this field to populate
    /// [QueryResult.fulfillment_text][google.cloud.dialogflow.v2.QueryResult.fulfillment_text] sent to the integration or API caller.
    #[prost(string, tag = "1")]
    pub fulfillment_text: std::string::String,
    /// Optional. The rich response messages intended for the end-user.
    /// When provided, Dialogflow uses this field to populate
    /// [QueryResult.fulfillment_messages][google.cloud.dialogflow.v2.QueryResult.fulfillment_messages] sent to the integration or API caller.
    #[prost(message, repeated, tag = "2")]
    pub fulfillment_messages: ::std::vec::Vec<intent::Message>,
    /// Optional. A custom field used to identify the webhook source.
    /// Arbitrary strings are supported.
    /// When provided, Dialogflow uses this field to populate
    /// [QueryResult.webhook_source][google.cloud.dialogflow.v2.QueryResult.webhook_source] sent to the integration or API caller.
    #[prost(string, tag = "3")]
    pub source: std::string::String,
    /// Optional. This field can be used to pass custom data from your webhook to the
    /// integration or API caller. Arbitrary JSON objects are supported.
    /// When provided, Dialogflow uses this field to populate
    /// [QueryResult.webhook_payload][google.cloud.dialogflow.v2.QueryResult.webhook_payload] sent to the integration or API caller.
    /// This field is also used by the
    /// [Google Assistant
    /// integration](https://cloud.google.com/dialogflow/docs/integrations/aog)
    /// for rich response messages.
    /// See the format definition at [Google Assistant Dialogflow webhook
    /// format](https://developers.google.com/assistant/actions/build/json/dialogflow-webhook-json)
    #[prost(message, optional, tag = "4")]
    pub payload: ::std::option::Option<::prost_types::Struct>,
    /// Optional. The collection of output contexts that will overwrite currently
    /// active contexts for the session and reset their lifespans.
    /// When provided, Dialogflow uses this field to populate
    /// [QueryResult.output_contexts][google.cloud.dialogflow.v2.QueryResult.output_contexts] sent to the integration or API caller.
    #[prost(message, repeated, tag = "5")]
    pub output_contexts: ::std::vec::Vec<Context>,
    /// Optional. Invokes the supplied events.
    /// When this field is set, Dialogflow ignores the `fulfillment_text`,
    /// `fulfillment_messages`, and `payload` fields.
    #[prost(message, optional, tag = "6")]
    pub followup_event_input: ::std::option::Option<EventInput>,
    /// Optional. Additional session entity types to replace or extend developer
    /// entity types with. The entity synonyms apply to all languages and persist
    /// for the session. Setting this data from a webhook overwrites
    /// the session entity types that have been set using `detectIntent`,
    /// `streamingDetectIntent` or [SessionEntityType][google.cloud.dialogflow.v2.SessionEntityType] management methods.
    #[prost(message, repeated, tag = "10")]
    pub session_entity_types: ::std::vec::Vec<SessionEntityType>,
}
/// Represents the contents of the original request that was passed to
/// the `[Streaming]DetectIntent` call.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OriginalDetectIntentRequest {
    /// The source of this request, e.g., `google`, `facebook`, `slack`. It is set
    /// by Dialogflow-owned servers.
    #[prost(string, tag = "1")]
    pub source: std::string::String,
    /// Optional. The version of the protocol used for this request.
    /// This field is AoG-specific.
    #[prost(string, tag = "2")]
    pub version: std::string::String,
    /// Optional. This field is set to the value of the `QueryParameters.payload`
    /// field passed in the request. Some integrations that query a Dialogflow
    /// agent may provide additional information in the payload.
    ///
    /// In particular, for the Dialogflow Phone Gateway integration, this field has
    /// the form:
    /// <pre>{
    ///  "telephony": {
    ///    "caller_id": "+18558363987"
    ///  }
    /// }</pre>
    /// Note: The caller ID field (`caller_id`) will be redacted for Standard
    /// Edition agents and populated with the caller ID in [E.164
    /// format](https://en.wikipedia.org/wiki/E.164) for Enterprise Edition agents.
    #[prost(message, optional, tag = "3")]
    pub payload: ::std::option::Option<::prost_types::Struct>,
}
