//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1135/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1135<F: Float>(t15610: F, t2911: F, t1519: F, t4104: F, t2862: F, t1042: F, t5117: F, t9292: F, t5114: F, t2913: F, t5113: F, t4108: F) -> (F, F, F, F, F, F) {
    let t15612 = F::new(6.0) * t2911 * t15610;
    let t15613 = t1519 * t4104;
    let t15615 = F::new(4.0) * t2862 * t15613;
    let t15616 = t5117 * t1042;
    let t15618 = F::new(0.96491876992155210402e2) * t9292 * t15616;
    let t15619 = t5114 * t1042;
    let t15621 = F::new(2.0) * t2862 * t15619;
    let t15622 = t5113 * t2913;
    let t15623 = t15622 * t1042;
    let t15625 = F::new(0.16081979498692535067e2) * t2911 * t15623;
    let t15626 = t4108 * t4104;
    (t15612, t15615, t15618, t15621, t15625, t15626)
}
