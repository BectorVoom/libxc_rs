//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1222/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1222<F: Float>(t24987: F, t8494: F, t114360: F, t25989: F, t26142: F, t8526: F, t22461: F, t7468: F, t111: F, t33123: F, t671: F, t8312: F) -> (F, F, F, F, F, F) {
    let t119796 = t24987 * t8494;
    let t119799 = t114360 * t25989;
    let t119810 = F::cast_from(4.0_f64) * t8526 * t26142;
    let t119811 = t22461 * t7468;
    let t119815 = t33123 * t111;
    let t119820 = t8312 * t671;
    (t119796, t119799, t119810, t119811, t119815, t119820)
}
