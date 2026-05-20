//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1711/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1711<F: Float>(t1408: F, t1530: F, t25: F, t5660: F, t28: F, t5527: F, t23788: F, t28248: F, t1484: F, t1649: F, t5544: F, t5664: F) -> (F, F, F, F, F, F, F) {
    let t28459 = t1408 * t1530;
    let t28462 = t25 * t5660;
    let t28764 = t28 * t5527;
    let t28771 = t23788 * t28248;
    let t28774 = t1649 * t1484;
    let t28778 = t28 * t5544;
    let t28789 = t28 * t5664;
    (t28459, t28462, t28764, t28771, t28774, t28778, t28789)
}
