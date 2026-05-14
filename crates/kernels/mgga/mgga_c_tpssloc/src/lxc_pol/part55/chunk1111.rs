//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1111/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1111<F: Float>(t16524: F, t31280: F, t33185: F, t20173: F, t33193: F, t3941: F, t4072: F, t8326: F, t31285: F, t16521: F, t12524: F, t33188: F, t6534: F, t7467: F, t1873: F, t26135: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t120788 = 54.0 * t16524 * t31280;
    let t120792 = 54.0 * t33185 * t31280;
    let t120800 = 27.0 * t20173 * t33193;
    let t120803 = 27.0 * t3941 * t8326 * t4072;
    let t120807 = 27.0 * t16524 * t31285;
    let t120809 = 0.135e2 * t16521 * t8326;
    let t120811 = 54.0 * t12524 * t33188;
    let t120818 = 27.0 * t12524 * t33193;
    let t120820 = 54.0 * t20173 * t33188;
    let t120823 = 54.0 * t3941 * t6534 * t7467;
    let t120830 = 54.0 * t3941 * t1873 * t26135;
    (t120788, t120792, t120800, t120803, t120807, t120809, t120811, t120818, t120820, t120823, t120830)
}
