//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1202/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1202<F: Float>(t18454: F, t3277: F, t3329: F, t5721: F, t3334: F, t1765: F, t3338: F, t339: F, t5726: F, t789: F) -> (F, F, F, F, F) {
    let t18455 = t18454 * t3277;
    let t18457 = t5721 * t3329;
    let t18459 = t5721 * t3334;
    let t18461 = t1765 * t3338;
    let t18464 = t339 * t5726 * t789;
    (t18455, t18457, t18459, t18461, t18464)
}
