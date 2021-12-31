use std::mem::MaybeUninit;
use rand::SeedableRng;
use rand::RngCore;

pub mod sys {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]

    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

fn main() {
    unsafe {
        // Set up a biome generator that reflects the biome generation of
        // Minecraft 1.18.
        let mut generator: MaybeUninit<sys::Generator> = MaybeUninit::uninit();
        sys::setupGenerator(
            generator.as_mut_ptr(),
            sys::MCversion_MC_1_18 as _,
            sys::LARGE_BIOMES,
        );
        let mut generator = generator.assume_init();

        // Seeds are internally represented as unsigned 64-bit integers.
        let mut seed: u64 = 0;
        let mut rng = rand_xorshift::XorShiftRng::from_seed(rand::random());
        loop {
            seed = rng.next_u64();

            // Apply the seed to the generator for the Overworld dismension.
            sys::applySeed(&mut generator, 0, seed);

            // To get the biome at a single block position, we can use getBiomeAt().
            let scale = 1; // scale=1: block coordinates, scale=4: biome coordinates
            let x = 0;
            let y = 63;
            let z = 0;

            let mut count = 0;
            let origin = sys::getBiomeAt(&mut generator, scale, 0, 63, 0);
            if origin == sys::BiomeID_mushroom_fields {
                count += 1;
            } else {
                continue;
            }
            let tl = sys::getBiomeAt(&mut generator, scale, -8192, 63, -8192);
            let tr = sys::getBiomeAt(&mut generator, scale, -8192, 63, 8192);
            let bl = sys::getBiomeAt(&mut generator, scale, 8192, 63, -8192);
            let br = sys::getBiomeAt(&mut generator, scale, 8192, 63, 8192);

            if tl == sys::BiomeID_giant_tree_taiga {
                count += 1;
            }
            if tr == sys::BiomeID_iceMountains {
                count += 1;
            }
            if bl == sys::BiomeID_desert {
                count += 1;
            }
            if br == sys::BiomeID_jungle {
                count += 1;
            }

            if count > 2 {
                println!("seed {} satisfies {}/5", seed, count);
            }
        }
    }
}
