//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1334/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1334<F: Float>(t1985: F, t8458: F, t90739: F, t114187: F, t1992: F, t22635: F, t31090: F, t5353: F, t114160: F, t6888: F, t7691: F, t26189: F, t31137: F) -> (F, F, F, F, F) {
    let t120324 = F::new(0.16449340668482264365e-1) * t1985 * t90739 * t8458;
    let t120327 = F::new(0.82246703342411321825e-2) * t114187;
    let t120334 = F::new(0.3289868133696452873e-1) * t1992 * t22635 * t31090 * t5353;
    let t120337 = F::new(0.3289868133696452873e-1) * t6888 * t114160 * t7691;
    let t120340 = F::new(0.3289868133696452873e-1) * t6888 * t31137 * t26189;
    (t120324, t120327, t120334, t120337, t120340)
}
