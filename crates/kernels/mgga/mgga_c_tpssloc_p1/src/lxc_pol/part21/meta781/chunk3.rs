//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2714/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2714<F: Float>(t39499: F, t39502: F, t39505: F, t39508: F, t39518: F, t39521: F, t39529: F, t39539: F, t39549: F, t56365: F, t56366: F, t56367: F, t56368: F, t56369: F, t56372: F, t56375: F, t56381: F) -> F {
    let t57200 = t39499 + t39502 - t39505 - t39508 + t56365 + t56366 - t56367 + t39518 - t39521 - t56368 - t39529 - t56369 - t56372 + t39539 - t56375 - t56381 + t39549;
    t57200
}
