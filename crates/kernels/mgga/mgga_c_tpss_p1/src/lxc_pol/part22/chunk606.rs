//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 606/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk606<F: Float>(t346: F, t939: F, t348: F, t356: F, t329: F) -> (F, F, F, F) {
    let t2715 = F::new(1.0) / t939 / t346;
    let t2716 = t2715 * t348;
    let t2717 = t356 * t356;
    let t2719 = F::new(1.0) / t2717 / t329;
    (t2715, t2716, t2717, t2719)
}
