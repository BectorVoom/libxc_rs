//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 971/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk971<F: Float>(t11047: F, t11060: F, t10471: F, t3127: F, t10470: F, t3131: F, t6739: F, t1049: F, t3120: F, t1060: F, t11023: F, t3201: F) -> (F, F, F, F, F, F, F) {
    let t11061 = t11047 * t11060;
    let t11064 = t10471 * t3127;
    let t11065 = t10470 * t11064;
    let t11066 = t6739 * t3131;
    let t11067 = t11047 * t11066;
    let t11077 = t1049 * t3120;
    let t11078 = t11077 * t1060;
    let t11081 = t11023 * t3201;
    (t11061, t11065, t11066, t11067, t11077, t11078, t11081)
}
