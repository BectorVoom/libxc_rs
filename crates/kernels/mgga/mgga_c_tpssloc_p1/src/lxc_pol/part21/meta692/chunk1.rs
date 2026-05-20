//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2508/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2508<F: Float>(t13080: F, t9638: F, t226: F, t40931: F, t68: F, t13377: F, t814: F, t13396: F, t808: F, t13068: F, t225: F, t13030: F) -> (F, F, F, F, F, F) {
    let t47353 = t9638 * t13080;
    let t47386 = t226 * t68 * t40931;
    let t47395 = t814 * t13377;
    let t47419 = t808 * t13396;
    let t47568 = t13068 * t225;
    let t47585 = t13030 * t225;
    (t47353, t47386, t47395, t47419, t47568, t47585)
}
