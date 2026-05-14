//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 970/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk970<F: Float>(t10572: F, t1379: F, t3683: F, t10578: F, t10579: F, t4722: F, t4707: F, t750: F, t762: F, t1368: F, t3610: F, t4711: F, t10573: F, t10584: F, t10661: F, t10678: F, t10679: F, t10777: F, t10803: F, t2147: F, t2173: F, t3626: F, t8171: F, t8204: F, t8287: F) -> (F, F, F, F) {
    let t14322 = t10572 * t1379 * t3683;
    let t14326 = t10578 * t10579 * t4722;
    let t14330 = t762 * t4707 * t750;
    let t14334 = t762 * t1368 * t3610;
    let t14338 = t762 * t4711 * t750;
    let t14343 = t10578 * t10584 * t10573;
    let t14347 = -5.0 / 384.0 * t2173 * t14322 + t2173 * t14326 / 384.0 - t8171 * t14330 / 4.0 + t2147 * t14334 / 8.0 + t2147 * t14338 / 16.0 - t10661 + t10678 - 119.0 / 6912.0 * t10679 - t3626 * t14343 / 192.0 - t8204 - 119.0 / 13824.0 * t8287 - t10777 - t10803;
    (t14322, t14326, t14343, t14347)
}
