//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 765/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk765<F: Float>(t1873: F, t6517: F, t8319: F, t88: F, t1268: F, t8326: F, t8313: F, t191: F, t1980: F, t192: F) -> (F, F, F, F) {
    let t8441 = t6517 * t1873;
    let t8444 = 2.0 * t88 * t8319;
    let t8445 = t1268 * t8326;
    let t8446 = 2.0 * t8445;
    let t8447 = t8313 + 4.0 * t8441 + t8444 + t8446;
    let t8449 = t1980 * t191;
    let t8450 = t8449 * t192;
    (t8446, t8447, t8449, t8450)
}
