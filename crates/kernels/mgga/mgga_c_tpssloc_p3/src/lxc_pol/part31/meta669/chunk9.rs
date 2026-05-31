//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1986/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1986<F: Float>(t13053: F, t17049: F, t17090: F, t2053: F, t24297: F, t24305: F, t2597: F, t2713: F, t2718: F, t29056: F, t29080: F, t5637: F, t5658: F, t7092: F, t7842: F, t855: F, t92938: F, t99003: F, t99019: F) -> F {
    let t101797 = F::cast_from(2.0_f64) * t855 * t2718 * t2053 * t17049 + F::cast_from(4.0_f64) * t2597 * t29080 - t2713 * t29056 - F::cast_from(2.0_f64) * t13053 * t7842 - t24297 * t5658 + F::cast_from(0.38381794893125283518e-1_f64) * t99003 + F::cast_from(2.0_f64) * t17090 * t7092 + F::cast_from(2.0_f64) * t24305 * t5637 - t92938 + F::cast_from(2.0_f64) * t24297 * t5637 + F::cast_from(0.16449340668482264365e-1_f64) * t99019;
    t101797
}
