//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 743/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk743<F: Float>(t23218: F, t6553: F, t1880: F, t2553: F, t6554: F, t6552: F, t218: F, t23150: F, t212: F, t252: F, t23171: F, t23168: F, t6556: F) -> (F, F, F, F, F, F, F, F) {
    let t23219 = t6553 * t23218;
    let t23220 = t1880 * t23219;
    let t23222 = t6554 * t2553;
    let t23223 = t6553 * t23222;
    let t23224 = t6552 * t23223;
    let t23226 = t218 * t23150;
    let t23228 = t212 * t252;
    let t23229 = t23228 * t6554;
    let t23230 = t23171 * t23229;
    let t23231 = F::cast_from(0.82246703342411321824e-2_f64) * t23230;
    let t23232 = t23168 * t6556;
    (t23220, t23222, t23224, t23226, t23228, t23230, t23231, t23232)
}
