//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 926/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk926<F: Float>(t22674: F, t7691: F, t22892: F, t6883: F, t7701: F, t254: F, t563: F, t1827: F, t22765: F, t5234: F, t6944: F, t1824: F, t236: F, t22705: F, t550: F, t22852: F) -> (F, F, F, F, F, F, F, F) {
    let t26197 = t22674 * t7691;
    let t26198 = t22892 * t26197;
    let t26200 = t6883 * t7701;
    let t26224 = t563 * t254;
    let t26231 = t22765 * t1827;
    let t26233 = t5234 * t6944;
    let t26243 = t236 * t1824;
    let t26245 = t22705 * t26243 * t550;
    let t26246 = t22852 * t26245;
    (t26197, t26198, t26200, t26224, t26231, t26233, t26245, t26246)
}
