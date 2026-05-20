//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1577/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1577<F: Float>(t3450: F, t5398: F, t3449: F, t18237: F, t4908: F, t3448: F, t6138: F, t3451: F, t6144: F, t18225: F, t11583: F, t5392: F) -> (F, F, F, F, F, F) {
    let t18409 = t3450 * t5398;
    let t18410 = t3449 * t18409;
    let t18413 = t4908 * t18237;
    let t18416 = t3448 * t6138;
    let t18417 = t18416 * t3451;
    let t18420 = t3448 * t6144;
    let t18421 = t18420 * t3451;
    let t18424 = t4908 * t18225;
    let t18427 = t11583 * t5392;
    (t18410, t18413, t18417, t18421, t18424, t18427)
}
