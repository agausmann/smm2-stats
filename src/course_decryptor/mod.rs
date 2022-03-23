mod keys;
mod rand;

use self::{keys::COURSE_KEY_TABLE, rand::Random};
use aes::cipher::block_padding::NoPadding;
use cbc::cipher::{BlockDecryptMut, BlockSizeUser, KeyIvInit};

const STATE_SIZE: usize = 4;
const NUM_ROUNDS: usize = 4;

type Aes128CbcDec = cbc::Decryptor<aes::Aes128Dec>;

fn gen_key(key_table: &[u32], rand_state: &mut Random) -> [u32; STATE_SIZE] {
    let mut out_key = [0; STATE_SIZE];

    for slot in &mut out_key {
        for _ in 0..NUM_ROUNDS {
            *slot <<= 8;
            *slot |= (key_table[(rand_state.gen() >> 26) as usize]
                >> ((rand_state.gen() >> 27) & 24))
                & 0xff;
        }
    }
    out_key
}

pub fn decrypt_course_data(input: &[u8]) -> Vec<u8> {
    let (data, end) = input.split_at(input.len() - 0x30);
    let data = &data[0x10..];
    assert!(data.len() % Aes128CbcDec::block_size() == 0);

    let iv = &end[..0x10];
    let key_seed = &end[0x10..0x20];
    let key = gen_key(
        COURSE_KEY_TABLE,
        &mut Random::new(*bytemuck::from_bytes(key_seed)),
    );

    let decryptor = Aes128CbcDec::new(bytemuck::bytes_of(&key).into(), iv.into());
    decryptor.decrypt_padded_vec_mut::<NoPadding>(data).unwrap()
}
