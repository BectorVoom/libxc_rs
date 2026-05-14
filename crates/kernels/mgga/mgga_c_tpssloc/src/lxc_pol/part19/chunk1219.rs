//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1219/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1219<F: Float>(t10623: F, t2952: F, t10632: F, t41825: F, t41827: F, t959: F, t10605: F, t2940: F, t41977: F, t942: F, t951: F, t41992: F, t41998: F, t42002: F, t42005: F, t42025: F, t42031: F, t42097: F, t42105: F) -> (F, F, F, F, F) {
    let t42682 = 0.10389515463408878255e3 * t10623 * t2952;
    let t42686 = 0.12304822629859687989e5 * t959 * t41825 * t41827 * t10632;
    let t42688 = 0.23392894490538584828e1 * t2940 * t10605;
    let t42692 = 0.5848223622634646207e0 * t959 * t942 * t41977 * t951;
    let t42693 = t41992 - t41998 - t42002 + t42005 - t42682 + t42025 - t42031 + t42097 + t42105 + t42686 - t42688 - t42692;
    (t42682, t42686, t42688, t42692, t42693)
}
