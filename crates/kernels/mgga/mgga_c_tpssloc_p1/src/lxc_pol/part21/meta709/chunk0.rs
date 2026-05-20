//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2543/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2543<F: Float>(t10756: F, t300: F, t10828: F, t2930: F, t10390: F, t14501: F, t10422: F, t13761: F, t3070: F, t1615: F, t3120: F, t3040: F) -> (F, F, F, F, F, F, F) {
    let t49513 = t300 * t10756;
    let t49532 = t300 * t10828;
    let t49541 = t300 * t2930;
    let t49604 = t10390 * t14501;
    let t49607 = t3070 * t10422 * t13761;
    let t49616 = t1615 * t3120;
    let t49621 = t1615 * t3040;
    (t49513, t49532, t49541, t49604, t49607, t49616, t49621)
}
