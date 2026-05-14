//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 232/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk232<F: Float>(t153: F, t751: F, t157: F, t717: F, t182: F, t187: F, t67: F, t181: F, t676: F, t686: F) -> (F, F, F, F, F) {
    let t752 = t153 * t751;
    let t753 = t717 * t157;
    let t755 = 0.19751673498613801407e-1 * t753 * t182;
    let t756 = t187 * t67;
    let t758 = t686 * t676 * t181;
    (t752, t753, t755, t756, t758)
}
