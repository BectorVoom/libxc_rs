//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2914/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2914<F: Float>(t10632: F, t5790: F, t10655: F, t17521: F, t17423: F, t2792: F, t912: F, t17422: F, t2844: F, t2842: F, t17524: F, t17528: F, t42023: F) -> (F, F, F, F, F, F) {
    let t60722 = t5790 * t10632;
    let t60741 = F::cast_from(0.32163958997385070134e2_f64) * t10655 * t17521;
    let t60744 = F::cast_from(4.0_f64) * t2792 * t17423 * t912;
    let t60745 = t17422 * t2844;
    let t60748 = F::cast_from(0.32163958997385070134e2_f64) * t2842 * t60745 * t912;
    let t60750 = F::cast_from(0.64327917994770140268e2_f64) * t10655 * t17524;
    let t60752 = F::cast_from(0.1034520258385468006e4_f64) * t42023 * t17528;
    (t60722, t60741, t60744, t60748, t60750, t60752)
}
