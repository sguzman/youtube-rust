use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct Youtube {}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct ResponseContext {
    pub serviceTrackingParams: Vec<ServiceTrackingParams>,
    pub maxAgeSeconds: u16,
    pub mainAppWebResponseContext: MainAppWebResponseContext,
    pub webResponseContextExtensionData: webResponseContextExtensionData,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct ServiceTrackingParams {
    pub service: String,
    pub params: Vec<Param>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct MainAppWebResponseContext {
    pub loggedOut: bool,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct webResponseContextExtensionData {
    pub ytConfigData: YtConfigData,
    pub hasDecorated: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct YtConfigData {
    pub visitorData: String,
    pub rootVisualElementType: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct Param {
    pub key: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct Contents {
    pub twoColumnBrowseResultsRenderer: TwoColumnBrowseResultsRenderer,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct TwoColumnBrowseResultsRenderer {
    pub tabs: Vec<Tab>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct Tab {
    pub tabRenderer: TabRenderer,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct TabRenderer {
    pub endpoint: Endpoint,
    pub title: Title,
    pub selected: bool,
    pub content: Content,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct Endpoint {
    pub clickTrackingParams: String,
    pub commandMetadata: CommandMetadata,
    pub browseEndpoint: BrowseEndpoint,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct CommandMetadata {
    pub webCommandMetadata: WebCommandMetadata,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct WebCommandMetadata {
    pub url: String,
    pub webPageType: String,
    pub rootVe: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct BrowseEndpoint {
    pub browseId: String,
    pub canonicalBaseUrl: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct Title {
    pub runs: Vec<Run>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct Run {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct Content {
    pub sectionListRenderer: SectionListRenderer,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct SectionListRenderer {
    pub contents: Vec<Content2>,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct Content2 {
    pub itemSectionRenderer: ItemSectionRenderer,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct ItemSectionRenderer {
    pub contents: Vec<Content3>,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct Content3 {
    pub gridRenderer: GridRenderer,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct GridRenderer {
    pub items: Vec<Item>,
    pub trackingParams: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct Item {
    pub gridVideoRenderer: GridVideoRenderer,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub struct Thumbnail {
    pub url: String,
    pub width: u16,
    pub height: u16,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Hash, Eq)]
pub 