//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1360/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1360<F: Float>(t1599: F, t17588: F, t1920: F, t1956: F, t21134: F, t21614: F, t225: F, t25810: F, t28491: F, t28515: F, t345: F, t387: F, t5844: F, t6687: F, t6689: F, t6690: F, t70978: F, t7553: F, t7561: F, t7565: F, t7600: F, t88138: F, t89672: F, t99210: F, t99214: F, t99895: F, t99948: F, t99956: F) -> F {
    let t106526 = F::new(0.82246703342411321826e-2) * t6687 * t25810 * t28515 + F::new(0.82246703342411321826e-2) * t6687 * t99214 * t7553 + F::new(0.36554090374405031922e-2) * t99948 + F::new(0.27415567780803773942e-2) * t6687 * t6689 * t6690 * t21134 + F::new(0.10966227112321509577e-1) * t6687 * t88138 * t28491 - F::new(0.54831135561607547883e-2) * t99956 + F::new(0.24674011002723396548e-1) * t6687 * t1599 * t99210 - F::new(0.24674011002723396548e-1) * t6687 * t5844 * t7561 + F::new(0.82246703342411321825e-2) * t1920 * t345 * t21614 * t225 * t387 - t70978 * t1956 + F::new(0.54831135561607547884e-2) * t89672 - F::new(0.24674011002723396548e-1) * t6687 * t99895 * t7565 + F::new(12.0) * t17588 * t7600;
    t106526
}
