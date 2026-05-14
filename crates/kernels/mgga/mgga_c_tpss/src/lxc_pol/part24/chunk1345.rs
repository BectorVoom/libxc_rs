//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1345/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1345<F: Float>(t21535: F, t550: F, t1781: F, t5480: F, t1789: F, t5465: F, t16041: F, t1673: F, t20083: F, t4544: F, t5466: F, t5779: F, t6296: F, t66161: F, t66163: F, t66165: F, t66167: F, t66173: F, t66175: F) -> (F,) {
    let t71123 = t21535 * t550;
    let t71125 = t1781 * t5480;
    let t71129 = t5465 * t1789;
    let t71130 = t16041 * t1789 + 2.0 * t1673 * t20083 + 2.0 * t4544 * t6296 + t5466 * t5779 + t66161 + t66163 + t66165 + t66167 + t66173 + t66175 + t71123 + t71125 + t71129;
    (t71130,)
}
