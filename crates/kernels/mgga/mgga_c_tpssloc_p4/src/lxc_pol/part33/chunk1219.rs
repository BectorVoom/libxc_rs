//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1219/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1219<F: Float>(t15: F, t2229: F, t1361: F, t192: F, t1995: F, t22690: F, t2230: F, t22843: F, t213: F, t22842: F, t531: F, t598: F) -> (F, F, F, F) {
    let t80881 = F::cast_from(1.0_f64) / t2229 / t15;
    let t80885 = t80881 * t1995 * t192 * t22690 * t1361;
    let t80886 = F::cast_from(0.33643963411783659044e-4_f64) * t80885;
    let t80887 = t2230 * t22843;
    let t80888 = t80887 * t213;
    let t80893 = t598 / t22842 / t531;
    (t80881, t80886, t80888, t80893)
}
