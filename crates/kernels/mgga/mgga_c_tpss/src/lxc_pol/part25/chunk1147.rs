//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1147/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1147<F: Float>(t1625: F, t1659: F, t1270: F, t5371: F, t5366: F, t18439: F, t5373: F, t5377: F, t5716: F, t18446: F, t5383: F, t18454: F, t5389: F, t5410: F, t5721: F, t5415: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21011 = t1625 * t1659;
    let t21017 = t1270 * t5371;
    let t21027 = t1270 * t5366;
    let t21036 = t18439 * t5373;
    let t21038 = t5716 * t5377;
    let t21040 = t18446 * t5383;
    let t21042 = t18454 * t5389;
    let t21044 = t5721 * t5410;
    let t21046 = t5721 * t5415;
    (t21011, t21017, t21027, t21036, t21038, t21040, t21042, t21044, t21046)
}
