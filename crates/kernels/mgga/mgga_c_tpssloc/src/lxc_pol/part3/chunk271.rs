//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 271/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk271<F: Float>(t249: F, t787: F, t803: F, t805: F, t809: F, t817: F, t831: F, t840: F, t843: F, t849: F) -> (F,) {
    let t852 = -t803 - t787 * t805 / 48.0 + t809 * t249 / 3072.0 - t817 * t831 / 3072.0 - t840 - t843 * t849 / 768.0;
    (t852,)
}
