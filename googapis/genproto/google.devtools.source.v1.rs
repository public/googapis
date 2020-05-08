/// A SourceContext is a reference to a tree of files. A SourceContext together
/// with a path point to a unique revision of a single file or directory.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SourceContext {
    /// A SourceContext can refer any one of the following types of repositories.
    #[prost(oneof = "source_context::Context", tags = "1, 2, 3, 6")]
    pub context: ::std::option::Option<source_context::Context>,
}
pub mod source_context {
    /// A SourceContext can refer any one of the following types of repositories.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Context {
        /// A SourceContext referring to a revision in a cloud repo.
        #[prost(message, tag = "1")]
        CloudRepo(super::CloudRepoSourceContext),
        /// A SourceContext referring to a snapshot in a cloud workspace.
        #[prost(message, tag = "2")]
        CloudWorkspace(super::CloudWorkspaceSourceContext),
        /// A SourceContext referring to a Gerrit project.
        #[prost(message, tag = "3")]
        Gerrit(super::GerritSourceContext),
        /// A SourceContext referring to any third party Git repo (e.g. GitHub).
        #[prost(message, tag = "6")]
        Git(super::GitSourceContext),
    }
}
/// An ExtendedSourceContext is a SourceContext combined with additional
/// details describing the context.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtendedSourceContext {
    /// Any source context.
    #[prost(message, optional, tag = "1")]
    pub context: ::std::option::Option<SourceContext>,
    /// Labels with user defined metadata.
    #[prost(map = "string, string", tag = "2")]
    pub labels: ::std::collections::HashMap<std::string::String, std::string::String>,
}
/// An alias to a repo revision.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AliasContext {
    /// The alias kind.
    #[prost(enumeration = "alias_context::Kind", tag = "1")]
    pub kind: i32,
    /// The alias name.
    #[prost(string, tag = "2")]
    pub name: std::string::String,
}
pub mod alias_context {
    /// The type of an Alias.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Kind {
        /// Do not use.
        Any = 0,
        /// Git tag
        Fixed = 1,
        /// Git branch
        Movable = 2,
        /// OTHER is used to specify non-standard aliases, those not of the kinds
        /// above. For example, if a Git repo has a ref named "refs/foo/bar", it
        /// is considered to be of kind OTHER.
        Other = 4,
    }
}
/// A CloudRepoSourceContext denotes a particular revision in a cloud
/// repo (a repo hosted by the Google Cloud Platform).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudRepoSourceContext {
    /// The ID of the repo.
    #[prost(message, optional, tag = "1")]
    pub repo_id: ::std::option::Option<RepoId>,
    /// A revision in a cloud repository can be identified by either its revision
    /// ID or its Alias.
    #[prost(oneof = "cloud_repo_source_context::Revision", tags = "2, 3, 4")]
    pub revision: ::std::option::Option<cloud_repo_source_context::Revision>,
}
pub mod cloud_repo_source_context {
    /// A revision in a cloud repository can be identified by either its revision
    /// ID or its Alias.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Revision {
        /// A revision ID.
        #[prost(string, tag = "2")]
        RevisionId(std::string::String),
        /// The name of an alias (branch, tag, etc.).
        #[prost(string, tag = "3")]
        AliasName(std::string::String),
        /// An alias, which may be a branch or tag.
        #[prost(message, tag = "4")]
        AliasContext(super::AliasContext),
    }
}
/// A CloudWorkspaceSourceContext denotes a workspace at a particular snapshot.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudWorkspaceSourceContext {
    /// The ID of the workspace.
    #[prost(message, optional, tag = "1")]
    pub workspace_id: ::std::option::Option<CloudWorkspaceId>,
    /// The ID of the snapshot.
    /// An empty snapshot_id refers to the most recent snapshot.
    #[prost(string, tag = "2")]
    pub snapshot_id: std::string::String,
}
/// A SourceContext referring to a Gerrit project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GerritSourceContext {
    /// The URI of a running Gerrit instance.
    #[prost(string, tag = "1")]
    pub host_uri: std::string::String,
    /// The full project name within the host. Projects may be nested, so
    /// "project/subproject" is a valid project name.
    /// The "repo name" is hostURI/project.
    #[prost(string, tag = "2")]
    pub gerrit_project: std::string::String,
    /// A revision in a Gerrit project can be identified by either its revision ID
    /// or its alias.
    #[prost(oneof = "gerrit_source_context::Revision", tags = "3, 4, 5")]
    pub revision: ::std::option::Option<gerrit_source_context::Revision>,
}
pub mod gerrit_source_context {
    /// A revision in a Gerrit project can be identified by either its revision ID
    /// or its alias.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Revision {
        /// A revision (commit) ID.
        #[prost(string, tag = "3")]
        RevisionId(std::string::String),
        /// The name of an alias (branch, tag, etc.).
        #[prost(string, tag = "4")]
        AliasName(std::string::String),
        /// An alias, which may be a branch or tag.
        #[prost(message, tag = "5")]
        AliasContext(super::AliasContext),
    }
}
/// A GitSourceContext denotes a particular revision in a third party Git
/// repository (e.g. GitHub).
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitSourceContext {
    /// Git repository URL.
    #[prost(string, tag = "1")]
    pub url: std::string::String,
    /// Git commit hash.
    /// required.
    #[prost(string, tag = "2")]
    pub revision_id: std::string::String,
}
/// A unique identifier for a cloud repo.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepoId {
    /// A cloud repository can be identified by either its project ID and
    /// repository name combination, or its globally unique identifier.
    #[prost(oneof = "repo_id::Id", tags = "1, 2")]
    pub id: ::std::option::Option<repo_id::Id>,
}
pub mod repo_id {
    /// A cloud repository can be identified by either its project ID and
    /// repository name combination, or its globally unique identifier.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Id {
        /// A combination of a project ID and a repo name.
        #[prost(message, tag = "1")]
        ProjectRepoId(super::ProjectRepoId),
        /// A server-assigned, globally unique identifier.
        #[prost(string, tag = "2")]
        Uid(std::string::String),
    }
}
/// Selects a repo using a Google Cloud Platform project ID
/// (e.g. winged-cargo-31) and a repo name within that project.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectRepoId {
    /// The ID of the project.
    #[prost(string, tag = "1")]
    pub project_id: std::string::String,
    /// The name of the repo. Leave empty for the default repo.
    #[prost(string, tag = "2")]
    pub repo_name: std::string::String,
}
/// A CloudWorkspaceId is a unique identifier for a cloud workspace.
/// A cloud workspace is a place associated with a repo where modified files
/// can be stored before they are committed.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CloudWorkspaceId {
    /// The ID of the repo containing the workspace.
    #[prost(message, optional, tag = "1")]
    pub repo_id: ::std::option::Option<RepoId>,
    /// The unique name of the workspace within the repo.  This is the name
    /// chosen by the client in the Source API's CreateWorkspace method.
    #[prost(string, tag = "2")]
    pub name: std::string::String,
}
