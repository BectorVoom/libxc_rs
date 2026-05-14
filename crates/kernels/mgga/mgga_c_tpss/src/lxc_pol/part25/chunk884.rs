//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 884/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk884<F: Float>(t3692: F, t768: F, t3724: F, t823: F, t3762: F, t845: F, t1411: F, t2530: F, t1408: F, t2193: F) -> (F, F, F, F, F) {
    let t10884 = t768 * t3692;
    let t10923 = t3724 * t823;
    let t10961 = t3762 * t845;
    let t10966 = t1411 * t2530;
    let t10980 = t2193 * t1408;
    (t10884, t10923, t10961, t10966, t10980)
}
