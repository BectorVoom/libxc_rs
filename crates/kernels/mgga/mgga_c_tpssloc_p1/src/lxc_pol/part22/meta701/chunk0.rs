//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2286/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2286<F: Float>(t15492: F, t5002: F, t1174: F, t18237: F, t3431: F, t6187: F, t698: F, t1227: F, t13969: F, t18341: F, t18345: F, t18589: F) -> (F, F, F, F, F, F) {
    let t65998 = t5002 * t15492;
    let t66001 = t1174 * t3431 * t18237;
    let t66015 = t1174 * t698 * t6187;
    let t66024 = t1227 * t13969 * t18341;
    let t66027 = t1227 * t13969 * t18345;
    let t66052 = t1227 * t13969 * t18589;
    (t65998, t66001, t66015, t66024, t66027, t66052)
}
