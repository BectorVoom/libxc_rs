//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1023/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1023<F: Float>(t5456: F, t8518: F, t1799: F, t22574: F, t26558: F, t33221: F, t33603: F, t7685: F, t1983: F, t28834: F, t31758: F, t191: F, t192: F, t29241: F) -> (F, F, F, F, F) {
    let t128555 = t8518 * t5456;
    let t128562 = F::cast_from(12.0_f64) * t22574 * t26558 * t33221 * t1799;
    let t128564 = F::cast_from(6.0_f64) * t7685 * t33603;
    let t128567 = F::cast_from(3.0_f64) * t1983 * t31758 * t28834;
    let t128570 = t29241 * t191 * t192;
    (t128555, t128562, t128564, t128567, t128570)
}
