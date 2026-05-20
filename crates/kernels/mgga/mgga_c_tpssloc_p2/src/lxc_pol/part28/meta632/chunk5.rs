//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1994/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1994<F: Float>(t87898: F, t87901: F, t87910: F, t87915: F, t87927: F, t87931: F, t10109: F, t7106: F, t13058: F, t13461: F, t1528: F, t24305: F, t25168: F, t26728: F, t2718: F, t4272: F, t4300: F, t4301: F, t7087: F, t82294: F, t82296: F, t85079: F, t855: F, t87924: F) -> (F, F, F, F, F) {
    let t92954 = F::cast_from(0.52089578783527170489e-1_f64) * t87898;
    let t92955 = F::cast_from(0.3289868133696452873e-1_f64) * t87901;
    let t92960 = F::cast_from(0.16449340668482264365e-1_f64) * t87910;
    let t92961 = F::cast_from(0.16449340668482264365e-1_f64) * t87915;
    let t92966 = F::cast_from(0.9869604401089358619e-1_f64) * t87927;
    let t92976 = F::cast_from(0.15352717957250113407e0_f64) * t87931;
    let t92981 = t10109 * t7106;
    let t92985 = F::cast_from(0.9869604401089358619e-1_f64) * t87924 - t92966 - t85079 * t1528 - F::cast_from(0.20835831513410868196e0_f64) * t82294 - F::cast_from(0.23029076935875170111e0_f64) * t82296 - F::new(2.0) * t24305 * t4301 - F::new(6.0) * t25168 * t26728 * t13058 - t7087 * t13461 - t92976 + F::new(4.0) * t855 * t2718 * t7106 * t4300 - F::new(12.0) * t25168 * t92981 * t4272;
    (t92954, t92955, t92960, t92961, t92985)
}
