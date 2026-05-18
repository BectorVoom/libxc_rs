//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1302/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1302<F: Float>(t118345: F, t118347: F, t125982: F, t125988: F, t125991: F, t126004: F, t126015: F, t1396: F, t1398: F, t1852: F, t2170: F, t27930: F, t32649: F, t34401: F, t5364: F, t7416: F, t7426: F, t8111: F, t8119: F, t8927: F) -> F {
    let t126018 = F::new(2.0) * t8111 * t7426 + F::new(2.0) * t125982 + F::new(2.0) * t7416 * t8119 + F::new(2.0) * t2170 * t27930 + t118345 + t118347 + t125988 + t5364 * t8927 + t1852 * t32649 + t125991 + t1396 * t34401 + t1398 * (t126004 + t126015);
    t126018
}
