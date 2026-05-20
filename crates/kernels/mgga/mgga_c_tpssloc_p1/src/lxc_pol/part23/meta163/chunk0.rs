//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 762/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk762<F: Float>(t1336: F, t1814: F, t1838: F, t1840: F, t5234: F, t544: F, t564: F, t6378: F, t6448: F, t6451: F, t6454: F, t6456: F, t6458: F) -> F {
    let t6460 = F::new(2.0) * t1336 * t6448 - F::new(2.0) * t1336 * t6451 - t1336 * t6454 - t1336 * t6456 + F::new(2.0) * t1814 * t1840 - F::new(2.0) * t1838 * t5234 + t544 * t6458 + t564 * t6378;
    t6460
}
