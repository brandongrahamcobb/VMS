use crate::models::character::model::Character;
use crate::models::item::equip::model::Equip;
use crate::net::error::NetworkError;
use crate::net::packet::io::error::IOError::WriteError;
use crate::net::packet::model::Packet;
use crate::op::send::SendOpcode;
use crate::prelude::*;

impl Packet {
    pub fn build_list_chars_handler_packet(
        &mut self,
        chars: Vec<Character>,
        channel_id: i16,
        char_max: i16,
        pic_status: i16,
    ) -> Result<&mut Self, NetworkError> {
        let op = SendOpcode::CharList as i16;
        self.write_short(op).map_err(WriteError)?;
        let channel_id = channel_id as i16;
        self.write_byte(channel_id).map_err(WriteError)?;
        let char_length = chars.len() as i16;
        self.write_byte(char_length) // number of chars
            .map_err(WriteError)?;
        for char in chars {
            self.build_look_part_packet(char.clone())?;
        }
        let pic_status = pic_status as i16;
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
        self.write_int(equip.model.wz_id).map_err(WriteError)?;
        Ok(self)
    }

    fn build_look_regular_hat_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let hat_identifier: i16 = 1;
        self.write_byte(hat_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_face_acc_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let face_acc_identifier: i16 = 2;
        self.write_byte(face_acc_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_eye_acc_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let eye_acc_identifier: i16 = 3;
        self.write_byte(eye_acc_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_ear_acc_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ear_acc_identifier: i16 = 4;
        self.write_byte(ear_acc_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_top_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let top_identifier: i16 = 5;
        self.write_byte(top_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_bottom_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let bottom_identifier: i16 = 6;
        self.write_byte(bottom_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_shoes_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let shoes_identifier: i16 = 7;
        self.write_byte(shoes_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_gloves_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let gloves_identifier: i16 = 8;
        self.write_byte(gloves_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_cape_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let cape_identifier: i16 = 9;
        self.write_byte(cape_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_shield_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let shield_identifier: i16 = 10;
        self.write_byte(shield_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_weapon_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let weapon_identifier: i16 = 11;
        self.write_byte(weapon_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_ring_one_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_one_identifier: i16 = 12;
        self.write_byte(ring_one_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_ring_two_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_two_identifier: i16 = 13;
        self.write_byte(ring_two_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_ring_three_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_three_identifier: i16 = 15;
        self.write_byte(ring_three_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_ring_four_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_four_identifier: i16 = 16;
        self.write_byte(ring_four_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_pendant_one_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let pendant_one_identifier: i16 = 17;
        self.write_byte(pendant_one_identifier)
            .map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_tamed_mob_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let tamed_mob_identifier: i16 = 18;
        self.write_byte(tamed_mob_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_saddle_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let saddle_identifier: i16 = 19;
        self.write_byte(saddle_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_medal_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let medal_identifier: i16 = 49;
        self.write_byte(medal_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_belt_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let belt_identifier: i16 = 50;
        self.write_byte(belt_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_pocket_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let pocket_identifier: i16 = 51;
        self.write_byte(pocket_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_book_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let book_identifier: i16 = 52;
        self.write_byte(book_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_pendant_two_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let pendant_two_identifier: i16 = 53;
        self.write_byte(pendant_two_identifier)
            .map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_shoulder_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let shoulder_identifier: i16 = 54;
        self.write_byte(shoulder_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_android_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let android_identifier: i16 = 55;
        self.write_byte(android_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_emblem_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let emblem_identifier: i16 = 56;
        self.write_byte(emblem_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_badge_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let badge_identifier: i16 = 57;
        self.write_byte(badge_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_subweapon_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let subweapon_identifier: i16 = 58;
        self.write_byte(subweapon_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_regular_heart_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let heart_identifier: i16 = 59;
        self.write_byte(heart_identifier).map_err(WriteError)?;
        self.build_look_regular_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    pub fn build_look_regular_equipment_part_packet(
        &mut self,
        char: Character,
    ) -> Result<&mut Self, NetworkError> {
        if let Some(hat) = char.regular_equip_set.hat {
            self.build_look_regular_hat_part_packet(hat.clone())?;
        }
        if let Some(face_acc) = char.regular_equip_set.face_acc {
            self.build_look_regular_face_acc_part_packet(face_acc.clone())?;
        }
        if let Some(eye_acc) = char.regular_equip_set.eye_acc {
            self.build_look_regular_eye_acc_part_packet(eye_acc.clone())?;
        }
        if let Some(ear_acc) = char.regular_equip_set.ear_acc {
            self.build_look_regular_ear_acc_part_packet(ear_acc.clone())?;
        }
        if let Some(top) = char.regular_equip_set.top {
            self.build_look_regular_top_part_packet(top.clone())?;
        }
        if let Some(bottom) = char.regular_equip_set.bottom {
            self.build_look_regular_bottom_part_packet(bottom.clone())?;
        }
        if let Some(shoes) = char.regular_equip_set.shoes {
            self.build_look_regular_shoes_part_packet(shoes.clone())?;
        }
        if let Some(gloves) = char.regular_equip_set.gloves {
            self.build_look_regular_gloves_part_packet(gloves.clone())?;
        }
        if let Some(cape) = char.regular_equip_set.cape {
            self.build_look_regular_cape_part_packet(cape.clone())?;
        }
        if let Some(shield) = char.regular_equip_set.shield {
            self.build_look_regular_shield_part_packet(shield.clone())?;
        }
        if let Some(weapon) = char.regular_equip_set.weapon {
            self.build_look_regular_weapon_part_packet(weapon.clone())?;
        }
        if let Some(ring_one) = char.regular_equip_set.ring_one {
            self.build_look_regular_ring_one_part_packet(ring_one.clone())?;
        }
        if let Some(ring_two) = char.regular_equip_set.ring_two {
            self.build_look_regular_ring_two_part_packet(ring_two.clone())?;
        }
        if let Some(ring_three) = char.regular_equip_set.ring_three {
            self.build_look_regular_ring_three_part_packet(ring_three.clone())?;
        }
        if let Some(ring_four) = char.regular_equip_set.ring_four {
            self.build_look_regular_ring_four_part_packet(ring_four.clone())?;
        }
        if let Some(pendant_one) = char.regular_equip_set.pendant_one {
            self.build_look_regular_pendant_one_part_packet(pendant_one.clone())?;
        }
        if let Some(tamed_mob) = char.regular_equip_set.tamed_mob {
            self.build_look_regular_tamed_mob_part_packet(tamed_mob.clone())?;
        }
        if let Some(saddle) = char.regular_equip_set.saddle {
            self.build_look_regular_saddle_part_packet(saddle.clone())?;
        }
        if let Some(medal) = char.regular_equip_set.medal {
            self.build_look_regular_medal_part_packet(medal.clone())?;
        }
        if let Some(belt) = char.regular_equip_set.belt {
            self.build_look_regular_belt_part_packet(belt.clone())?;
        }
        if let Some(pocket) = char.regular_equip_set.pocket {
            self.build_look_regular_pocket_part_packet(pocket.clone())?;
        }
        if let Some(book) = char.regular_equip_set.book {
            self.build_look_regular_book_part_packet(book.clone())?;
        }
        if let Some(pendant_two) = char.regular_equip_set.pendant_two {
            self.build_look_regular_pendant_two_part_packet(pendant_two.clone())?;
        }
        if let Some(shoulder) = char.regular_equip_set.shoulder {
            self.build_look_regular_shoulder_part_packet(shoulder.clone())?;
        }
        if let Some(android) = char.regular_equip_set.android {
            self.build_look_regular_android_part_packet(android.clone())?;
        }
        if let Some(emblem) = char.regular_equip_set.emblem {
            self.build_look_regular_emblem_part_packet(emblem.clone())?;
        }
        if let Some(badge) = char.regular_equip_set.badge {
            self.build_look_regular_badge_part_packet(badge.clone())?;
        }
        if let Some(subweapon) = char.regular_equip_set.subweapon {
            self.build_look_regular_subweapon_part_packet(subweapon.clone())?;
        }
        if let Some(heart) = char.regular_equip_set.heart {
            self.build_look_regular_heart_part_packet(heart.clone())?;
        }
        Ok(self)
    }

    fn build_look_cash_equip_meta_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        self.write_int(equip.model.wz_id).map_err(WriteError)?;
        Ok(self)
    }

    fn build_look_cash_hat_part_packet(&mut self, equip: Equip) -> Result<&mut Self, NetworkError> {
        let hat_identifier: i16 = 1 + 100;
        self.write_byte(hat_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_cash_face_acc_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let face_acc_identifier: i16 = 2 + 100;
        self.write_byte(face_acc_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_cash_eye_acc_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let eye_acc_identifier: i16 = 3 + 100;
        self.write_byte(eye_acc_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_cash_ear_acc_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ear_acc_identifier: i16 = 4 + 100;
        self.write_byte(ear_acc_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_cash_top_part_packet(&mut self, equip: Equip) -> Result<&mut Self, NetworkError> {
        let top_identifier: i16 = 5 + 100;
        self.write_byte(top_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_cash_bottom_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let bottom_identifier: i16 = 6 + 100;
        self.write_byte(bottom_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_cash_shoes_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let shoes_identifier: i16 = 7 + 100;
        self.write_byte(shoes_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_cash_gloves_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let gloves_identifier: i16 = 8 + 100;
        self.write_byte(gloves_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_cash_cape_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let cape_identifier: i16 = 9 + 100;
        self.write_byte(cape_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_cash_weapon_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let weapon_identifier: i16 = 11 + 100;
        self.write_byte(weapon_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_cash_ring_one_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_one_identifier: i16 = 12 + 100;
        self.write_byte(ring_one_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_cash_ring_two_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_two_identifier: i16 = 13 + 100;
        self.write_byte(ring_two_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_cash_ring_three_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_three_identifier: i16 = 15 + 100;
        self.write_byte(ring_three_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    fn build_look_cash_ring_four_part_packet(
        &mut self,
        equip: Equip,
    ) -> Result<&mut Self, NetworkError> {
        let ring_four_identifier: i16 = 16 + 100;
        self.write_byte(ring_four_identifier).map_err(WriteError)?;
        self.build_look_cash_equip_meta_part_packet(equip.clone())?;
        Ok(self)
    }

    pub fn build_look_cash_equipment_part_packet(
        &mut self,
        char: Character,
    ) -> Result<&mut Self, NetworkError> {
        if let Some(bottom) = char.cash_equip_set.bottom {
            self.build_look_cash_bottom_part_packet(bottom.clone())?;
        }
        if let Some(cape) = char.cash_equip_set.cape {
            self.build_look_cash_cape_part_packet(cape.clone())?;
        }
        if let Some(ear_acc) = char.cash_equip_set.ear_acc {
            self.build_look_cash_ear_acc_part_packet(ear_acc.clone())?;
        }
        if let Some(eye_acc) = char.cash_equip_set.eye_acc {
            self.build_look_cash_eye_acc_part_packet(eye_acc.clone())?;
        }
        if let Some(face_acc) = char.cash_equip_set.face_acc {
            self.build_look_cash_face_acc_part_packet(face_acc.clone())?;
        }
        if let Some(gloves) = char.cash_equip_set.gloves {
            self.build_look_cash_gloves_part_packet(gloves.clone())?;
        }
        if let Some(hat) = char.cash_equip_set.hat {
            self.build_look_cash_hat_part_packet(hat.clone())?;
        }
        if let Some(ring_one) = char.cash_equip_set.ring_one {
            self.build_look_cash_ring_one_part_packet(ring_one.clone())?;
        }
        if let Some(ring_two) = char.cash_equip_set.ring_two {
            self.build_look_cash_ring_two_part_packet(ring_two.clone())?;
        }
        if let Some(ring_three) = char.cash_equip_set.ring_three {
            self.build_look_cash_ring_three_part_packet(ring_three.clone())?;
        }
        if let Some(ring_four) = char.cash_equip_set.ring_four {
            self.build_look_cash_ring_four_part_packet(ring_four.clone())?;
        }
        if let Some(shoes) = char.cash_equip_set.shoes {
            self.build_look_cash_shoes_part_packet(shoes.clone())?;
        }
        if let Some(top) = char.cash_equip_set.top {
            self.build_look_cash_top_part_packet(top.clone())?;
        }
        if let Some(weapon) = char.cash_equip_set.weapon {
            self.build_look_cash_weapon_part_packet(weapon.clone())?;
        }
        Ok(self)
    }

    fn build_look_part_packet(&mut self, char: Character) -> Result<&mut Self, NetworkError> {
        self.build_list_char_meta_part_packet(char.clone())?;
        self.build_look_meta_part_packet(char.clone())?;
        self.write_byte(0).map_err(WriteError)?;
        // Disable rank.
        self.write_byte(0).map_err(WriteError)?;
        Ok(self)
    }

    pub fn build_list_char_meta_part_packet(
        &mut self,
        char: Character,
    ) -> Result<&mut Self, NetworkError> {
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
        Ok(self)
    }

    pub fn build_look_meta_part_packet(
        &mut self,
        char: Character,
    ) -> Result<&mut Self, NetworkError> {
        let gender_id = char.model.gender_id as i16;
        self.write_byte(gender_id).map_err(WriteError)?;
        let skin_id = char.model.skin_id as i16;
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
