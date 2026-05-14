//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 680/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk680<F: Float>(t30: F, t33: F, t14: F, t22: F, t498: F, t558: F, t563: F, t491: F, t580: F, t1197: F, t1991: F, t494: F, t1006: F, t1201: F, t2829: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t3211 = t14 * t22;
    let t3213 = 12.0 * t3211 * t498;
    let t3214 = t558 * t563;
    let t3216 = 32.0 * t3214 * t498;
    let t3217 = 1.0 / t491;
    let t3218 = t580 * t580;
    let t3224 = piecewise3(t31, 0.0, -2.0 / 9.0 * t3217 * t3218 + 2.0 / 3.0 * t1197 * t1991);
    let t3225 = 1.0 / t494;
    let t3226 = t1006 * t1006;
    let t3232 = piecewise3(t34, 0.0, -2.0 / 9.0 * t3225 * t3226 + 2.0 / 3.0 * t1201 * t2829);
    let t3234 = t3224 / 2.0 + t3232 / 2.0;
    (t3211, t3213, t3214, t3216, t3217, t3218, t3225, t3226, t3234)
}
