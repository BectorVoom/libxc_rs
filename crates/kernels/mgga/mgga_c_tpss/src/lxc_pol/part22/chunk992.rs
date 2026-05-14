//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 992/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk992<F: Float>(t11789: F, t981: F, t11708: F, t11710: F, t11722: F, t11726: F, t11730: F, t1483: F, t2771: F, t2778: F, t2805: F, t373: F, t3990: F, t3994: F, t4017: F, t9058: F, t978: F, t991: F) -> (F,) {
    let t11790 = t981 * t11789;
    let t11792 = t11708 * t373 - 2.0 * t11710 * t991 - 6.0 * t11722 * t978 + 4.0 * t11726 * t978 + 2.0 * t11730 * t978 - t11790 * t978 - t1483 * t9058 + 4.0 * t2771 * t3994 - 2.0 * t2771 * t4017 + 2.0 * t2778 * t3990 - t2805 * t3990;
    (t11792,)
}
