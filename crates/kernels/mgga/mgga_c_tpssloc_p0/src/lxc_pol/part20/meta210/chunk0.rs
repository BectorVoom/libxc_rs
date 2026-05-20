//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1246/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1246<F: Float>(t5154: F, t763: F, t1787: F, t67: F, t758: F, t193: F, t533: F) -> (F, F, F, F) {
    let t5155 = t5154 * t763;
    let t5156 = F::cast_from(0.5848223622634646207e0_f64) * t5155;
    let t5157 = t1787 * t67;
    let t5158 = t5157 * t758;
    let t5159 = F::cast_from(0.18311447306006545054e-3_f64) * t5158;
    let t5160 = t193 * t533;
    (t5156, t5157, t5159, t5160)
}
