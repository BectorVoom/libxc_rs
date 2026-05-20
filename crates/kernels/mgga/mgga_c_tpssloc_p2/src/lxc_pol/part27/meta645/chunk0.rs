//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2207/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2207<F: Float>(t23422: F, t4603: F, t14159: F, t6717: F, t14137: F, t6765: F, t7583: F, t83138: F, t23509: F, t25682: F, t25644: F, t82926: F) -> (F, F, F, F, F, F) {
    let t88335 = t23422 * t4603 / F::new(162.0);
    let t88336 = t6717 * t14159;
    let t88339 = F::new(5.0) / F::new(5184.0) * t6765 * t14137;
    let t88341 = F::cast_from(0.20186378047070195428e-3_f64) * t83138 * t7583;
    let t88342 = t23509 * t25682;
    let t88348 = t82926 * t25644;
    (t88335, t88336, t88339, t88341, t88342, t88348)
}
