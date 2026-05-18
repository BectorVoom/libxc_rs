//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1398/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1398<F: Float>(t116324: F, t116330: F, t116368: F, t116375: F, t116377: F, t116383: F, t123326: F, t123330: F, t123331: F, t123332: F, t123334: F, t123335: F, t1396: F, t1858: F, t31924: F, t33783: F, t7020: F, t7416: F, t7774: F, t8111: F) -> F {
    let t123336 = t1396 * t33783 + t1858 * t31924 + t7020 * t8111 + t7416 * t7774 + t116324 + t116330 + t116368 + t116375 + t116377 + t116383 + t123326 + t123330 + t123331 + t123332 + t123334 + t123335;
    t123336
}
