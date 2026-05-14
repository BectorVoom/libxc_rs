//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 691/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk691<F: Float>(t189: F, t3297: F, t489: F, t2281: F, t2285: F, t2292: F, t2302: F, t2310: F, t3189: F, t3199: F, t3201: F, t3209: F, t3281: F, t1170: F, t1184: F, t1186: F) -> (F, F, F, F, F, F) {
    let t3298 = t3297 * t189;
    let t3299 = t489 * t3298;
    let t3300 = t2302 + t2310 - t2292 - t2281 - t2285 + t3281 + t3299 + t3199 - t3201 - t3209 + t3189;
    let t3301 = t1170 * t1184;
    let t3302 = 8.0 * t3301;
    let t3304 = 8.0 * t1170 * t1186;
    (t3298, t3299, t3300, t3301, t3302, t3304)
}
