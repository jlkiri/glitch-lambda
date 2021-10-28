use rand::{self, Rng};

const CHUNK_LEN: usize = 19;

pub fn glitch_replace(image: &mut [u8]) {
    let mut rng = rand::thread_rng();
    let size = image.len() - 1;
    let rand_idx: usize = rng.gen_range(0..=size);
    image[rand_idx] = rng.gen_range(0..=255);
}

pub fn glitch_sort(image: &mut [u8]) {
    let mut rng = rand::thread_rng();
    let size = image.len() - 1;
    let split_idx: usize = rng.gen_range(0..=size - CHUNK_LEN);
    let (_left, right) = image.split_at_mut(split_idx);
    let (glitched, _rest) = right.split_at_mut(CHUNK_LEN);
    glitched.sort();
}

pub fn glitch(image: &mut [u8]) {
    glitch_replace(image);
    glitch_sort(image);
    glitch_replace(image);
    glitch_sort(image);
    glitch_replace(image);
    glitch_sort(image);
    glitch_sort(image);
}
