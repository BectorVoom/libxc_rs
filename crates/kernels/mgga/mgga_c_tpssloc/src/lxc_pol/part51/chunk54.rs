//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 54/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk54<F: Float>(t40: F, t148: F, t74: F, t52: F, t77: F, t145: F, zeta_threshold: F) -> (F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t149 = piecewise3(t146, t148, t74);
    let t150 = t52 <= zeta_threshold;
    let t151 = piecewise3(t150, t148, t77);
    let t152 = t149 + t151 - 2.0;
    let t153 = t145 * t152;
    let t154 = M_CBRT2;
    (t152, t153, t154)
}
