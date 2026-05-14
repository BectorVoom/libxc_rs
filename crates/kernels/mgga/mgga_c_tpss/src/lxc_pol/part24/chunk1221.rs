//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1221/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1221<F: Float>(t17930: F, t21262: F, t1288: F, t1364: F, t30: F, t4701: F, t17949: F, t4708: F, t4712: F, t5547: F, t17956: F, t4718: F, t17964: F, t4724: F, t4761: F, t5552: F) -> (F, F, F, F, F, F, F, F) {
    let t21263 = t17930 * t21262;
    let t21266 = t1288 * t1364;
    let t21270 = t30 * t4701;
    let t21274 = t17949 * t4708;
    let t21276 = t5547 * t4712;
    let t21278 = t17956 * t4718;
    let t21280 = t17964 * t4724;
    let t21282 = t5552 * t4761;
    (t21263, t21266, t21270, t21274, t21276, t21278, t21280, t21282)
}
