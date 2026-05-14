//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 949/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk949<F: Float>(t28: F, t5527: F, t23788: F, t28248: F, t1484: F, t1649: F, t5544: F, t5664: F, t1530: F, t5660: F, t191: F, t192: F, t6295: F, t1390: F, t6330: F, t1799: F, t1845: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t28764 = t28 * t5527;
    let t28771 = t23788 * t28248;
    let t28774 = t1649 * t1484;
    let t28778 = t28 * t5544;
    let t28789 = t28 * t5664;
    let t28792 = t1649 * t1530;
    let t28795 = t28 * t5660;
    let t28821 = t6295 * t191 * t192;
    let t28826 = t1390 * t6330;
    let t28830 = t1799 * t1845;
    (t28764, t28771, t28774, t28778, t28789, t28792, t28795, t28821, t28826, t28830)
}
