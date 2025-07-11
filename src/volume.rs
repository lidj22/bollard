//! Volume API: Create and manage persistent storage that can be attached to containers.
#![allow(deprecated)]

use bytes::Bytes;
use http::request::Builder;
use http_body_util::Full;
use hyper::Method;
use serde_derive::{Deserialize, Serialize};

use std::cmp::Eq;
use std::collections::HashMap;
use std::hash::Hash;

use super::Docker;
use crate::docker::BodyType;
use crate::errors::Error;
use crate::models::*;

/// Parameters used in the [List Volume API](Docker::list_volumes())
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[deprecated(
    since = "0.19.0",
    note = "use the OpenAPI generated bollard::query_parameters::ListVolumesOptions and associated ListVolumesOptionsBuilder"
)]
pub struct ListVolumesOptions<T>
where
    T: Into<String> + Eq + Hash + serde::ser::Serialize,
{
    /// JSON encoded value of the filters (a `map[string][]string`) to process on the volumes list. Available filters:
    ///  - `dangling=<boolean>` When set to `true` (or `1`), returns all volumes that are not in use by a container. When set to `false` (or `0`), only volumes that are in use by one or more containers are returned.
    ///  - `driver=<volume-driver-name>` Matches volumes based on their driver.
    ///  - `label=<key>` or `label=<key>:<value>` Matches volumes based on the presence of a `label` alone or a `label` and a value.
    ///  - `name=<volume-name>` Matches all or part of a volume name.
    #[serde(serialize_with = "crate::docker::serialize_as_json")]
    pub filters: HashMap<T, Vec<T>>,
}

impl<T> From<ListVolumesOptions<T>> for crate::query_parameters::ListVolumesOptions
where
    T: Into<String> + Eq + Hash + serde::ser::Serialize,
{
    fn from(opts: ListVolumesOptions<T>) -> Self {
        crate::query_parameters::ListVolumesOptionsBuilder::default()
            .filters(
                &opts
                    .filters
                    .into_iter()
                    .map(|(k, v)| (k.into(), v.into_iter().map(T::into).collect()))
                    .collect(),
            )
            .build()
    }
}

/// Volume configuration used in the [Create Volume
/// API](Docker::create_volume())
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[deprecated(
    since = "0.19.0",
    note = "use the OpenAPI generated bollard::query_parameters::CreateVolumeOptions and associated CreateVolumeOptionsBuilder"
)]
#[serde(rename_all = "PascalCase")]
pub struct CreateVolumeOptions<T>
where
    T: Into<String> + Eq + Hash + serde::ser::Serialize,
{
    /// The new volume's name. If not specified, Docker generates a name.
    pub name: T,
    /// Name of the volume driver to use.
    pub driver: T,
    /// A mapping of driver options and values. These options are passed directly to the driver and
    /// are driver specific.
    pub driver_opts: HashMap<T, T>,
    /// User-defined key/value metadata.
    pub labels: HashMap<T, T>,
}

impl<T> From<CreateVolumeOptions<T>> for VolumeCreateOptions
where
    T: Into<String> + Eq + Hash + serde::ser::Serialize,
{
    fn from(opts: CreateVolumeOptions<T>) -> Self {
        VolumeCreateOptions {
            name: Some(opts.name.into()),
            driver: Some(opts.driver.into()),
            driver_opts: Some(
                opts.driver_opts
                    .into_iter()
                    .map(|(k, v)| (k.into(), v.into()))
                    .collect(),
            ),
            labels: Some(
                opts.labels
                    .into_iter()
                    .map(|(k, v)| (k.into(), v.into()))
                    .collect(),
            ),
            ..Default::default()
        }
    }
}

/// Parameters used in the [Remove Volume API](super::Docker::remove_volume())
#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize)]
#[deprecated(
    since = "0.19.0",
    note = "use the OpenAPI generated bollard::query_parameters::RemoveVolumeOptions and associated RemoveVolumeOptionsBuilder"
)]
#[serde(rename_all = "camelCase")]
pub struct RemoveVolumeOptions {
    /// Force the removal of the volume.
    pub force: bool,
}

impl From<RemoveVolumeOptions> for crate::query_parameters::RemoveVolumeOptions {
    fn from(opts: RemoveVolumeOptions) -> Self {
        crate::query_parameters::RemoveVolumeOptionsBuilder::default()
            .force(opts.force)
            .build()
    }
}

/// Parameters used in the [Prune Volumes API](Docker::prune_volumes())
///
/// ## Examples
///
/// ```rust
/// use bollard::volume::PruneVolumesOptions;
///
/// use std::collections::HashMap;
///
/// let mut filters = HashMap::new();
/// filters.insert("label!", vec!["maintainer=some_maintainer"]);
///
/// PruneVolumesOptions{
///     filters
/// };
/// ```
///
/// ```rust
/// # use bollard::volume::PruneVolumesOptions;
/// # use std::default::Default;
///
/// PruneVolumesOptions::<&str>{
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, Default, PartialEq, Serialize)]
#[deprecated(
    since = "0.19.0",
    note = "use the OpenAPI generated bollard::query_parameters::PruneVolumesOptions and associated PruneVolumesOptionsBuilder"
)]
pub struct PruneVolumesOptions<T>
where
    T: Into<String> + Eq + Hash + serde::ser::Serialize,
{
    /// Filters to process on the prune list, encoded as JSON.
    ///  - `label` (`label=<key>`, `label=<key>=<value>`, `label!=<key>`, or
    ///    `label!=<key>=<value>`) Prune volumes with (or without, in case `label!=...` is used) the
    ///    specified labels.
    #[serde(serialize_with = "crate::docker::serialize_as_json")]
    pub filters: HashMap<T, Vec<T>>,
}

impl<T> From<PruneVolumesOptions<T>> for crate::query_parameters::PruneVolumesOptions
where
    T: Into<String> + Eq + Hash + serde::ser::Serialize,
{
    fn from(opts: PruneVolumesOptions<T>) -> Self {
        crate::query_parameters::PruneVolumesOptionsBuilder::default()
            .filters(
                &opts
                    .filters
                    .into_iter()
                    .map(|(k, v)| (k.into(), v.into_iter().map(T::into).collect()))
                    .collect(),
            )
            .build()
    }
}

impl Docker {
    /// ---
    ///
    /// # List volumes
    ///
    /// # Arguments
    ///
    ///  - [List Volumes Options](ListVolumesOptions) struct.
    ///
    /// # Returns
    ///
    ///  - A [Volume List Response]VolumeListResponse) struct, wrapped in a
    ///    Future.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use bollard::Docker;
    /// # let docker = Docker::connect_with_http_defaults().unwrap();
    ///
    /// use bollard::volume::ListVolumesOptions;
    ///
    /// use std::collections::HashMap;
    ///
    /// let mut filters = HashMap::new();
    /// filters.insert("dangling", vec!("1"));
    ///
    /// let options = ListVolumesOptions {
    ///     filters,
    /// };
    ///
    /// docker.list_volumes(Some(options));
    /// ```
    pub async fn list_volumes(
        &self,
        options: Option<impl Into<crate::query_parameters::ListVolumesOptions>>,
    ) -> Result<VolumeListResponse, Error> {
        let url = "/volumes";

        let req = self.build_request(
            url,
            Builder::new().method(Method::GET),
            options.map(Into::into),
            Ok(BodyType::Left(Full::new(Bytes::new()))),
        );

        self.process_into_value(req).await
    }

    /// ---
    ///
    /// # Create Volume
    ///
    /// Create a new volume.
    ///
    /// # Arguments
    ///
    ///  - [Create Volume Options](CreateVolumeOptions) struct.
    ///
    /// # Returns
    ///
    ///  - A [Volume](Volume) struct, wrapped in a
    ///    Future.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use bollard::Docker;
    /// # let docker = Docker::connect_with_http_defaults().unwrap();
    ///
    /// use bollard::volume::CreateVolumeOptions;
    ///
    /// use std::default::Default;
    ///
    /// let config = CreateVolumeOptions {
    ///     name: "certs",
    ///     ..Default::default()
    /// };
    ///
    /// docker.create_volume(config);
    /// ```
    pub async fn create_volume(
        &self,
        config: impl Into<VolumeCreateOptions>,
    ) -> Result<Volume, Error> {
        let url = "/volumes/create";

        let req = self.build_request(
            url,
            Builder::new().method(Method::POST),
            None::<String>,
            Docker::serialize_payload(Some(config.into())),
        );

        self.process_into_value(req).await
    }

    /// ---
    ///
    /// # Inspect a Volume
    ///
    /// # Arguments
    ///
    ///  - Volume name as a string slice.
    ///
    /// # Returns
    ///
    ///  - A [Volume](Volume) struct, wrapped in a
    ///    Future.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use bollard::Docker;
    /// # let docker = Docker::connect_with_http_defaults().unwrap();
    ///
    /// docker.inspect_volume("my_volume_name");
    /// ```
    pub async fn inspect_volume(&self, volume_name: &str) -> Result<Volume, Error> {
        let url = format!("/volumes/{volume_name}");

        let req = self.build_request(
            &url,
            Builder::new().method(Method::GET),
            None::<String>,
            Ok(BodyType::Left(Full::new(Bytes::new()))),
        );

        self.process_into_value(req).await
    }

    /// ---
    ///
    /// # Remove a Volume
    ///
    /// # Arguments
    ///
    ///  - Volume name as a string slice.
    ///
    /// # Arguments
    ///
    ///  - [Remove Volume Options](RemoveVolumeOptions) struct.
    ///
    /// # Returns
    ///
    ///  - unit type `()`, wrapped in a Future.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use bollard::Docker;
    /// # let docker = Docker::connect_with_http_defaults().unwrap();
    ///
    /// use bollard::volume::RemoveVolumeOptions;
    ///
    /// let options = RemoveVolumeOptions {
    ///     force: true,
    /// };
    ///
    /// docker.remove_volume("my_volume_name", Some(options));
    /// ```
    pub async fn remove_volume(
        &self,
        volume_name: &str,
        options: Option<impl Into<crate::query_parameters::RemoveVolumeOptions>>,
    ) -> Result<(), Error> {
        let url = format!("/volumes/{volume_name}");

        let req = self.build_request(
            &url,
            Builder::new().method(Method::DELETE),
            options.map(Into::into),
            Ok(BodyType::Left(Full::new(Bytes::new()))),
        );

        self.process_into_unit(req).await
    }

    /// ---
    ///
    /// # Prune Volumes
    ///
    /// Delete unused volumes.
    ///
    /// # Arguments
    ///
    ///  - A [Prune Volumes Options](PruneVolumesOptions) struct.
    ///
    /// # Returns
    ///
    ///  - A [Volume Prune Response](VolumePruneResponse) struct.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use bollard::Docker;
    /// # let docker = Docker::connect_with_http_defaults().unwrap();
    ///
    /// use bollard::volume::PruneVolumesOptions;
    ///
    /// use std::collections::HashMap;
    ///
    /// let mut filters = HashMap::new();
    /// filters.insert("label", vec!["maintainer=some_maintainer"]);
    ///
    /// let options = PruneVolumesOptions {
    ///     filters,
    /// };
    ///
    /// docker.prune_volumes(Some(options));
    /// ```
    pub async fn prune_volumes(
        &self,
        options: Option<impl Into<crate::query_parameters::PruneVolumesOptions>>,
    ) -> Result<VolumePruneResponse, Error> {
        let url = "/volumes/prune";

        let req = self.build_request(
            url,
            Builder::new().method(Method::POST),
            options.map(Into::into),
            Ok(BodyType::Left(Full::new(Bytes::new()))),
        );

        self.process_into_value(req).await
    }
}
