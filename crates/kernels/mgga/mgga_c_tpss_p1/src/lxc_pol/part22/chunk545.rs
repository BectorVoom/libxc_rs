//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 545/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk545<F: Float>(t158: F, t2332: F, t581: F, t725: F, t681: F, t157: F, t37: F, t190: F, t1985: F, t72: F, t727: F, t732: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2333 = t158 * t2332;
    let t2334 = t725 * t581;
    let t2335 = t681 * t2334;
    let t2336 = F::cast_from(8.0_f64) * t2335;
    let t2337 = t37 * t157;
    let t2338 = t190 * t1985;
    let t2340 = F::cast_from(12.0_f64) * t2337 * t2338;
    let t2341 = t727 * t72;
    let t2342 = t2341 * t732;
    (t2333, t2334, t2335, t2336, t2337, t2338, t2340, t2341, t2342)
}
