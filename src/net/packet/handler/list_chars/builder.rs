use crate::models::character::model::{Character, CharacterModel};
use crate::models::wz::equip::model::Equip;
use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_list_chars_handler_packet(
        &mut self,
        chars: Vec<Character>,
        channel_id: i8,
        char_max: i8,
        pic_status: i8,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::CharList as i16;
        self.write_short(op).map_err(WriteError)?;
        let channel_id = channel_id as i8;
        self.write_byte(channel_id).map_err(WriteError)?;
        let char_length = chars.len() as i8;
        self.write_byte(char_length) // number of chars
            .map_err(WriteError)?;
        for char in chars {
            self.build_look_part_packet(char.clone())?;
        }
        let pic_status = pic_status as i8;
        self.write_byte(pic_status) // use pic?
            .map_err(WriteError)?;
        let char_max = char_max as i32;
        self.write_int(char_max) // Number of character slots
            .map_err(WriteError)?;
        Ok(self)
    }

    fn build_look_regular_equip_meta_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        self.write_int(equip.wz_id).map_err(WriteError)?;
        Ok(self)
    }

    fn build_look_regular_hat_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let hat_identifier: i8 = 1;
        self.write_byte(hat_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_face_acc_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let face_acc_identifier: i8 = 2;
        self.write_byte(face_acc_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_eye_acc_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let eye_acc_identifier: i8 = 3;
        self.write_byte(eye_acc_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_ear_acc_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ear_acc_identifier: i8 = 4;
        self.write_byte(ear_acc_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_top_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let top_identifier: i8 = 5;
        self.write_byte(top_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_bottom_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let bottom_identifier: i8 = 6;
        self.write_byte(bottom_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_shoes_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let shoes_identifier: i8 = 7;
        self.write_byte(shoes_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_gloves_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let gloves_identifier: i8 = 8;
        self.write_byte(gloves_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_cape_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let cape_identifier: i8 = 9;
        self.write_byte(cape_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_shield_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let shield_identifier: i8 = 10;
        self.write_byte(shield_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_weapon_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let weapon_identifier: i8 = 11;
        self.write_byte(weapon_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_ring_one_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_one_identifier: i8 = 12;
        self.write_byte(ring_one_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_ring_two_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_two_identifier: i8 = 13;
        self.write_byte(ring_two_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_ring_three_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_three_identifier: i8 = 15;
        self.write_byte(ring_three_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_ring_four_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_four_identifier: i8 = 16;
        self.write_byte(ring_four_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_pendant_one_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let pendant_one_identifier: i8 = 17;
        self.write_byte(pendant_one_identifier)
            .map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_tamed_mob_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let tamed_mob_identifier: i8 = 18;
        self.write_byte(tamed_mob_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_saddle_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let saddle_identifier: i8 = 19;
        self.write_byte(saddle_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_medal_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let medal_identifier: i8 = 49;
        self.write_byte(medal_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_belt_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let belt_identifier: i8 = 50;
        self.write_byte(belt_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_pocket_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let pocket_identifier: i8 = 51;
        self.write_byte(pocket_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_book_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let book_identifier: i8 = 52;
        self.write_byte(book_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_pendant_two_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let pendant_two_identifier: i8 = 53;
        self.write_byte(pendant_two_identifier)
            .map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_shoulder_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let shoulder_identifier: i8 = 54;
        self.write_byte(shoulder_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_android_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let android_identifier: i8 = 55;
        self.write_byte(android_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_emblem_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let emblem_identifier: i8 = 56;
        self.write_byte(emblem_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_badge_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let badge_identifier: i8 = 57;
        self.write_byte(badge_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_subweapon_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let subweapon_identifier: i8 = 58;
        self.write_byte(subweapon_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_heart_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let heart_identifier: i8 = 59;
        self.write_byte(heart_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    pub fn build_look_regular_equipment_part_packet(
        &mut self,
        char: Character,
    ) -> Result<&mut Self, NetworkError> {
        if !char.regular_equips.hat.model.id.is_negative() {
            self.build_look_regular_hat_part_packet(char.regular_equips.hat.clone())?;
        }
        if !char.regular_equips.face_acc.model.id.is_negative() {
            self.build_look_regular_face_acc_part_packet(char.regular_equips.face_acc.clone())?;
        }
        if !char.regular_equips.eye_acc.model.id.is_negative() {
            self.build_look_regular_eye_acc_part_packet(char.regular_equips.eye_acc.clone())?;
        }
        if !char.regular_equips.ear_acc.model.id.is_negative() {
            self.build_look_regular_ear_acc_part_packet(char.regular_equips.ear_acc.clone())?;
        }
        if !char.regular_equips.top.model.id.is_negative() {
            self.build_look_regular_top_part_packet(char.regular_equips.top.clone())?;
        }
        if !char.regular_equips.bottom.model.id.is_negative() {
            self.build_look_regular_bottom_part_packet(char.regular_equips.bottom.clone())?;
        }
        if !char.regular_equips.shoes.model.id.is_negative() {
            self.build_look_regular_shoes_part_packet(char.regular_equips.shoes.clone())?;
        }
        if !char.regular_equips.gloves.model.id.is_negative() {
            self.build_look_regular_gloves_part_packet(char.regular_equips.gloves.clone())?;
        }
        if !char.regular_equips.cape.model.id.is_negative() {
            self.build_look_regular_cape_part_packet(char.regular_equips.cape.clone())?;
        }
        if !char.regular_equips.shield.model.id.is_negative() {
            self.build_look_regular_shield_part_packet(char.regular_equips.shield.clone())?;
        }
        if !char.regular_equips.weapon.model.id.is_negative() {
            self.build_look_regular_weapon_part_packet(char.regular_equips.weapon.clone())?;
        }
        if !char.regular_equips.ring_one.model.id.is_negative() {
            self.build_look_regular_ring_one_part_packet(char.regular_equips.ring_one.clone())?;
        }
        if !char.regular_equips.ring_two.model.id.is_negative() {
            self.build_look_regular_ring_two_part_packet(char.regular_equips.ring_two.clone())?;
        }
        if !char.regular_equips.ring_three.model.id.is_negative() {
            self.build_look_regular_ring_three_part_packet(char.regular_equips.ring_three.clone())?;
        }
        if !char.regular_equips.ring_four.model.id.is_negative() {
            self.build_look_regular_ring_four_part_packet(char.regular_equips.ring_four.clone())?;
        }
        if !char.regular_equips.pendant_one.model.id.is_negative() {
            self.build_look_regular_pendant_one_part_packet(
                char.regular_equips.pendant_one.clone(),
            )?;
        }
        if !char.regular_equips.tamed_mob.model.id.is_negative() {
            self.build_look_regular_tamed_mob_part_packet(char.regular_equips.tamed_mob.clone())?;
        }
        if !char.regular_equips.saddle.model.id.is_negative() {
            self.build_look_regular_saddle_part_packet(char.regular_equips.saddle.clone())?;
        }
        if !char.regular_equips.medal.model.id.is_negative() {
            self.build_look_regular_medal_part_packet(char.regular_equips.medal.clone())?;
        }
        if !char.regular_equips.belt.model.id.is_negative() {
            self.build_look_regular_belt_part_packet(char.regular_equips.belt.clone())?;
        }
        if !char.regular_equips.pocket.model.id.is_negative() {
            self.build_look_regular_pocket_part_packet(char.regular_equips.pocket.clone())?;
        }
        if !char.regular_equips.book.model.id.is_negative() {
            self.build_look_regular_book_part_packet(char.regular_equips.book.clone())?;
        }
        if !char.regular_equips.pendant_two.model.id.is_negative() {
            self.build_look_regular_pendant_two_part_packet(
                char.regular_equips.pendant_two.clone(),
            )?;
        }
        if !char.regular_equips.shoulder.model.id.is_negative() {
            self.build_look_regular_shoulder_part_packet(char.regular_equips.shoulder.clone())?;
        }
        if !char.regular_equips.android.model.id.is_negative() {
            self.build_look_regular_android_part_packet(char.regular_equips.android.clone())?;
        }
        if !char.regular_equips.emblem.model.id.is_negative() {
            self.build_look_regular_emblem_part_packet(char.regular_equips.emblem.clone())?;
        }
        if !char.regular_equips.badge.model.id.is_negative() {
            self.build_look_regular_badge_part_packet(char.regular_equips.badge.clone())?;
        }
        if !char.regular_equips.subweapon.model.id.is_negative() {
            self.build_look_regular_subweapon_part_packet(char.regular_equips.subweapon.clone())?;
        }
        if !char.regular_equips.heart.model.id.is_negative() {
            self.build_look_regular_heart_part_packet(char.regular_equips.heart.clone())?;
        }
        Ok(self)
    }

    fn build_look_cash_equip_meta_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        self.write_int(equip.wz_id).map_err(WriteError)?;
        Ok(self)
    }

    fn build_look_cash_hat_part_packet(&mut self, equip: Equip) -> Result<&mut Self, NetworkError> {
        let hat_identifier: i8 = 1 + 100;
        self.write_byte(hat_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_cash_face_acc_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let face_acc_identifier: i8 = 2 + 100;
        self.write_byte(face_acc_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_cash_eye_acc_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let eye_acc_identifier: i8 = 3 + 100;
        self.write_byte(eye_acc_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_cash_ear_acc_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ear_acc_identifier: i8 = 4 + 100;
        self.write_byte(ear_acc_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_cash_top_part_packet(&mut self, equip: Equip) -> Result<&mut Self, NetworkError> {
        let top_identifier: i8 = 5 + 100;
        self.write_byte(top_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_cash_bottom_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let bottom_identifier: i8 = 6 + 100;
        self.write_byte(bottom_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_cash_shoes_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let shoes_identifier: i8 = 7 + 100;
        self.write_byte(shoes_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_cash_gloves_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let gloves_identifier: i8 = 8 + 100;
        self.write_byte(gloves_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_cash_cape_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let cape_identifier: i8 = 9 + 100;
        self.write_byte(cape_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_cash_weapon_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let weapon_identifier: i8 = 11 + 100;
        self.write_byte(weapon_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_cash_ring_one_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_one_identifier: i8 = 12 + 100;
        self.write_byte(ring_one_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_cash_ring_two_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_two_identifier: i8 = 13 + 100;
        self.write_byte(ring_two_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_cash_ring_three_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_three_identifier: i8 = 15 + 100;
        self.write_byte(ring_three_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_cash_ring_four_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_four_identifier: i8 = 16 + 100;
        self.write_byte(ring_four_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    pub fn build_look_cash_equipment_part_packet(
        &mut self,
        char: Character,
    ) -> Result<&mut Self, NetworkError> {
        if !char.cash_equips.bottom.model.id.is_negative() {
            self.build_look_cash_bottom_part_packet(char.cash_equips.bottom.clone())?;
        }
        if !char.cash_equips.cape.model.id.is_negative() {
            self.build_look_cash_cape_part_packet(char.cash_equips.cape.clone())?;
        }
        if !char.cash_equips.ear_acc.model.id.is_negative() {
            self.build_look_cash_ear_acc_part_packet(char.cash_equips.ear_acc.clone())?;
        }
        if !char.cash_equips.eye_acc.model.id.is_negative() {
            self.build_look_cash_eye_acc_part_packet(char.cash_equips.eye_acc.clone())?;
        }
        if !char.cash_equips.face_acc.model.id.is_negative() {
            self.build_look_cash_face_acc_part_packet(char.cash_equips.face_acc.clone())?;
        }
        if !char.cash_equips.gloves.model.id.is_negative() {
            self.build_look_cash_gloves_part_packet(char.cash_equips.gloves.clone())?;
        }
        if !char.cash_equips.hat.model.id.is_negative() {
            self.build_look_cash_hat_part_packet(char.cash_equips.hat.clone())?;
        }
        if !char.cash_equips.ring_four.model.id.is_negative() {
            self.build_look_cash_ring_four_part_packet(char.cash_equips.ring_four.clone())?;
        }
        if !char.cash_equips.ring_one.model.id.is_negative() {
            self.build_look_cash_ring_one_part_packet(char.cash_equips.ring_one.clone())?;
        }
        if !char.cash_equips.ring_three.model.id.is_negative() {
            self.build_look_cash_ring_three_part_packet(char.cash_equips.ring_three.clone())?;
        }
        if !char.cash_equips.ring_two.model.id.is_negative() {
            self.build_look_cash_ring_two_part_packet(char.cash_equips.ring_two.clone())?;
        }
        if !char.cash_equips.shoes.model.id.is_negative() {
            self.build_look_cash_shoes_part_packet(char.cash_equips.shoes.clone())?;
        }
        if !char.cash_equips.top.model.id.is_negative() {
            self.build_look_cash_top_part_packet(char.cash_equips.top.clone())?;
        }
        if !char.cash_equips.weapon.model.id.is_negative() {
            self.build_look_cash_weapon_part_packet(char.cash_equips.weapon.clone())?;
        }
        Ok(self)
    }

    fn build_look_part_packet(&mut self, char: Character) -> Result<&mut Self, NetworkError> {
        self.build_list_char_meta_part_packet(char.model.clone())?;
        self.build_look_meta_part_packet(char.clone())?;
        self.write_byte(0).map_err(WriteError)?;
        // Disable rank.
        self.write_byte(0).map_err(WriteError)?;
        Ok(self)
    }

    pub fn build_list_char_meta_part_packet(
        &mut self,
        char_model: CharacterModel,
    ) -> Result<&mut Self, NetworkError> {
        self.write_int(char_model.id).map_err(WriteError)?;
        self.write_str(char_model.ign.clone()).map_err(WriteError)?;
        self.write_bytes(vec![0u8; 13 - char_model.ign.len()])
            .map_err(WriteError)?;
        let gender_id = char_model.gender_id as i8;
        self.write_byte(gender_id).map_err(WriteError)?;
        let skin_id = char_model.skin_id as i8;
        self.write_byte(skin_id).map_err(WriteError)?;
        self.write_int(char_model.face_id).map_err(WriteError)?;
        self.write_int(char_model.hair_id).map_err(WriteError)?;
        // Pets... Not implemented yet
        self.write_long(0).map_err(WriteError)?;
        self.write_long(0).map_err(WriteError)?;
        self.write_long(0).map_err(WriteError)?;
        let level = char_model.level as i8;
        self.write_byte(level).map_err(WriteError)?;
        self.write_short(char_model.job_id).map_err(WriteError)?;
        self.write_short(char_model.strength).map_err(WriteError)?;
        self.write_short(char_model.dexterity).map_err(WriteError)?;
        self.write_short(char_model.intelligence)
            .map_err(WriteError)?;
        self.write_short(char_model.luck).map_err(WriteError)?;
        self.write_short(char_model.hp).map_err(WriteError)?;
        self.write_short(char_model.max_hp).map_err(WriteError)?;
        self.write_short(char_model.mp).map_err(WriteError)?;
        self.write_short(char_model.max_mp).map_err(WriteError)?;
        self.write_short(char_model.ap).map_err(WriteError)?;
        // SP
        self.write_short(0).map_err(WriteError)?;
        self.write_int(char_model.exp).map_err(WriteError)?;
        self.write_short(char_model.fame).map_err(WriteError)?;
        // Gach xp?
        self.write_int(0).map_err(WriteError)?;
        self.write_int(char_model.map_id).map_err(WriteError)?;
        self.write_byte(0).map_err(WriteError)?;
        self.write_int(0).map_err(WriteError)?;
        Ok(self)
    }

    pub fn build_look_meta_part_packet(
        &mut self,
        char: Character,
    ) -> Result<&mut Self, NetworkError> {
        let gender_id = char.model.gender_id as i8;
        self.write_byte(gender_id).map_err(WriteError)?;
        let skin_id = char.model.skin_id as i8;
        self.write_byte(skin_id).map_err(WriteError)?;
        self.write_int(char.model.face_id).map_err(WriteError)?;
        self.write_byte(0) // megaphone
            .map_err(WriteError)?;
        self.write_int(char.model.hair_id).map_err(WriteError)?;
        self.build_look_regular_equipment_part_packet(char.clone())?;
        self.write_byte(0xFF).map_err(WriteError)?;
        self.build_look_cash_equipment_part_packet(char.clone())?;
        self.write_byte(0xFF).map_err(WriteError)?;
        self.write_int(0) //maskedequips -111
            .map_err(WriteError)?;
        // Pet stuff...
        self.write_int(0).map_err(WriteError)?;
        self.write_int(0).map_err(WriteError)?;
        self.write_int(0).map_err(WriteError)?;
        Ok(self)
    }
}
