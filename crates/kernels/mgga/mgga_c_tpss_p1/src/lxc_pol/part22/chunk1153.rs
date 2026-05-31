//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1153/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1153<F: Float>(t10089: F, t3259: F, t4415: F, t4416: F, t3261: F, t3267: F, t4462: F, t12678: F, t12688: F, t12690: F, t12692: F, t12729: F, t12730: F, t12738: F, t7929: F, t7932: F, t7936: F, t7945: F, t9839: F, t9844: F, t9846: F, t9848: F, t9854: F) -> (F, F, F, F, F) {
    let t12892 = t10089 * t3259;
    let t12894 = t4415 * t4416 * t12892;
    let t12898 = t4415 * t4416 * t3261;
    let t12902 = F::cast_from(7.0_f64) / F::cast_from(2304.0_f64) * t3267 * t4462;
    let t12903 = t12678 - t12688 - t12690 + t12692 + t12729 - t9839 + t12730 + t9844 + t9846 - t9848 + t7929 - t7932 - t7936 + t9854 + t7945 - t12738;
    (t12892, t12894, t12898, t12902, t12903)
}
