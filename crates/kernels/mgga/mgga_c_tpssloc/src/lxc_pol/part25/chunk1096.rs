//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1096/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1096<F: Float>(t2230: F, t22843: F, t213: F, t22847: F, t22842: F, t531: F, t598: F, t12156: F, t1998: F, t236: F, t12328: F, t2003: F) -> (F, F, F) {
    let t80887 = t2230 * t22843;
    let t80888 = t80887 * t213;
    let t80889 = t80888 * t22847;
    let t80893 = t598 / t22842 / t531;
    let t80894 = t80893 * t213;
    let t80897 = t80894 * t1998 * t236 * t12156;
    let t80899 = t2003 * t12328;
    (t80889, t80897, t80899)
}
