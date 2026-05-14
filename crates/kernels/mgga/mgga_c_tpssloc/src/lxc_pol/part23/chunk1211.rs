//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1211/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1211<F: Float>(t1484: F, t193: F, t202: F, t20800: F, t2522: F, t39593: F, t41254: F, t4310: F, t67112: F, t75950: F, t75951: F, t75952: F, t75978: F, t76017: F, t76018: F, t76020: F, t76024: F, t76025: F, t76497: F, t76532: F, t766: F, t870: F) -> (F,) {
    let t76543 = -t39593 + t75950 + t75951 - t75952 + 3.0 * t193 * t766 * t75978 + t193 * t202 * (t76497 + t76532) * t870 + 12.0 * t2522 * t4310 * t20800 + t76017 + 12.0 * t2522 * t67112 * t1484 + t41254 - t76018 + t76020 + t76024 + t76025;
    (t76543,)
}
