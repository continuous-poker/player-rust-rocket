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

#[derive(Debug, Deserialize)]
pub struct Table {
    #[serde(alias = "communityCards")]
    pub community_cards: Vec<crate::models::card::Card>,
    pub players: Vec<crate::models::player::Player>,
    pub round: i32,
    #[serde(alias = "smallBlind")]
    pub small_blind: i32,
    #[serde(alias = "minimumBet")]
    pub minimum_bet: i32,
    #[serde(alias = "minimumRaise")]
    pub minimum_raise: i32,
    pub pot: i32,
    #[serde(alias = "activePlayer")]
    pub active_player: i32,
    #[serde(alias = "currentDealer")]
    pub current_dealer: i32,
}