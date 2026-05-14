//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 818/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk818<F: Float>(t1270: F, t1625: F, t1630: F, t5716: F, t1642: F, t5721: F, t1646: F, t5728: F, t1649: F, t1705: F, t935: F, t1791: F, t6090: F) -> (F, F, F, F, F, F, F) {
    let t6245 = t1270 * t1625;
    let t6249 = t5716 * t1630;
    let t6251 = t5721 * t1642;
    let t6253 = t5728 * t1646;
    let t6259 = t1705 * t1649;
    let t6260 = t6259 * t935;
    let t6304 = t1791 * t6090;
    (t6245, t6249, t6251, t6253, t6259, t6260, t6304)
}
