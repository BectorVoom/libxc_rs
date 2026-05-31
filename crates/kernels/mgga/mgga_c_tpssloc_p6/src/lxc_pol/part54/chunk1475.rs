//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1475/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1475<F: Float>(t122667: F, t122671: F, t122678: F, t122681: F, t122692: F, t122696: F, t2114: F, t2165: F, t26114: F, t26179: F, t26870: F, t26967: F, t32318: F, t32365: F, t4028: F, t7156: F, t7264: F, t7458: F, t7890: F, t7983: F, t8835: F) -> F {
    let t125017 = -t2114 * t26870 - t2165 * t26967 - F::cast_from(2.0_f64) * t26114 * t8835 - F::cast_from(2.0_f64) * t26179 * t8835 - F::cast_from(2.0_f64) * t32318 * t7458 - F::cast_from(2.0_f64) * t32365 * t4028 - t7156 * t7983 - t7264 * t7890 + t122667 + t122671 + t122678 - t122681 - t122692 + t122696;
    t125017
}
