//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 455/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk455<F: Float>(t1496: F, t1500: F, t1512: F, t1516: F, t249: F, t787: F, t803: F, t817: F, t840: F, t843: F) -> (F,) {
    let t1519 = -t803 - t787 * t1496 / 48.0 + t1500 * t249 / 3072.0 - t817 * t1512 / 3072.0 - t840 - t843 * t1516 / 768.0;
    (t1519,)
}
