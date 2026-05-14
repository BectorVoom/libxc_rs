//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1220/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1220<F: Float>(t25992: F, t8690: F, t24991: F, t119837: F, t119839: F, t119841: F, t119844: F, t119845: F, t119850: F, t119852: F, t119856: F, t24983: F, t27290: F, t4026: F, t6517: F, t7266: F, t8682: F) -> (F,) {
    let t123027 = t8690 * t25992;
    let t123028 = t8690 * t24991;
    let t123034 = -2.0 * t24983 * t7266 - 2.0 * t27290 * t6517 - t4026 * t8682 - t119837 - t119839 - t119841 - t119844 - t119845 - t119850 - t119852 - t119856 - t123027 + 3.0 * t123028;
    (t123034,)
}
