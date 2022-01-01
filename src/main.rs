use rand::RngCore;
use rand::SeedableRng;
use std::mem::MaybeUninit;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod sys;

fn main() {
    let num_cpus = num_cpus::get();
    println!("running on {} cpus", num_cpus);

    let join_handles: Vec<_> = (0..num_cpus)
        .map(|_| std::thread::spawn(|| unsafe { run_it() }))
        .collect();

    for h in join_handles {
        h.join().unwrap();
    }
}

unsafe fn run_it() {
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

        sys::applySeed(&mut generator, 0, seed);

        let scale = 1;
        let x = 0;
        let y = 63;
        let z = 0;

        let mut count = 0;
        let origin = getCategoryAt(&mut generator, 0, 0);
        if origin != sys::BiomeID_mushroom_fields {
            continue;
        }
        let tl = getCategoryAt(&mut generator, -8192, -8192);
        let tr = getCategoryAt(&mut generator, -8192, 8192);
        let bl = getCategoryAt(&mut generator, 8192, -8192);
        let br = getCategoryAt(&mut generator, 8192, 8192);

        let corners = [tl, tr, bl, br];
        let have_jungle = corners.contains(&sys::BiomeID_jungle);
        let have_desert = corners.contains(&sys::BiomeID_desert);
        let have_ice = corners.contains(&sys::BiomeID_snowy_tundra);
        let have_mesa = corners.contains(&sys::BiomeID_mesa)
            || corners.contains(&sys::BiomeID_badlands_plateau);

        let count = 1
            + (have_jungle as u32)
            + (have_desert as u32)
            + (have_ice as u32)
            + (have_mesa as u32);

        if count > 4 {
            println!("seed {} satisfies {}/5", seed, count);
        }
    }
}

fn getCategoryAt(g: &mut sys::Generator, x: i32, y: i32) -> i32 {
    unsafe { sys::getCategory(sys::MCversion_MC_1_18 as _, sys::getBiomeAt(g, 1, x, 63, y)) }
}
