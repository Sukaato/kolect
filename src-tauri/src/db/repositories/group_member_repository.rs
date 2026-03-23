// Cas spécial : PK composite (artist_id, group_id), pas de soft delete.
// N'implémente pas le trait Repository<T> (pas d'id propre).
// Le départ d'un membre se gère via update() en renseignant leave_date.

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use super::RepoResult;
use crate::db::models::GroupMember;
use crate::db::schema::group_members::dsl::*;

pub struct GroupMemberRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> GroupMemberRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    /// Retourne tous les membres d'un groupe (actifs et anciens).
    pub fn find_by_group_id(&mut self, g_id: &str) -> RepoResult<Vec<GroupMember>> {
        Ok(group_members
            .filter(group_id.eq(g_id))
            .order(artist_id.asc())
            .load::<GroupMember>(self.conn)?)
    }
}
