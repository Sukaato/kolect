// Cas spécial : PK composite (artist_id, group_id), pas de soft delete.
// N'implémente pas le trait Repository<T> (pas d'id propre).
// Le départ d'un membre se gère via update() en renseignant leave_date.

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use super::{Page, PaginatedResult, RepoResult};
use crate::infrastructure::db::models::GroupMember;
use crate::infrastructure::db::schema::group_members::dsl::*;

pub struct GroupMemberRepository<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> GroupMemberRepository<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    pub fn insert(&mut self, item: GroupMember) -> RepoResult<GroupMember> {
        diesel::insert_into(group_members)
            .values(&item)
            .execute(self.conn)?;
        self.find_by_ids(&item.artist_id, &item.group_id)
            .map(|opt| opt.unwrap())
    }

    pub fn find_by_ids(&mut self, a_id: &str, g_id: &str) -> RepoResult<Option<GroupMember>> {
        Ok(group_members
            .filter(artist_id.eq(a_id).and(group_id.eq(g_id)))
            .first::<GroupMember>(self.conn)
            .optional()?)
    }

    /// Retourne tous les membres actifs (leave_date IS NULL), paginés.
    pub fn find_all(&mut self, page: Page) -> RepoResult<PaginatedResult<GroupMember>> {
        let total = group_members
            .filter(leave_date.is_null())
            .count()
            .get_result::<i64>(self.conn)?;

        let data = group_members
            .filter(leave_date.is_null())
            .order((group_id.asc(), artist_id.asc()))
            .limit(page.limit())
            .offset(page.offset())
            .load::<GroupMember>(self.conn)?;

        Ok(PaginatedResult::new(data, page, total))
    }

    /// Retourne tous les membres d'un groupe (actifs et anciens).
    pub fn find_by_group_id(&mut self, g_id: &str) -> RepoResult<Vec<GroupMember>> {
        Ok(group_members
            .filter(group_id.eq(g_id))
            .order(artist_id.asc())
            .load::<GroupMember>(self.conn)?)
    }

    pub fn update(&mut self, item: GroupMember) -> RepoResult<GroupMember> {
        diesel::update(
            group_members.filter(
                artist_id
                    .eq(&item.artist_id)
                    .and(group_id.eq(&item.group_id)),
            ),
        )
        .set((
            roles.eq(&item.roles),
            join_date.eq(&item.join_date),
            leave_date.eq(&item.leave_date),
        ))
        .execute(self.conn)?;
        self.find_by_ids(&item.artist_id, &item.group_id)
            .map(|opt| opt.unwrap())
    }

    /// Marque un membre comme parti en renseignant leave_date.
    pub fn set_leave_date(&mut self, a_id: &str, g_id: &str, date: &str) -> RepoResult<()> {
        diesel::update(group_members.filter(artist_id.eq(a_id).and(group_id.eq(g_id))))
            .set(leave_date.eq(date))
            .execute(self.conn)?;
        Ok(())
    }
}
