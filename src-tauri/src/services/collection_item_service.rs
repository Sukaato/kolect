use diesel::sqlite::SqliteConnection;

use crate::db::repositories::{CollectionRepository, RepositoryError};
use crate::dto::input::commands::UpdateItemDto;

pub struct CollectionItemService<'a> {
    conn: &'a mut SqliteConnection,
}

impl<'a> CollectionItemService<'a> {
    pub fn new(conn: &'a mut SqliteConnection) -> Self {
        Self { conn }
    }

    /// Met à jour la possession d'un item en faisant le diff entre
    /// l'état actuel en BDD et l'état désiré (owned_count + signed_count).
    pub fn update_item(&mut self, dto: UpdateItemDto) -> Result<(), RepositoryError> {
        let mut repo = CollectionRepository::new(self.conn);

        // 1. Récupérer les lignes existantes triées par acquired_at ASC
        let mut rows = repo.find_by_item(&dto.item_type, &dto.item_id)?;

        let current_count = rows.len() as u32;
        let target_count = dto.owned_count;
        let target_signed = dto.signed_count.min(target_count); // signed ne peut pas dépasser owned

        // 2. Ajouter ou supprimer des lignes pour atteindre owned_count
        if target_count > current_count {
            let to_add = target_count - current_count;
            for _ in 0..to_add {
                repo.add(&dto.item_type, &dto.item_id)?;
            }
            // Recharger après les ajouts
            rows = repo.find_by_item(&dto.item_type, &dto.item_id)?;
        } else if target_count < current_count {
            let to_remove = current_count - target_count;
            // Supprimer les lignes les plus récentes (fin de la liste triée ASC)
            let ids_to_remove: Vec<String> = rows
                .iter()
                .rev()
                .take(to_remove as usize)
                .map(|r| r.id.clone())
                .collect();

            for row_id in &ids_to_remove {
                repo.remove(row_id)?;
            }

            // Retirer les lignes supprimées de la liste locale
            rows.retain(|r| !ids_to_remove.contains(&r.id));
        }

        // 3. Mettre à jour is_signed sur les N premières lignes
        // Les `target_signed` premières sont signées, le reste ne l'est pas
        for (i, row) in rows.iter().enumerate() {
            let should_be_signed = (i as u32) < target_signed;
            let currently_signed = row.is_signed != 0;

            if should_be_signed != currently_signed {
                repo.set_signed(&row.id, should_be_signed)?;
            }
        }

        Ok(())
    }
}
