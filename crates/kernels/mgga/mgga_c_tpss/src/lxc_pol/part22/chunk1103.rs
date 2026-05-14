//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1103/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1103<F: Float>(t1675: F, t18660: F, t1791: F, t18331: F, t5784: F, t7690: F, t38: F, t599: F, t1981: F) -> (F, F, F, F, F) {
    let t18661 = t1675 * t18660;
    let t18663 = t1791 * t18331;
    let t18666 = t7690 * t5784;
    let t18669 = t38 * t599;
    let t18670 = t1981 * t18669;
    (t18661, t18663, t18666, t18669, t18670)
}
