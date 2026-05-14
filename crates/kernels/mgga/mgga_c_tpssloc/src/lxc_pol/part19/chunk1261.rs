//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1261/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1261<F: Float>(t11126: F, t3423: F, t11286: F, t3411: F, t11629: F, t11399: F, t1164: F, t3400: F, t4883: F, t3377: F) -> (F, F, F, F, F) {
    let t43670 = 0.10389515463408878255e3 * t11126 * t3423;
    let t43672 = 0.4101607543286562663e4 * t3411 * t11286;
    let t43674 = 0.14035736694323150897e2 * t3411 * t11629;
    let t43678 = 0.69263436422725855036e2 * t1164 * t3400 * t11399 * t4883;
    let t43679 = t3377 * t3377;
    (t43670, t43672, t43674, t43678, t43679)
}
