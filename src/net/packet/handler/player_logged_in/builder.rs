use crate::models::character::keybinding::model::Keybinding;
use crate::models::character::model::Character;
use crate::models::item::equip::model::Equip;
use crate::models::shroom::channel::model::Channel;
use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_player_logged_in_handler_keymap_packet(
        &mut self,
        binds: Vec<Keybinding>,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::KeyMap as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        for bind in binds {
            let bind_model = bind.model.clone();
            let bind_type = bind_model.bind_type as i16;
            self.write_byte(bind_type).map_err(WriteError)?;
            let bind_action = bind_model.action as i32;
            self.write_int(bind_action).map_err(WriteError)?;
        }
        Ok(self)
    }

    pub fn build_player_logged_in_handler_char_packet(
        &mut self,
        char: Character,
        channel: Channel,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::SetField as i16;
        self.write_short(op).map_err(WriteError)?;
        let channel_id = channel.model.id as i32;
        self.write_int(channel_id).map_err(WriteError)?;
        self //mode 1
            .write_byte(1)
            .map_err(WriteError)?;
        self //mode 2
            .write_byte(2)
            .map_err(WriteError)?;
        // Skip 23 bytes
        let skip = vec![0u8; 23];
        self.write_bytes(skip).map_err(WriteError)?;
        self.write_int(char.model.get_id()?).map_err(WriteError)?;
        self.write_str(char.model.ign.clone()).map_err(WriteError)?;
        self.write_bytes(vec![0u8; 13 - char.model.ign.len()])
            .map_err(WriteError)?;
        let gender_id = char.model.gender_id as i16;
        self.write_byte(gender_id).map_err(WriteError)?;
        let skin_id = char.model.skin_id as i16;
        self.write_byte(skin_id).map_err(WriteError)?;
        self.write_int(char.model.face_id).map_err(WriteError)?;
        self.write_int(char.model.hair_id).map_err(WriteError)?;
        // Pets... Not implemented yet
        self.write_long(0).map_err(WriteError)?;
        self.write_long(0).map_err(WriteError)?;
        self.write_long(0).map_err(WriteError)?;
        self.build_player_logged_in_meta_part_packet(char.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_hat_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let hat_identifier: i16 = 1;
        self.write_short(hat_identifier).map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_face_acc_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let face_acc_identifier: i16 = 2;
        self.write_short(face_acc_identifier).map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_eye_acc_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let eye_acc_identifier: i16 = 3;
        self.write_short(eye_acc_identifier).map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_ear_acc_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ear_acc_identifier: i16 = 4;
        self.write_short(ear_acc_identifier).map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_top_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let top_identifier: i16 = 5;
        self.write_short(top_identifier).map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_bottom_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let bottom_identifier: i16 = 6;
        self.write_short(bottom_identifier).map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_shoes_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let shoes_identifier: i16 = 7;
        self.write_short(shoes_identifier).map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_gloves_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let gloves_identifier: i16 = 8;
        self.write_short(gloves_identifier).map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_cape_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let cape_identifier: i16 = 9;
        self.write_short(cape_identifier).map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_shield_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let shield_identifier: i16 = 10;
        self.write_short(shield_identifier).map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_weapon_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let weapon_identifier: i16 = 11;
        self.write_short(weapon_identifier).map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_ring_one_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_one_identifier: i16 = 12;
        self.write_short(ring_one_identifier).map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_ring_two_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_two_identifier: i16 = 13;
        self.write_short(ring_two_identifier).map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_ring_three_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_three_identifier: i16 = 15;
        self.write_short(ring_three_identifier)
            .map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_ring_four_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_four_identifier: i16 = 16;
        self.write_short(ring_four_identifier).map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_pendant_one_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let pendant_one_identifier: i16 = 17;
        self.write_short(pendant_one_identifier)
            .map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_tamed_mob_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let tamed_mob_identifier: i16 = 18;
        self.write_short(tamed_mob_identifier).map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_saddle_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let saddle_identifier: i16 = 19;
        self.write_short(saddle_identifier).map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_medal_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let medal_identifier: i16 = 49;
        self.write_short(medal_identifier).map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_belt_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let belt_identifier: i16 = 50;
        self.write_short(belt_identifier).map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_pocket_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let pocket_identifier: i16 = 51;
        self.write_short(pocket_identifier).map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_book_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let book_identifier: i16 = 52;
        self.write_short(book_identifier).map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_pendant_two_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let pendant_two_identifier: i16 = 53;
        self.write_short(pendant_two_identifier)
            .map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_shoulder_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let shoulder_identifier: i16 = 54;
        self.write_short(shoulder_identifier).map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_android_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let android_identifier: i16 = 55;
        self.write_short(android_identifier).map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_emblem_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let emblem_identifier: i16 = 56;
        self.write_short(emblem_identifier).map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_badge_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let badge_identifier: i16 = 57;
        self.write_short(badge_identifier).map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_subweapon_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let subweapon_identifier: i16 = 58;
        self.write_short(subweapon_identifier).map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_regular_heart_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let heart_identifier: i16 = 59;
        self.write_short(heart_identifier).map_err(WriteError)?;
        self.build_inventory_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    pub fn build_inventory_regular_equipment_part_packet(
        &mut self,
        char: Character,
    ) -> Result<&mut Self, NetworkError> {
        if let Some(hat) = char.regular_equip_set.hat {
            self.build_inventory_regular_hat_part_packet(hat.clone())?;
        }
        if let Some(face_acc) = char.regular_equip_set.face_acc {
            self.build_inventory_regular_face_acc_part_packet(face_acc.clone())?;
        }
        if let Some(eye_acc) = char.regular_equip_set.eye_acc {
            self.build_inventory_regular_eye_acc_part_packet(eye_acc.clone())?;
        }
        if let Some(ear_acc) = char.regular_equip_set.ear_acc {
            self.build_inventory_regular_ear_acc_part_packet(ear_acc.clone())?;
        }
        if let Some(top) = char.regular_equip_set.top {
            self.build_inventory_regular_top_part_packet(top.clone())?;
        }
        if let Some(bottom) = char.regular_equip_set.bottom {
            self.build_inventory_regular_bottom_part_packet(bottom.clone())?;
        }
        if let Some(shoes) = char.regular_equip_set.shoes {
            self.build_inventory_regular_shoes_part_packet(shoes.clone())?;
        }
        if let Some(gloves) = char.regular_equip_set.gloves {
            self.build_inventory_regular_gloves_part_packet(gloves.clone())?;
        }
        if let Some(cape) = char.regular_equip_set.cape {
            self.build_inventory_regular_cape_part_packet(cape.clone())?;
        }
        if let Some(shield) = char.regular_equip_set.shield {
            self.build_inventory_regular_shield_part_packet(shield.clone())?;
        }
        if let Some(weapon) = char.regular_equip_set.weapon {
            self.build_inventory_regular_weapon_part_packet(weapon.clone())?;
        }
        if let Some(ring_one) = char.regular_equip_set.ring_one {
            self.build_inventory_regular_ring_one_part_packet(ring_one.clone())?;
        }
        if let Some(ring_two) = char.regular_equip_set.ring_two {
            self.build_inventory_regular_ring_two_part_packet(ring_two.clone())?;
        }
        if let Some(ring_three) = char.regular_equip_set.ring_three {
            self.build_inventory_regular_ring_three_part_packet(ring_three.clone())?;
        }
        if let Some(ring_four) = char.regular_equip_set.ring_four {
            self.build_inventory_regular_ring_four_part_packet(ring_four.clone())?;
        }
        if let Some(pendant_one) = char.regular_equip_set.pendant_one {
            self.build_inventory_regular_pendant_one_part_packet(pendant_one.clone())?;
        }
        if let Some(tamed_mob) = char.regular_equip_set.tamed_mob {
            self.build_inventory_regular_tamed_mob_part_packet(tamed_mob.clone())?;
        }
        if let Some(saddle) = char.regular_equip_set.saddle {
            self.build_inventory_regular_saddle_part_packet(saddle.clone())?;
        }
        if let Some(medal) = char.regular_equip_set.medal {
            self.build_inventory_regular_medal_part_packet(medal.clone())?;
        }
        if let Some(belt) = char.regular_equip_set.belt {
            self.build_inventory_regular_belt_part_packet(belt.clone())?;
        }
        if let Some(pocket) = char.regular_equip_set.pocket {
            self.build_inventory_regular_pocket_part_packet(pocket.clone())?;
        }
        if let Some(book) = char.regular_equip_set.book {
            self.build_inventory_regular_book_part_packet(book.clone())?;
        }
        if let Some(pendant_two) = char.regular_equip_set.pendant_two {
            self.build_inventory_regular_pendant_two_part_packet(pendant_two.clone())?;
        }
        if let Some(shoulder) = char.regular_equip_set.shoulder {
            self.build_inventory_regular_shoulder_part_packet(shoulder.clone())?;
        }
        if let Some(android) = char.regular_equip_set.android {
            self.build_inventory_regular_android_part_packet(android.clone())?;
        }
        if let Some(emblem) = char.regular_equip_set.emblem {
            self.build_inventory_regular_emblem_part_packet(emblem.clone())?;
        }
        if let Some(badge) = char.regular_equip_set.badge {
            self.build_inventory_regular_badge_part_packet(badge.clone())?;
        }
        if let Some(subweapon) = char.regular_equip_set.subweapon {
            self.build_inventory_regular_subweapon_part_packet(subweapon.clone())?;
        }
        if let Some(heart) = char.regular_equip_set.heart {
            self.build_inventory_regular_heart_part_packet(heart.clone())?;
        }
        Ok(self)
    }

    fn build_inventory_regular_equip_meta_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        // Dummy values
        self.write_byte(1).map_err(WriteError)?;
        self.write_int(equip.model.wz_id).map_err(WriteError)?;
        const NUM_EQUIP_STATS: i16 = 15;
        let is_cash = false as i16;
        self.write_byte(is_cash).map_err(WriteError)?;
        self.write_long(0).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        for _ in 0..NUM_EQUIP_STATS {
            self.write_short(0).map_err(WriteError)?;
        }
        self.write_str_with_length(String::new())
            .map_err(WriteError)?;
        self.write_short(0).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.write_short(0).map_err(WriteError)?;
        self.write_short(0).map_err(WriteError)?;
        self.write_int(0).map_err(WriteError)?;
        self.write_long(0).map_err(WriteError)?;
        self.write_bytes(vec![0u8; 12]).map_err(WriteError)?;
        Ok(self)
    }

    fn build_inventory_cash_equip_meta_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        self.write_int(equip.model.wz_id).map_err(WriteError)?;
        Ok(self)
    }

    fn build_inventory_cash_hat_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let hat_identifier: i16 = 1 + 100;
        self.write_short(hat_identifier).map_err(WriteError)?;
        self.build_inventory_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_cash_face_acc_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let face_acc_identifier: i16 = 2 + 100;
        self.write_short(face_acc_identifier).map_err(WriteError)?;
        self.build_inventory_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_cash_eye_acc_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let eye_acc_identifier: i16 = 3 + 100;
        self.write_short(eye_acc_identifier).map_err(WriteError)?;
        self.build_inventory_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_cash_ear_acc_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ear_acc_identifier: i16 = 4 + 100;
        self.write_short(ear_acc_identifier).map_err(WriteError)?;
        self.build_inventory_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_cash_top_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let top_identifier: i16 = 5 + 100;
        self.write_short(top_identifier).map_err(WriteError)?;
        self.build_inventory_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_cash_bottom_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let bottom_identifier: i16 = 6 + 100;
        self.write_short(bottom_identifier).map_err(WriteError)?;
        self.build_inventory_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_cash_shoes_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let shoes_identifier: i16 = 7 + 100;
        self.write_short(shoes_identifier).map_err(WriteError)?;
        self.build_inventory_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_cash_gloves_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let gloves_identifier: i16 = 8 + 100;
        self.write_short(gloves_identifier).map_err(WriteError)?;
        self.build_inventory_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_cash_cape_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let cape_identifier: i16 = 9 + 100;
        self.write_short(cape_identifier).map_err(WriteError)?;
        self.build_inventory_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_cash_weapon_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let weapon_identifier: i16 = 11 + 100;
        self.write_short(weapon_identifier).map_err(WriteError)?;
        self.build_inventory_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_cash_ring_one_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_one_identifier: i16 = 12 + 100;
        self.write_short(ring_one_identifier).map_err(WriteError)?;
        self.build_inventory_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_cash_ring_two_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_two_identifier: i16 = 13 + 100;
        self.write_short(ring_two_identifier).map_err(WriteError)?;
        self.build_inventory_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_cash_ring_three_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_three_identifier: i16 = 15 + 100;
        self.write_short(ring_three_identifier)
            .map_err(WriteError)?;
        self.build_inventory_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_inventory_cash_ring_four_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_four_identifier: i16 = 16 + 100;
        self.write_short(ring_four_identifier).map_err(WriteError)?;
        self.build_inventory_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    pub fn build_inventory_cash_equipment_part_packet(
        &mut self,
        char: Character,
    ) -> Result<&mut Self, NetworkError> {
        if let Some(bottom) = char.cash_equip_set.bottom {
            self.build_inventory_cash_bottom_part_packet(bottom.clone())?;
        }
        if let Some(cape) = char.cash_equip_set.cape {
            self.build_inventory_cash_cape_part_packet(cape.clone())?;
        }
        if let Some(ear_acc) = char.cash_equip_set.ear_acc {
            self.build_inventory_cash_ear_acc_part_packet(ear_acc.clone())?;
        }
        if let Some(eye_acc) = char.cash_equip_set.eye_acc {
            self.build_inventory_cash_eye_acc_part_packet(eye_acc.clone())?;
        }
        if let Some(face_acc) = char.cash_equip_set.face_acc {
            self.build_inventory_cash_face_acc_part_packet(face_acc.clone())?;
        }
        if let Some(gloves) = char.cash_equip_set.gloves {
            self.build_inventory_cash_gloves_part_packet(gloves.clone())?;
        }
        if let Some(hat) = char.cash_equip_set.hat {
            self.build_inventory_cash_hat_part_packet(hat.clone())?;
        }
        if let Some(ring_four) = char.cash_equip_set.ring_four {
            self.build_inventory_cash_ring_four_part_packet(ring_four.clone())?;
        }
        if let Some(ring_one) = char.cash_equip_set.ring_one {
            self.build_inventory_cash_ring_one_part_packet(ring_one.clone())?;
        }
        if let Some(ring_three) = char.cash_equip_set.ring_three {
            self.build_inventory_cash_ring_three_part_packet(ring_three.clone())?;
        }
        if let Some(ring_two) = char.cash_equip_set.ring_two {
            self.build_inventory_cash_ring_two_part_packet(ring_two.clone())?;
        }
        if let Some(shoes) = char.cash_equip_set.shoes {
            self.build_inventory_cash_shoes_part_packet(shoes.clone())?;
        }
        if let Some(top) = char.cash_equip_set.top {
            self.build_inventory_cash_top_part_packet(top.clone())?;
        }
        if let Some(weapon) = char.cash_equip_set.weapon {
            self.build_inventory_cash_weapon_part_packet(weapon.clone())?;
        }
        Ok(self)
    }

    pub fn build_inventory_part_packet(
        &mut self,
        char: Character,
    ) -> Result<&mut Self, NetworkError> {
        self.write_int(char.model.meso).map_err(WriteError)?;
        // Dummy values
        // Inventory slot Capacities
        self.write_bytes(vec![0u8; 5]).map_err(WriteError)?;
        // Time?
        self.write_long(0).map_err(WriteError)?;
        self.build_inventory_regular_equipment_part_packet(char.clone())?;
        self.build_inventory_cash_equipment_part_packet(char.clone())?;
        // End of equipment equipped (all id's) MUST BE ENDED WITH A SHORT 0
        self.write_short(0).map_err(WriteError)?;
        // Start of equipment inventory (negative id's) MUST BE ENDED WITH A SHORT 0
        self.write_short(0).map_err(WriteError)?;
        // Start of equipment inventory (postive id's)  MUST BE ENDED WITH A SHORT 0
        self.write_short(0).map_err(WriteError)?;
        // Skip 2 bytes after equips
        let skip = vec![0u8; 2];
        self.write_bytes(skip).map_err(WriteError)?;
        // Dummy values
        // Start of USE
        self.write_byte(0).map_err(WriteError)?;
        // Start of SETUP
        self.write_byte(0).map_err(WriteError)?;
        // Start of ETC
        self.write_byte(0).map_err(WriteError)?;
        // Start of CASH
        self.write_byte(0).map_err(WriteError)?;
        Ok(self)
    }

    pub fn build_player_logged_in_meta_part_packet(
        &mut self,
        char: Character,
    ) -> Result<&mut Self, NetworkError> {
        let level = char.model.level as i16;
        self.write_byte(level).map_err(WriteError)?;
        self.write_short(char.model.job_id).map_err(WriteError)?;
        self.write_short(char.model.strength).map_err(WriteError)?;
        self.write_short(char.model.dexterity).map_err(WriteError)?;
        self.write_short(char.model.intelligence)
            .map_err(WriteError)?;
        self.write_short(char.model.luck).map_err(WriteError)?;
        self.write_short(char.model.hp).map_err(WriteError)?;
        self.write_short(char.model.max_hp).map_err(WriteError)?;
        self.write_short(char.model.mp).map_err(WriteError)?;
        self.write_short(char.model.max_mp).map_err(WriteError)?;
        self.write_short(char.model.ap).map_err(WriteError)?;
        // SP
        self.write_short(0).map_err(WriteError)?;
        self.write_int(char.model.exp).map_err(WriteError)?;
        self.write_short(char.model.fame).map_err(WriteError)?;
        // Gach xp?
        self.write_int(0).map_err(WriteError)?;
        self.write_int(char.model.map_id).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.write_int(0).map_err(WriteError)?;
        let bl_capacity = 25;
        self.write_byte(bl_capacity).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.build_inventory_part_packet(char.clone())?
            .build_skills_part_packet()?
            .build_quests_part_packet()?
            .build_minigames_part_packet()?
            .build_rings_part_packet()?
            .build_teleport_part_packet()?
            .build_codex_part_packet()?
            .build_new_year_cards_part_packet()?
            .build_area_info_part_packet()?;
        Ok(self)
    }

    fn build_skills_part_packet(&mut self) -> Result<&mut Self, NetworkError> {
        // Dummy values
        // No skills!
        self.write_short(0).map_err(WriteError)?;
        // No no cooldowns!
        self.write_short(0).map_err(WriteError)?;
        Ok(self)
    }

    fn build_quests_part_packet(&mut self) -> Result<&mut Self, NetworkError> {
        // Dummy values
        let started_quests = 0;
        self.write_short(started_quests).map_err(WriteError)?;
        let completed_quests = 0;
        self.write_short(completed_quests).map_err(WriteError)?;
        Ok(self)
    }

    fn build_minigames_part_packet(&mut self) -> Result<&mut Self, NetworkError> {
        // Dummy values
        self.write_short(0).map_err(WriteError)?;
        Ok(self)
    }

    fn build_rings_part_packet(&mut self) -> Result<&mut Self, NetworkError> {
        // Dummy values
        let num_crush_rings = 0;
        let num_friendship_rings = 0;
        self.write_short(num_crush_rings).map_err(WriteError)?;
        self.write_short(num_friendship_rings).map_err(WriteError)?;
        // Not married
        self.write_short(0).map_err(WriteError)?;
        Ok(self)
    }

    fn build_teleport_part_packet(&mut self) -> Result<&mut Self, NetworkError> {
        // Dummy values
        for _ in 0..5 {
            self.write_int(0).map_err(WriteError)?;
        }
        for _ in 0..10 {
            self.write_int(0).map_err(WriteError)?;
        }
        Ok(self)
    }

    fn build_codex_part_packet(&mut self) -> Result<&mut Self, NetworkError> {
        // Dummy values
        let codex_cover = 1;
        let num_cards = 0;
        self.write_int(codex_cover).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.write_short(num_cards).map_err(WriteError)?;
        Ok(self)
    }

    fn build_new_year_cards_part_packet(&mut self) -> Result<&mut Self, NetworkError> {
        // Dummy values
        let num_cards = 0;
        self.write_short(num_cards).map_err(WriteError)?;
        Ok(self)
    }

    fn build_area_info_part_packet(&mut self) -> Result<&mut Self, NetworkError> {
        let num_areas = 0;
        self.write_short(num_areas).map_err(WriteError)?;
        Ok(self)
    }

    pub fn build_spawn_player_packet(
        &mut self,
        char: Character,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::SpawnPlayer as i16;
        self.write_short(op).map_err(WriteError)?;
        self.write_int(char.model.get_id()?).map_err(WriteError)?;
        let level = char.model.level as i16;
        self.write_byte(level).map_err(WriteError)?;
        self.write_str_with_length(char.model.ign.clone())
            .map_err(WriteError)?;
        let guild_name = String::from("Guild Name");
        self.write_str_with_length(guild_name).map_err(WriteError)?;
        let skip = 0 as i16;
        self.write_short(skip).map_err(WriteError)?; // guildlogobg
        let skip = 0 as i16;
        self.write_byte(skip).map_err(WriteError)?; //guildlogobgcolor
        let skip = 0 as i16;
        self.write_short(skip).map_err(WriteError)?; //guildlogo
        let skip = 0 as i16;
        self.write_byte(skip).map_err(WriteError)?; //guildlogocolor
        let skip = vec![0u8; 8];
        self.write_bytes(skip).map_err(WriteError)?;
        let morphed = 0; // 2 if morphed
        self.write_int(morphed).map_err(WriteError)?;
        let buff_mask_one = 0;
        self.write_int(buff_mask_one).map_err(WriteError)?;
        if buff_mask_one != 0 {
            if morphed == 2 {
                let buff_value = 0; // changes if morphed
                self.write_short(buff_value).map_err(WriteError)?;
            } else {
                let buff_value = 0; // changes if not morphed
                self.write_byte(buff_value).map_err(WriteError)?;
            }
        }
        let buff_mask_two = 0; // 0 not sure
        self.write_int(buff_mask_two).map_err(WriteError)?;
        let skip = vec![0u8; 43];
        self.write_bytes(skip).map_err(WriteError)?;
        let mount = 0; // 0 not sure
        self.write_int(mount).map_err(WriteError)?;
        let skip = vec![0u8; 61];
        self.write_bytes(skip).map_err(WriteError)?;
        self.write_short(char.model.job_id).map_err(WriteError)?;
        self.build_look_meta_part_packet(char.clone())?;
        let count = 5110000;
        self.write_int(count).map_err(WriteError)?;
        let item_effect = 0; // 0 not sure
        self.write_int(item_effect).map_err(WriteError)?;
        let chair = 0; // 0 not sure
        self.write_int(chair).map_err(WriteError)?;
        let position_x = 0; // 0 this is a point so it might be wrong
        let position_y = 0; // 0 this is a point so it might be wrong
        self.write_short(position_x).map_err(WriteError)?;
        self.write_short(position_y).map_err(WriteError)?;
        let stance = 0 as i16; // 0 not sure
        self.write_byte(stance).map_err(WriteError)?;
        let skip = vec![0u8; 3];
        self.write_bytes(skip).map_err(WriteError)?;
        for _ in 0..3 {
            let available = 0; // 0 not sure
            self.write_byte(available).map_err(WriteError)?;
            if available == 1 {
                let byte_two = 0 as i16; // 0 not sure
                self.write_byte(byte_two).map_err(WriteError)?;
                let pet_id = 0; // 0 is definitely not right
                self.write_int(pet_id).map_err(WriteError)?;
                let pet_name = String::from("George");
                self.write_str_with_length(pet_name).map_err(WriteError)?;
                let unique_id = 0; // 0 not sure
                self.write_int(unique_id).map_err(WriteError)?;
                let skip = 0;
                self.write_int(skip).map_err(WriteError)?;
                self.write_short(position_x).map_err(WriteError)?;
                self.write_short(position_y).map_err(WriteError)?;
                self.write_byte(stance).map_err(WriteError)?;
                let fhid = 0; // 0 not sure
                self.write_int(fhid).map_err(WriteError)?;
            } else {
                break;
            }
        }
        let mount_level = 0; // 0 not sure
        self.write_int(mount_level).map_err(WriteError)?;
        let mount_exp = 0; // 0 not sure
        self.write_int(mount_exp).map_err(WriteError)?;
        let mount_tiredness = 0; // 0 not sure
        self.write_int(mount_tiredness).map_err(WriteError)?;
        let skip = 0 as i16;
        self.write_byte(skip).map_err(WriteError)?; // shop stuff
        let chalkboard_bool: bool = false; // false not sure
        let chalkboard: i16 = chalkboard_bool as i16; // false not sure
        self.write_byte(chalkboard).map_err(WriteError)?;
        if chalkboard_bool {
            let chalkboard_text = String::from("Placeholder");
            self.write_str_with_length(chalkboard_text)
                .map_err(WriteError)?;
        }
        let skip = vec![0u8; 3];
        self.write_bytes(skip).map_err(WriteError)?;
        let team = 0 as i16; // 0 not sure
        self.write_byte(team).map_err(WriteError)?;
        Ok(self)
    }
}
