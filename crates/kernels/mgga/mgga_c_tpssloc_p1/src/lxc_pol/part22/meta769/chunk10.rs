//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2620/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2620<F: Float>(t20246: F, t972: F, t1198: F, t15740: F, t18364: F, t3447: F, t45250: F, t53249: F, t53322: F, t53434: F, t53440: F, t53453: F, t53490: F, t6192: F, t66571: F, t66575: F, t66597: F, t66599: F, t68513: F) -> (F, F) {
    let t73113 = t20246 * t972;
    let t73126 = -t45250 + t66571 / F::new(216.0) - t53434 + t66575 / F::new(108.0) - F::new(5.0) / F::new(1296.0) * t53440 - t53453 + F::new(77.0) / F::new(486.0) * t73113 * t1198 - t66597 / F::new(1152.0) + t66599 / F::new(216.0) - F::new(7.0) / F::new(216.0) * t3447 * t53249 * t68513 - F::new(5.0) / F::new(162.0) * t53490 + F::new(5.0) / F::new(4608.0) * t15740 * t18364 - t53322 * t6192 / F::new(768.0);
    (t73113, t73126)
}
