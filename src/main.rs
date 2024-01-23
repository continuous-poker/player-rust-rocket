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

#[macro_use] extern crate rocket;
use rocket::serde::json::Json;

mod models;
mod logic;

#[post("/", format = "json", data = "<table>")]
fn index(table: Json<models::table::Table>) -> Json<models::bet::Bet> {
    Json(logic::strategy::decide(table))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}