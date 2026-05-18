//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 997/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk997<F: Float>(t10110: F, t114760: F, t121371: F, t126249: F, t126264: F, t126278: F, t127798: F, t127803: F, t127814: F, t127818: F, t127829: F, t1527: F, t25348: F, t26713: F, t2718: F, t29060: F, t33398: F, t33433: F, t4147: F, t4268: F, t5636: F, t6627: F, t7517: F, t7830: F, t855: F, t8562: F) -> F {
    let t127833 = -F::new(0.82246703342411321825e-2) * t127798 - t126249 + F::new(4.0) * t25348 * t7830 - F::new(0.82246703342411321825e-2) * t127803 + F::new(4.0) * t855 * t2718 * t33398 * t1527 + t126264 - F::new(0.76763589786250567036e-1) * t121371 + F::new(4.0) * t4147 * t33433 + F::new(0.16449340668482264365e-1) * t127814 + t114760 - F::new(0.6579736267392905746e-1) * t127818 - t126278 + F::new(4.0) * t4268 * t33433 + F::new(2.0) * t6627 * t29060 - F::new(6.0) * t855 * t10110 * t8562 * t5636 + F::new(0.3289868133696452873e-1) * t127829 + F::new(4.0) * t26713 * t7517;
    t127833
}
