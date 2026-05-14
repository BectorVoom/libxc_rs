//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1126/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1126<F: Float>(t12856: F, t12963: F, t12994: F, t13030: F, t219: F, t4488: F, t10180: F, t1656: F, t3366: F, t1265: F, t4516: F, t3365: F, t3384: F, t10085: F, t220: F, t73: F) -> (F, F, F, F, F, F, F) {
    let t13032 = t12856 + t12963 + t12994 + t13030;
    let t13033 = param_beta * t13032;
    let t13035 = t4488 * t219;
    let t13047 = t10180 * t1656 * t3366;
    let t13050 = t4516 * t1265;
    let t13051 = t3365 * t13050;
    let t13055 = t3365 * t1656 * t3384;
    let t13059 = t220 * t73 * t10085;
    (t13032, t13033, t13035, t13047, t13051, t13055, t13059)
}
