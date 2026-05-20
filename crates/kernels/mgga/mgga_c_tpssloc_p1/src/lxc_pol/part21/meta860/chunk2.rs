//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3121/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3121<F: Float>(t14842: F, t4869: F, t11292: F, t6084: F, t1164: F, t3404: F, t3637: F, t43706: F, t4700: F, t6274: F, t63566: F, t63568: F, t63571: F, t63574: F, t63576: F, t63579: F, t63582: F, t63585: F, t63587: F, t63591: F, t63594: F) -> (F, F, F) {
    let t64536 = F::cast_from(0.2077903092681775651e3_f64) * t4869 * t14842;
    let t64537 = t11292 * t6084;
    let t64540 = F::cast_from(0.10389515463408878255e3_f64) * t1164 * t64537 * t3404;
    let t64545 = -F::new(6.0) * t3637 * t43706 * t4700 * t6274 - t63566 - t63568 - t63571 - t63574 - t63576 - t63579 - t63582 - t63585 + t63587 + t63591 + t63594 + t64536 + t64540;
    (t64536, t64540, t64545)
}
