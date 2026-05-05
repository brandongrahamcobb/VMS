use crate::models::character::equipment_set;
use crate::models::character::equipment_set::model::{CashEquipmentSet, RegularEquipmentSet};
use crate::models::character::error::CharacterError;
use crate::models::character::model::Character;
use crate::models::wz;
use crate::models::wz::equip::model::Equip;
use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;
use crate::runtime::state::SharedState;

impl Packet {
    pub async fn build_list_chars_handler_packet(
        &mut self,
        state: &SharedState,
        channel_id: &i8,
        chars: &Vec<Character>,
        char_max: &i16,
        pic_status: &i8,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::CharList as i16;
        self.write_short(&op).map_err(WriteError)?;
        let channel_id = *channel_id as u8;
        self.write_byte(&channel_id).map_err(WriteError)?;
        let char_length = (*chars).len() as u8;
        self.write_byte(&char_length) // number of chars
            .map_err(WriteError)?;
        for char in chars {
            let regular_equips =
                equipment_set::query::get_regular_equipment_set_by_character_id(state, &char.id)
                    .await?;
            let cash_equips =
                equipment_set::query::get_cash_equipment_set_by_character_id(state, &char.id)
                    .await?;
            self.build_look_part_packet(state, &char, &regular_equips, &cash_equips)
                .await?;
        }
        let pic_status = *pic_status as u8;
        self.write_byte(&pic_status) // use pic?
            .map_err(WriteError)?;
        let char_max = *char_max as i32;
        self.write_int(&char_max) // Number of character slots
            .map_err(WriteError)?;
        Ok(self)
    }

    fn build_look_regular_equip_meta_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        self.write_int(&equip.wz_id).map_err(WriteError)?;
        Ok(self)
    }
    fn build_look_regular_hat_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let hat_identifier: u8 = 1;
        self.write_byte(&hat_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_regular_face_acc_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let face_acc_identifier: u8 = 2;
        self.write_byte(&face_acc_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_regular_eye_acc_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let eye_acc_identifier: u8 = 3;
        self.write_byte(&eye_acc_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_regular_ear_acc_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ear_acc_identifier: u8 = 4;
        self.write_byte(&ear_acc_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_regular_top_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let top_identifier: u8 = 5;
        self.write_byte(&top_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_regular_bottom_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let bottom_identifier: u8 = 6;
        self.write_byte(&bottom_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_regular_shoes_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let shoes_identifier: u8 = 7;
        self.write_byte(&shoes_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_regular_gloves_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let gloves_identifier: u8 = 8;
        self.write_byte(&gloves_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_regular_cape_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let cape_identifier: u8 = 9;
        self.write_byte(&cape_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_regular_shield_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let shield_identifier: u8 = 10;
        self.write_byte(&shield_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_regular_weapon_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let weapon_identifier: u8 = 11;
        self.write_byte(&weapon_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_regular_ring_one_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_one_identifier: u8 = 12;
        self.write_byte(&ring_one_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_regular_ring_two_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_two_identifier: u8 = 13;
        self.write_byte(&ring_two_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_regular_ring_three_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_three_identifier: u8 = 15;
        self.write_byte(&ring_three_identifier)
            .map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_regular_ring_four_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_four_identifier: u8 = 16;
        self.write_byte(&ring_four_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_regular_pendant_one_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let pendant_one_identifier: u8 = 17;
        self.write_byte(&pendant_one_identifier)
            .map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_regular_tamed_mob_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let tamed_mob_identifier: u8 = 18;
        self.write_byte(&tamed_mob_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_regular_saddle_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let saddle_identifier: u8 = 19;
        self.write_byte(&saddle_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_regular_medal_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let medal_identifier: u8 = 49;
        self.write_byte(&medal_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_regular_belt_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let belt_identifier: u8 = 50;
        self.write_byte(&belt_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_regular_pocket_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let pocket_identifier: u8 = 51;
        self.write_byte(&pocket_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_regular_book_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let book_identifier: u8 = 52;
        self.write_byte(&book_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_regular_pendant_two_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let pendant_two_identifier: u8 = 53;
        self.write_byte(&pendant_two_identifier)
            .map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_regular_shoulder_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let shoulder_identifier: u8 = 54;
        self.write_byte(&shoulder_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_regular_android_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let android_identifier: u8 = 55;
        self.write_byte(&android_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_regular_emblem_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let emblem_identifier: u8 = 56;
        self.write_byte(&emblem_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_regular_badge_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let badge_identifier: u8 = 57;
        self.write_byte(&badge_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_regular_subweapon_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let subweapon_identifier: u8 = 58;
        self.write_byte(&subweapon_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_regular_heart_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let heart_identifier: u8 = 59;
        self.write_byte(&heart_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    pub async fn build_look_regular_equipment_part_packet(
        &mut self,
        state: &SharedState,
        regular_equips: &RegularEquipmentSet,
    ) -> Result<&mut Self, NetworkError> {
        if let Some(id) = regular_equips.hat {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_regular_hat_part_packet(&equip)?;
        }
        if let Some(id) = regular_equips.face_acc {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_regular_face_acc_part_packet(&equip)?;
        }
        if let Some(id) = regular_equips.eye_acc {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_regular_eye_acc_part_packet(&equip)?;
        }
        if let Some(id) = regular_equips.ear_acc {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_regular_ear_acc_part_packet(&equip)?;
        }
        if let Some(id) = regular_equips.top {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_regular_top_part_packet(&equip)?;
        }
        if let Some(id) = regular_equips.bottom {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_regular_bottom_part_packet(&equip)?;
        }
        if let Some(id) = regular_equips.shoes {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_regular_shoes_part_packet(&equip)?;
        }
        if let Some(id) = regular_equips.gloves {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_regular_gloves_part_packet(&equip)?;
        }
        if let Some(id) = regular_equips.cape {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_regular_cape_part_packet(&equip)?;
        }
        if let Some(id) = regular_equips.shield {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_regular_shield_part_packet(&equip)?;
        }
        if let Some(id) = regular_equips.weapon {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_regular_weapon_part_packet(&equip)?;
        }
        if let Some(id) = regular_equips.ring_one {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_regular_ring_one_part_packet(&equip)?;
        }
        if let Some(id) = regular_equips.ring_two {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_regular_ring_two_part_packet(&equip)?;
        }
        if let Some(id) = regular_equips.ring_three {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_regular_ring_three_part_packet(&equip)?;
        }
        if let Some(id) = regular_equips.ring_four {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_regular_ring_four_part_packet(&equip)?;
        }
        if let Some(id) = regular_equips.pendant_one {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_regular_pendant_one_part_packet(&equip)?;
        }
        if let Some(id) = regular_equips.tamed_mob {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_regular_tamed_mob_part_packet(&equip)?;
        }
        if let Some(id) = regular_equips.saddle {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_regular_saddle_part_packet(&equip)?;
        }
        if let Some(id) = regular_equips.medal {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_regular_medal_part_packet(&equip)?;
        }
        if let Some(id) = regular_equips.belt {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_regular_belt_part_packet(&equip)?;
        }
        if let Some(id) = regular_equips.pocket {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_regular_pocket_part_packet(&equip)?;
        }
        if let Some(id) = regular_equips.book {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_regular_book_part_packet(&equip)?;
        }
        if let Some(id) = regular_equips.pendant_two {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_regular_pendant_two_part_packet(&equip)?;
        }
        if let Some(id) = regular_equips.shoulder {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_regular_shoulder_part_packet(&equip)?;
        }
        if let Some(id) = regular_equips.android {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_regular_android_part_packet(&equip)?;
        }
        if let Some(id) = regular_equips.emblem {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_regular_emblem_part_packet(&equip)?;
        }
        if let Some(id) = regular_equips.badge {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_regular_badge_part_packet(&equip)?;
        }
        if let Some(id) = regular_equips.subweapon {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_regular_subweapon_part_packet(&equip)?;
        }
        if let Some(id) = regular_equips.heart {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_regular_heart_part_packet(&equip)?;
        }
        Ok(self)
    }

    fn build_look_cash_equip_meta_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        self.write_int(&equip.wz_id).map_err(WriteError)?;
        Ok(self)
    }

    fn build_look_cash_hat_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let hat_identifier: u8 = 1 + 100;
        self.write_byte(&hat_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_cash_face_acc_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let face_acc_identifier: u8 = 2 + 100;
        self.write_byte(&face_acc_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_cash_eye_acc_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let eye_acc_identifier: u8 = 3 + 100;
        self.write_byte(&eye_acc_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_cash_ear_acc_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ear_acc_identifier: u8 = 4 + 100;
        self.write_byte(&ear_acc_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_cash_top_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let top_identifier: u8 = 5 + 100;
        self.write_byte(&top_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_cash_bottom_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let bottom_identifier: u8 = 6 + 100;
        self.write_byte(&bottom_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_cash_shoes_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let shoes_identifier: u8 = 7 + 100;
        self.write_byte(&shoes_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_cash_gloves_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let gloves_identifier: u8 = 8 + 100;
        self.write_byte(&gloves_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_cash_cape_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let cape_identifier: u8 = 9 + 100;
        self.write_byte(&cape_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_cash_weapon_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let weapon_identifier: u8 = 11 + 100;
        self.write_byte(&weapon_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_cash_ring_one_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_one_identifier: u8 = 12 + 100;
        self.write_byte(&ring_one_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_cash_ring_two_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_two_identifier: u8 = 13 + 100;
        self.write_byte(&ring_two_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_cash_ring_three_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_three_identifier: u8 = 15 + 100;
        self.write_byte(&ring_three_identifier)
            .map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    fn build_look_cash_ring_four_part_packet(
        &mut self,
        equip: &Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_four_identifier: u8 = 16 + 100;
        self.write_byte(&ring_four_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip)?;
        Ok(self)
    }

    pub async fn build_look_cash_equipment_part_packet(
        &mut self,
        state: &SharedState,
        cash_equips: &CashEquipmentSet,
    ) -> Result<&mut Self, NetworkError> {
        if let Some(id) = cash_equips.hat {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_cash_hat_part_packet(&equip)?;
        }
        if let Some(id) = cash_equips.face_acc {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_cash_face_acc_part_packet(&equip)?;
        }
        if let Some(id) = cash_equips.eye_acc {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_cash_eye_acc_part_packet(&equip)?;
        }
        if let Some(id) = cash_equips.ear_acc {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_cash_ear_acc_part_packet(&equip)?;
        }
        if let Some(id) = cash_equips.top {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_cash_top_part_packet(&equip)?;
        }
        if let Some(id) = cash_equips.bottom {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_cash_bottom_part_packet(&equip)?;
        }
        if let Some(id) = cash_equips.shoes {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_cash_shoes_part_packet(&equip)?;
        }
        if let Some(id) = cash_equips.gloves {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_cash_gloves_part_packet(&equip)?;
        }
        if let Some(id) = cash_equips.cape {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_cash_cape_part_packet(&equip)?;
        }
        if let Some(id) = cash_equips.weapon {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_cash_weapon_part_packet(&equip)?;
        }
        if let Some(id) = cash_equips.ring_one {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_cash_ring_one_part_packet(&equip)?;
        }
        if let Some(id) = cash_equips.ring_two {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_cash_ring_two_part_packet(&equip)?;
        }
        if let Some(id) = cash_equips.ring_three {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_cash_ring_three_part_packet(&equip)?;
        }
        if let Some(id) = cash_equips.ring_four {
            let equip = wz::equip::query::get_equip_by_id(state, &id).await?;
            self.build_look_cash_ring_four_part_packet(&equip)?;
        }
        Ok(self)
    }

    async fn build_look_part_packet(
        &mut self,
        state: &SharedState,
        char: &Character,
        regular_equips: &RegularEquipmentSet,
        cash_equips: &CashEquipmentSet,
    ) -> Result<&mut Self, NetworkError> {
        self.build_list_char_meta_part_packet(char)?;
        self.build_look_meta_part_packet(state, char, regular_equips, cash_equips)
            .await?;
        self.write_byte(&0).map_err(WriteError)?;
        // Disable rank.
        self.write_byte(&0).map_err(WriteError)?;
        Ok(self)
    }

    pub fn build_list_char_meta_part_packet(
        &mut self,
        char: &Character,
    ) -> Result<&mut Self, NetworkError> {
        self.write_int(&char.id).map_err(WriteError)?;
        self.write_str(&char.ign).map_err(WriteError)?;
        self.write_bytes(&vec![0u8; 13 - char.ign.len()])
            .map_err(WriteError)?;
        let gender_id = char.gender_id as u8;
        self.write_byte(&gender_id).map_err(WriteError)?;
        let skin_id = char.skin_id as u8;
        self.write_byte(&skin_id).map_err(WriteError)?;
        self.write_int(&char.face_id).map_err(WriteError)?;
        self.write_int(&char.hair_id).map_err(WriteError)?;
        // Pets... Not implemented yet
        self.write_long(&0).map_err(WriteError)?;
        self.write_long(&0).map_err(WriteError)?;
        self.write_long(&0).map_err(WriteError)?;
        let level = char.level.ok_or(CharacterError::MissingField(char.id))? as u8;
        self.write_byte(&level).map_err(WriteError)?;
        self.write_short(&char.job_id).map_err(WriteError)?;
        self.write_short(&char.strength.ok_or(CharacterError::MissingField(char.id))?)
            .map_err(WriteError)?;
        self.write_short(
            &char
                .dexterity
                .ok_or(CharacterError::MissingField(char.id))?,
        )
        .map_err(WriteError)?;
        self.write_short(
            &char
                .intelligence
                .ok_or(CharacterError::MissingField(char.id))?,
        )
        .map_err(WriteError)?;
        self.write_short(&char.luck.ok_or(CharacterError::MissingField(char.id))?)
            .map_err(WriteError)?;
        self.write_short(&char.hp.ok_or(CharacterError::MissingField(char.id))?)
            .map_err(WriteError)?;
        self.write_short(&char.max_hp.ok_or(CharacterError::MissingField(char.id))?)
            .map_err(WriteError)?;
        self.write_short(&char.mp.ok_or(CharacterError::MissingField(char.id))?)
            .map_err(WriteError)?;
        self.write_short(&char.max_mp.ok_or(CharacterError::MissingField(char.id))?)
            .map_err(WriteError)?;
        self.write_short(&char.ap.ok_or(CharacterError::MissingField(char.id))?)
            .map_err(WriteError)?;
        // SP
        self.write_short(&0).map_err(WriteError)?;
        self.write_int(&char.exp.ok_or(CharacterError::MissingField(char.id))?)
            .map_err(WriteError)?;
        self.write_short(&char.fame.ok_or(CharacterError::MissingField(char.id))?)
            .map_err(WriteError)?;
        // Gach xp?
        self.write_int(&0).map_err(WriteError)?;
        self.write_int(&char.map_id).map_err(WriteError)?;
        self.write_byte(&0).map_err(WriteError)?;
        self.write_int(&0).map_err(WriteError)?;
        Ok(self)
    }

    pub async fn build_look_meta_part_packet(
        &mut self,
        state: &SharedState,
        char: &Character,
        regular_equips: &RegularEquipmentSet,
        cash_equips: &CashEquipmentSet,
    ) -> Result<&mut Self, NetworkError> {
        let gender_id = char.gender_id as u8;
        self.write_byte(&gender_id).map_err(WriteError)?;
        let skin_id = char.skin_id as u8;
        self.write_byte(&skin_id).map_err(WriteError)?;
        self.write_int(&char.face_id).map_err(WriteError)?;
        self.write_byte(&0) // megaphone
            .map_err(WriteError)?;
        self.write_int(&char.hair_id).map_err(WriteError)?;
        self.build_look_regular_equipment_part_packet(state, regular_equips)
            .await?;
        self.write_byte(&0xFF).map_err(WriteError)?;
        self.build_look_cash_equipment_part_packet(state, cash_equips)
            .await?;
        self.write_byte(&0xFF).map_err(WriteError)?;
        self.write_int(&0) //maskedequips -111
            .map_err(WriteError)?;
        // Pet stuff...
        self.write_int(&0).map_err(WriteError)?;
        self.write_int(&0).map_err(WriteError)?;
        self.write_int(&0).map_err(WriteError)?;
        Ok(self)
    }
}
