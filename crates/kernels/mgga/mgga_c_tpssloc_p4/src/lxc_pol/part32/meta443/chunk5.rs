//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1698/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1698<F: Float>(t22643: F, t6890: F, t22642: F, t225: F, t6911: F, t1372: F, t214: F) -> (F, F, F, F) {
    let t22644 = t22643 * t6890;
    let t22645 = t22642 * t22644;
    let t22646 = F::cast_from(0.82246703342411321824e-2_f64) * t22645;
    let t22656 = t6911 * t225;
    let t22666 = t214 * t1372;
    (t22644, t22646, t22656, t22666)
}
