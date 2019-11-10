/*
 * server/user.rs
 *
 * deepwell - Database management and migrations service
 * Copyright (C) 2019 Ammon Smith
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program. If not, see <http://www.gnu.org/licenses/>.
 */

use crate::prelude::*;

use diesel::Connection;

impl Server {
    /// Creates a new user with the given name and email. Returns its ID.
    #[inline]
    pub fn create_user(&self, name: &str, email: &str, password: &str) -> Result<UserId> {
        self.conn.transaction::<_, Error, _>(|| {
            let user_id = self.user.create(name, email)?;
            self.password.set(user_id, password)?;

            Ok(user_id)
        })
    }

    /// Edits data attached to a user with the given ID.
    #[inline]
    pub fn edit_user(&self, id: UserId, changes: UserMetadata) -> Result<()> {
        self.user.edit(id, changes)
    }

    /// Get the model for a user from its ID.
    #[inline]
    pub fn get_user_from_id(&self, id: UserId) -> Result<User> {
        self.user.get_from_id(id)?.ok_or(Error::UserNotFound)
    }

    /// Gets the models for users from their IDs.
    /// Results are returned in the same order as the IDs, and any missing
    /// users give `None` instead.
    #[inline]
    pub fn get_users_from_ids(&self, ids: &[UserId]) -> Result<Vec<Option<User>>> {
        self.user.get_from_ids(ids)
    }

    /// Gets the model for a user from its name.
    #[inline]
    pub fn get_user_from_name(&self, name: &str) -> Result<Option<User>> {
        self.user.get_from_name(name)
    }

    /// Gets the model for a user from its email.
    #[inline]
    pub fn get_user_from_email(&self, email: &str) -> Result<Option<User>> {
        self.user.get_from_email(email)
    }

    /// Marks a user as verified.
    #[inline]
    pub fn verify_user(&self, id: UserId) -> Result<()> {
        self.user.verify(id)
    }

    /// Marks the user as "inactive", effectively deleting them.
    #[inline]
    pub fn mark_user_inactive(&self, id: UserId) -> Result<()> {
        self.user.mark_inactive(id, true)
    }

    /// Marks the user as "active" again, effectively un-deleting them.
    #[inline]
    pub fn mark_user_active(&self, id: UserId) -> Result<()> {
        self.user.mark_inactive(id, false)
    }
}
