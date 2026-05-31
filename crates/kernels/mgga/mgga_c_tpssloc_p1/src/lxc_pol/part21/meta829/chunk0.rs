//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2922/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2922<F: Float>(t14344: F, t4488: F, t959: F, t11094: F, t5946: F, t1068: F, t3213: F, t4700: F, t60842: F, t60844: F, t60847: F, t60850: F, t60852: F, t60855: F, t60857: F, t60860: F, t60862: F, t60864: F, t60866: F, t60867: F) -> (F, F) {
    let t60873 = F::cast_from(0.23392894490538584828e1_f64) * t959 * t4488 * t14344;
    let t60874 = t5946 * t11094;
    let t60878 = -F::cast_from(2.0_f64) * t1068 * t4700 * t60867 + F::cast_from(2.0_f64) * t3213 * t4700 * t60874 - t60842 - t60844 + t60847 - t60850 + t60852 - t60855 + t60857 - t60860 - t60862 - t60864 - t60866 + t60873;
    (t60873, t60878)
}
