//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2487/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2487<F: Float>(t2517: F, t4098: F, t1472: F, t9862: F, t13115: F, t9932: F, t32: F, t4094: F, t13034: F, t225: F, t10109: F, t1527: F) -> (F, F, F, F, F, F) {
    let t46437 = t4098 * t2517;
    let t46439 = t1472 * t9862;
    let t46445 = t13115 * t9932;
    let t46447 = t32 * t4094;
    let t46452 = t13034 * t225;
    let t46488 = t10109 * t1527;
    (t46437, t46439, t46445, t46447, t46452, t46488)
}
