//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1399/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1399<F: Float>(t23592: F, t23631: F, t974: F, t25721: F, t6743: F, t210: F, t23599: F, t23632: F, t23511: F, t23634: F, t3040: F, t607: F) -> (F, F, F, F, F) {
    let t83239 = t23631 * t974 * t23592;
    let t83240 = t6743 * t25721;
    let t83244 = t23599 * t210;
    let t83245 = t83244 * t23632;
    let t83246 = t23511 * t23634;
    let t83247 = t607 * t3040;
    (t83239, t83240, t83245, t83246, t83247)
}
