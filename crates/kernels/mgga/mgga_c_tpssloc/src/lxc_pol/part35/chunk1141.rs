//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1141/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1141<F: Float>(t154: F, t8705: F, t1887: F, t534: F, t131: F, t22791: F, t9537: F, t1338: F, t225: F, t236: F, t1336: F, t2690: F, t6950: F, t15: F, t2229: F, t1361: F, t192: F, t1995: F, t22690: F) -> (F, F, F, F, F, F, F, F) {
    let t80845 = t8705 * t154;
    let t80847 = t80845 * t534 * t1887;
    let t80848 = 455.0 / 1296.0 * t80847;
    let t80853 = t22791 * t131 * t9537;
    let t80854 = t225 * t1338;
    let t80855 = t80854 * t236;
    let t80866 = t1336 * t6950 * t2690;
    let t80881 = 1.0 / t2229 / t15;
    let t80885 = t80881 * t1995 * t192 * t22690 * t1361;
    (t80845, t80848, t80853, t80854, t80855, t80866, t80881, t80885)
}
