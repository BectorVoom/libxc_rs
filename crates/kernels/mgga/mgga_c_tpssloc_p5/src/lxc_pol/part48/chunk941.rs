//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 941/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk941<F: Float>(t23896: F, t24465: F, t55571: F, t8657: F, t1873: F, t23917: F, t3941: F, t6534: F, t7056: F, t45560: F, t20173: F, t31814: F) -> (F, F, F, F, F, F) {
    let t114515 = F::new(27.0) * t24465 * t23896;
    let t114517 = F::new(27.0) * t55571 * t8657;
    let t114520 = F::new(27.0) * t3941 * t23917 * t1873;
    let t114525 = F::new(54.0) * t3941 * t7056 * t6534;
    let t114527 = F::new(27.0) * t45560 * t8657;
    let t114529 = F::new(54.0) * t20173 * t31814;
    (t114515, t114517, t114520, t114525, t114527, t114529)
}
