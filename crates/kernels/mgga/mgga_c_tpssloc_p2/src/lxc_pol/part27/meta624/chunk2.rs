//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2106/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2106<F: Float>(t109: F, t86603: F, t1401: F, t55571: F, t7769: F, t20173: F, t26542: F, t26545: F, t12524: F, t1458: F, t22479: F, t3941: F, t4072: F, t6534: F) -> (F, F, F, F, F, F, F, F) {
    let t110 = F::cast_from(1.0_f64) < t109;
    let t86604 = piecewise3::<F>(t110, F::cast_from(0.0_f64), t86603);
    let t86606 = F::cast_from(0.135e2_f64) * t1401 * t86604;
    let t86610 = F::cast_from(27.0_f64) * t55571 * t7769;
    let t86612 = F::cast_from(54.0_f64) * t20173 * t26542;
    let t86614 = F::cast_from(54.0_f64) * t20173 * t26545;
    let t86616 = F::cast_from(54.0_f64) * t12524 * t26545;
    let t86619 = F::cast_from(27.0_f64) * t3941 * t22479 * t1458;
    let t86622 = F::cast_from(54.0_f64) * t3941 * t6534 * t4072;
    (t86604, t86606, t86610, t86612, t86614, t86616, t86619, t86622)
}
