//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 380/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk380<F: Float>(t1021: F, t1616: F, t248: F, t1044: F, t1539: F, t1020: F, t1038: F, t1041: F, t1607: F, t1612: F, t378: F, t973: F, t997: F) -> (F, F, F) {
    let t1618 = t248 * t1021 * t1616;
    let t1622 = t248 * t1044 * t1539;
    let t1625 = t997 + t973 * t1607 / F::cast_from(288.0_f64) + t1612 * t378 / F::cast_from(3072.0_f64) + t1020 * t1618 / F::cast_from(3072.0_f64) + t1038 + t1041 * t1622 / F::cast_from(4608.0_f64);
    (t1618, t1622, t1625)
}
