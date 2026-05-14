//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1164/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1164<F: Float>(t12679: F, t7029: F, t18547: F, t1270: F, t3234: F, t5708: F, t1760: F, t1786: F, t3403: F, t1279: F, t5773: F, t5776: F, t1688: F, t2061: F, t547: F, t116: F, t5531: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t18548 = t7029 * t12679;
    let t18550 = 6.0 * t18547 * t18548;
    let t18551 = t1270 * t3234;
    let t18552 = t5708 * t18551;
    let t18554 = 3.0 * t1760 * t18552;
    let t18584 = 3.0 * t3403 * t1786;
    let t18586 = 12.0 * t1279 * t5773;
    let t18588 = 6.0 * t1279 * t5776;
    let t18589 = t2061 * t1688;
    let t18591 = 6.0 * t547 * t18589;
    let t18592 = t116 * t5531;
    (t18548, t18550, t18551, t18552, t18554, t18584, t18586, t18588, t18589, t18591, t18592)
}
