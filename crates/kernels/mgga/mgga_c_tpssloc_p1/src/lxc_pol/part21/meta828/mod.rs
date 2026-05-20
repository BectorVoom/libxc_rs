//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta828 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2920;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2921;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta828<F: Float>(t17299: F, t2940: F, t13659: F, t4483: F, t17947: F, t2907: F, t959: F, t17191: F, t300: F, t961: F, t13724: F, t17564: F, t42671: F, t17948: F, t2933: F, t17934: F, t2952: F, t1589: F, t48766: F, t14473: F, t4493: F, t18169: F, t3216: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t60842, t60844, t60847, t60850, t60852, t60855) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2920::<F>(t17299, t2940, t13659, t4483, t17947, t2907, t959, t17191, t300, t961, t13724, t17564, t42671);
        let (t60857, t60860, t60862, t60864, t60866, t60867) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2921::<F>(t17948, t2940, t17564, t2933, t959, t17934, t2952, t1589, t48766, t14473, t4493, t18169, t3216);
    (t60842, t60844, t60847, t60850, t60852, t60855, t60857, t60860, t60862, t60864, t60866, t60867)
}
