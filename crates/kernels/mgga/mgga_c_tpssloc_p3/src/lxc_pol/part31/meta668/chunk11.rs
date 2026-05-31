//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1976/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1976<F: Float>(t10109: F, t7841: F, t13065: F, t13463: F, t1528: F, t17052: F, t17090: F, t2054: F, t24305: F, t25168: F, t26703: F, t26713: F, t4147: F, t4272: F, t4301: F, t5658: F, t59498: F, t7092: F, t7107: F, t7830: F, t7842: F, t85101: F, t87779: F, t92846: F, t92847: F, t92862: F, t92866: F, t92872: F, t98921: F, t98923: F, t98927: F) -> F {
    let t101551 = t10109 * t7841;
    let t101569 = -t85101 - t92846 - t24305 * t5658 + F::cast_from(4.0_f64) * t4147 * t26703 - F::cast_from(2.0_f64) * t13463 * t7842 - F::cast_from(2.0_f64) * t59498 * t2054 + t92862 - F::cast_from(12.0_f64) * t25168 * t101551 * t4272 - F::cast_from(2.0_f64) * t26713 * t4301 + F::cast_from(0.3289868133696452873e-1_f64) * t87779 - F::cast_from(2.0_f64) * t92847 * t1528 + F::cast_from(4.0_f64) * t13065 * t7830 - t92866 - t17090 * t7107 - t17052 * t7107 + F::cast_from(2.0_f64) * t17052 * t7092 + t92872 + F::cast_from(0.76763589786250567037e-1_f64) * t98921 - F::cast_from(0.76763589786250567037e-1_f64) * t98923 + F::cast_from(0.16449340668482264365e-1_f64) * t98927;
    t101569
}
