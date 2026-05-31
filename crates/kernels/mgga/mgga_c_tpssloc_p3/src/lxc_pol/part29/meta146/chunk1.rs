//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 801/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk801<F: Float>(t390: F, t1070: F, t193: F, t2786: F, t2789: F, t2796: F, t2839: F, t2847: F, t2937: F, t2939: F, t2942: F, t2946: F, t2950: F, t2954: F, t3209: F, t3213: F, t336: F) -> (F, F, F) {
    let t3215 = t390 * t390;
    let t3216 = F::cast_from(1.0_f64) / t3215;
    let t3219 = t1070 * t193 * t3209 * t336 - t193 * t3213 * t3216 * t336 - t2786 + t2789 - t2796 + t2839 + t2847 + t2937 + t2939 - t2942 + t2946 - t2950 - t2954;
    (t3215, t3216, t3219)
}
