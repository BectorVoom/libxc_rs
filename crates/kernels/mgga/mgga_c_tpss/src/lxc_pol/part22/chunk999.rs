//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 999/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk999<F: Float>(t11875: F, t1289: F, t9187: F, t1985: F, t9230: F, t128: F) -> (F, F, F) {
    let t11876 = 0.39862222222222222222e0 * t11875;
    let t11877 = t9187 * t1289;
    let t11878 = t11877 * t1985;
    let t11879 = t9230 * t11878;
    let t11880 = t128 * t11879;
    (t11876, t11878, t11880)
}
