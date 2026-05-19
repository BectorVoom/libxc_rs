//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1380/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1380<F: Float>(t105223: F, t105232: F, t105240: F, t105269: F, t105443: F, t105466: F, t105474: F, t105508: F, t105700: F, t105723: F, t1527: F, t17092: F, t1911: F, t21033: F, t21049: F, t25188: F, t25348: F, t2718: F, t28311: F, t28431: F, t40890: F, t4147: F, t5637: F, t5658: F, t7517: F, t82219: F, t855: F, t86870: F, t86903: F, t87779: F, t98117: F, t98921: F, t98923: F, t98927: F, t98932: F) -> F {
    let t105726 = F::cast_from(0.24674011002723396547e-1_f64) * t87779 - F::cast_from(0.38381794893125283518e0_f64) * t86903 - F::cast_from(0.15626873635058151147e0_f64) * t86870 - t82219 + F::new(12.0) * t17092 * t7517 - F::new(18.0) * t4147 * t28311 + F::new(6.0) * t25348 * t5637 - F::new(3.0) * t25188 * t5658 + t105700 + t105466 + F::cast_from(0.11514538467937585055e0_f64) * t98932 + F::cast_from(0.24674011002723396548e-1_f64) * t98927 + F::cast_from(0.23029076935875170111e0_f64) * t98117 + t105723 + t105269 + F::cast_from(0.14804406601634037928e0_f64) * t105223 + t105443 + F::cast_from(0.49348022005446793095e-1_f64) * t105240 + F::cast_from(0.49348022005446793095e-1_f64) * t105474 + t105508 - F::cast_from(0.19739208802178717238e0_f64) * t105232 + F::cast_from(0.11514538467937585055e0_f64) * t98921 - F::cast_from(0.11514538467937585055e0_f64) * t98923 + F::new(2.0) * t855 * t2718 * t1911 * t21033 + F::new(24.0) * t855 * t40890 * t1911 * t21049 + F::new(6.0) * t855 * t2718 * t28431 * t1527;
    t105726
}
