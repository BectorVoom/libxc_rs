//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1264/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1264<F: Float>(t20217: F, t3247: F, t21826: F, t300: F, t21746: F, t699: F, t21750: F, t21794: F, t21780: F, t3287: F, t3270: F, t21801: F) -> (F, F, F, F, F, F, F, F) {
    let t71176 = t3247 * t20217;
    let t71231 = t300 * t21826;
    let t71335 = t699 * t21746;
    let t71337 = t699 * t21750;
    let t71408 = t699 * t21794;
    let t71445 = t3287 * t21780;
    let t71448 = t3270 * t21780;
    let t71470 = t699 * t21801;
    (t71176, t71231, t71335, t71337, t71408, t71445, t71448, t71470)
}
