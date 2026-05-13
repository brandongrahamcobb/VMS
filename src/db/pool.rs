/* pool.rs
 * The purpose of this module is to serve the database pool.
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
use crate::db::error::DatabaseError::DieselError;
use crate::runtime::error::RuntimeError;
use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub async fn spawn_db<F, T>(pool: DbPool, f: F) -> Result<T, RuntimeError>
where
    F: FnOnce(&mut PgConnection) -> Result<T, diesel::result::Error> + Send + 'static,
    T: Send + 'static,
{
    tokio::task::spawn_blocking(move || {
        let mut conn = pool.get().map_err(|_| diesel::result::Error::NotFound)?;
        f(&mut conn)
    })
    .await
    .map_err(RuntimeError::JoinError)?
    .map_err(DieselError)
    .map_err(RuntimeError::from)
}
