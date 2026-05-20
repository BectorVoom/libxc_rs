//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2027/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2027<F: Float>(t22893: F, t23164: F, t25306: F, t7524: F, t81612: F, t81613: F, t4250: F, t81749: F, t23145: F, t4166: F, t22690: F, t234: F) -> (F, F, F, F, F) {
    let t87165 = t23164 * t22893 * t25306;
    let t87166 = F::cast_from(0.16449340668482264365e-1_f64) * t87165;
    let t87177 = t81612 * t81613 * t7524;
    let t87197 = t81749 * t4250;
    let t87198 = F::new(7.0) / F::new(288.0) * t87197;
    let t87199 = t4166 * t23145;
    let t87202 = t22690 * t234;
    (t87166, t87177, t87198, t87199, t87202)
}
