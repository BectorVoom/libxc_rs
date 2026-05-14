//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 943/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk943<F: Float>(t13850: F, t520: F, t1224: F, t774: F, t1206: F, t5366: F, t3348: F, t3342: F, t5420: F, t10161: F, t10166: F, t1222: F, t1244: F, t12993: F, t13004: F, t13006: F, t13013: F, t13018: F, t13021: F, t13795: F, t13800: F) -> (F, F, F, F, F) {
    let t13851 = t13850 * t520;
    let t13853 = t1224 * t774 * t13851;
    let t13856 = t5366 * t1206;
    let t13858 = t3348 * t774 * t13856;
    let t13862 = t3342 * t5420;
    let t13864 = t12993 - t13004 + t13006 - 35.0 / 216.0 * t10161 - t10166 - 5.0 / 128.0 * t1244 * t13795 + 5.0 / 384.0 * t1244 * t13800 - t1222 * t13853 / 3072.0 + 5.0 / 768.0 * t1244 * t13858 + t13013 - 119.0 / 1728.0 * t13018 - 35.0 / 1152.0 * t13862 - t13021;
    (t13851, t13853, t13856, t13858, t13864)
}
