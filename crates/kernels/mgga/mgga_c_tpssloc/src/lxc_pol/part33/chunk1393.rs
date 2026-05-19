//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1393/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1393<F: Float>(t107092: F, t107131: F, t107180: F, t107208: F, t1985: F, t26193: F, t28205: F, t107056: F, t1375: F, t1807: F, t1842: F, t20060: F, t2016: F, t28107: F, t28186: F, t3887: F, t539: F, t568: F, t74860: F, t7729: F, t81282: F, t97529: F, t97537: F, t97548: F) -> (F, F) {
    let t107210 = t107092 + t107131 + t107180 + t107208;
    let t107214 = t1985 * t26193 * t28205;
    let t107220 = F::new(6.0) * t1375 * t3887 * t28186 * t1842 + F::cast_from(0.23029076935875170111e0_f64) * t97529 - F::cast_from(0.16449340668482264365e-1_f64) * t107056 + t81282 + F::new(6.0) * t20060 * t7729 + F::new(3.0) * t1807 * t28107 * t568 + t539 * t107210 * t568 - F::cast_from(0.24674011002723396548e-1_f64) * t107214 + F::cast_from(0.11514538467937585055e0_f64) * t97537 - F::cast_from(0.11514538467937585055e0_f64) * t97548 - F::new(3.0) * t74860 * t2016;
    (t107210, t107220)
}
