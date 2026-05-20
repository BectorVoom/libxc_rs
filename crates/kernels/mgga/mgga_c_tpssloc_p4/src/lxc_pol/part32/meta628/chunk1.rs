//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2039/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2039<F: Float>(t25324: F, t6562: F, t794: F, t23030: F, t25258: F, t22893: F, t23164: F, t25306: F, t7524: F, t81612: F, t81613: F, t4250: F, t81749: F) -> (F, F, F, F, F) {
    let t87153 = t6562 * t794 * t25324;
    let t87154 = F::cast_from(0.82246703342411321824e-2_f64) * t87153;
    let t87155 = t23030 * t25258;
    let t87165 = t23164 * t22893 * t25306;
    let t87166 = F::cast_from(0.16449340668482264365e-1_f64) * t87165;
    let t87177 = t81612 * t81613 * t7524;
    let t87197 = t81749 * t4250;
    (t87154, t87155, t87166, t87177, t87197)
}
