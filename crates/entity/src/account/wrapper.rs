/* account/wrapper.rs
 * The purpose of this module is to provide an account wrapper.
 *
 * Copyright (C) 2026  https://github.com/brandongrahamcobb/VMS.git
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::account::model::AccountModel;
use crate::character::wrapper::Character;

pub struct Account {
    pub model: AccountModel,
    pub chars: Vec<Character>,
}

#[derive(Clone)]
pub enum StatusCode {
    Failed(FailedCode),
    Pending(PendingCode),
    Success(SuccessCode),
}

#[derive(Clone)]
pub enum PendingCode {
    PendingTOS = 23,
}

#[derive(Clone)]
pub enum SuccessCode {
    Success = 0,
}

#[derive(Clone)]
pub enum FailedCode {
    Banned = 2,
    InvalidCredentials = 4,
    UnknownCredentials = 5,
    Playing = 7,
}
