/*
Copyright 2022 The Numaproj Authors.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

// Code generated by Openapi Generator. DO NOT EDIT.

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PulsarSource {
    #[serde(rename = "auth", skip_serializing_if = "Option::is_none")]
    pub auth: Option<Box<crate::models::PulsarAuth>>,
    #[serde(rename = "consumerName")]
    pub consumer_name: String,
    /// Maximum number of messages that are in not yet acked state. Once this limit is crossed, futher read requests will return empty list.
    #[serde(rename = "maxUnack", skip_serializing_if = "Option::is_none")]
    pub max_unack: Option<i64>,
    #[serde(rename = "serverAddr")]
    pub server_addr: String,
    #[serde(rename = "subscriptionName")]
    pub subscription_name: String,
    #[serde(rename = "topic")]
    pub topic: String,
}

impl PulsarSource {
    pub fn new(
        consumer_name: String,
        server_addr: String,
        subscription_name: String,
        topic: String,
    ) -> PulsarSource {
        PulsarSource {
            auth: None,
            consumer_name,
            max_unack: None,
            server_addr,
            subscription_name,
            topic,
        }
    }
}
