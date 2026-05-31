//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1436/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1436<F: Float>(t22479: F, t3941: F, t671: F, t2363: F, t6534: F, t1873: F, t55344: F, t12524: F, t23893: F, t23896: F, t12529: F, t12532: F, t2022: F, t2319: F, t23877: F, t23880: F, t577: F, t7010: F, t83973: F, t83979: F, t83980: F, t83984: F, t83988: F, t83991: F, t83993: F, t83999: F, t84001: F, t84003: F, t84004: F, t9416: F) -> F {
    let t84009 = F::cast_from(81.0_f64) * t3941 * t22479 * t671;
    let t84012 = F::cast_from(81.0_f64) * t3941 * t6534 * t2363;
    let t84014 = F::cast_from(81.0_f64) * t55344 * t1873;
    let t84016 = F::cast_from(162.0_f64) * t12524 * t23893;
    let t84018 = F::cast_from(81.0_f64) * t12524 * t23896;
    let t84019 = F::cast_from(0.45e1_f64) * t83973 * t577 + F::cast_from(81.0_f64) * t23880 * t12532 + t83979 + F::cast_from(81.0_f64) * t83980 * t2319 + t83984 + F::cast_from(27.0_f64) * t2022 * t12529 + t83988 + t83991 + t83993 + F::cast_from(0.405e2_f64) * t23877 * t2363 + F::cast_from(0.135e2_f64) * t7010 * t9416 + t83999 + t84001 + t84003 + F::cast_from(0.405e2_f64) * t84004 * t671 + t84009 + t84012 + t84014 + t84016 + t84018;
    t84019
}
