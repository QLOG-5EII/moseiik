/* 
 * Project   : moseiik
 * Authors   : Faucheux Valentin and Plumejeau Maxime
 * File      : lib.rs
 */
pub mod main;


#[cfg(test)]
mod tests {
    mod l1_neon_tests;
    mod l1_x86_sse_tests;
    mod l1_generic_tests;
    mod prepare_target_tests;
    mod prepare_tiles_tests;
}
