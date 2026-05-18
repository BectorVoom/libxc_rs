//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1361/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1361<F: Float>(t26135: F, t7010: F, t1873: F, t86656: F, t12524: F, t33193: F, t20173: F, t33188: F, t3941: F, t6534: F, t7467: F, t26523: F) -> (F, F, F, F, F, F) {
    let t120812 = t7010 * t26135;
    let t120815 = t86656 * t1873;
    let t120818 = F::new(27.0) * t12524 * t33193;
    let t120820 = F::new(54.0) * t20173 * t33188;
    let t120823 = F::new(54.0) * t3941 * t6534 * t7467;
    let t120826 = t26523 * t6534;
    (t120812, t120815, t120818, t120820, t120823, t120826)
}
