/*
 * Copyright © 2020 - 2024 Jan Kreutzfeld
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Eq)]
pub enum Rank {
    #[serde(alias = "A")]
    A,
    #[serde(alias = "K")]
    K,
    #[serde(alias = "Q")]
    Q,
    #[serde(alias = "J")]
    J,
    #[serde(alias = "10")]
    _10,
    #[serde(alias = "9")]
    _9,
    #[serde(alias = "8")]
    _8,
    #[serde(alias = "7")]
    _7,
    #[serde(alias = "6")]
    _6,
    #[serde(alias = "5")]
    _5,
    #[serde(alias = "4")]
    _4,
    #[serde(alias = "3")]
    _3,
    #[serde(alias = "2")]
    _2
}