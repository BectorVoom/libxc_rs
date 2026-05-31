//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2565/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2565<F: Float>(t11478: F, t4869: F, t11282: F, t1164: F, t14854: F, t4857: F, t14961: F, t3411: F, t11311: F, t1694: F, t44154: F, t11947: F, t3637: F, t4700: F, t5091: F, t51641: F, t51669: F, t51736: F, t51738: F, t51741: F, t51744: F) -> (F, F, F, F, F) {
    let t51870 = F::cast_from(0.5848223622634646207e0_f64) * t4869 * t11478;
    let t51874 = F::cast_from(0.30762056574649219973e4_f64) * t1164 * t11282 * t4857 * t14854;
    let t51880 = F::cast_from(0.70178683471615754484e1_f64) * t3411 * t14961;
    let t51884 = F::cast_from(0.12304822629859687989e5_f64) * t1164 * t44154 * t1694 * t11311;
    let t51885 = F::cast_from(6.0_f64) * t11947 * t3637 * t4700 * t5091 + t51641 + t51669 + t51736 + t51738 + t51741 + t51744 - t51870 - t51874 + t51880 + t51884;
    (t51870, t51874, t51880, t51884, t51885)
}
