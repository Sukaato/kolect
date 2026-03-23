use crate::db::models::GroupMember;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GroupMemberDto {
    pub artist_id: String,
    pub group_id: String,
    /// CSV: "vocalist,rapper"
    pub roles: Option<String>,
    pub join_date: Option<String>,
    pub leave_date: Option<String>,
}

impl From<GroupMemberDto> for GroupMember {
    fn from(dto: GroupMemberDto) -> Self {
        Self {
            artist_id: dto.artist_id,
            group_id: dto.group_id,
            roles: dto.roles.unwrap_or("".into()),
            join_date: dto.join_date,
            leave_date: dto.leave_date,
        }
    }
}
