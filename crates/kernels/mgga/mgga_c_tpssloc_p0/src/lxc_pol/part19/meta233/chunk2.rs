//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 942/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk942<F: Float>(t1068: F, t3213: F, t3215: F, t390: F, t10521: F, t10528: F, t10607: F, t10625: F, t10627: F, t10635: F, t1070: F, t10711: F, t10729: F, t10733: F, t10849: F, t10851: F, t11087: F, t193: F, t336: F) -> (F, F) {
    let t11091 = t3213 * t1068;
    let t11094 = F::cast_from(1.0_f64) / t3215 / t390;
    let t11098 = t1070 * t11087 * t193 * t336 + F::cast_from(2.0_f64) * t11091 * t11094 * t193 * t336 - t10521 + t10528 - t10607 - t10625 - t10627 - t10635 - t10711 - t10729 + t10733 + t10849 + t10851;
    (t11094, t11098)
}
