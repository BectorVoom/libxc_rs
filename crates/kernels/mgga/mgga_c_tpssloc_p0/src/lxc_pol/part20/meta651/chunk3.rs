//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2397/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2397<F: Float>(t47681: F, t47686: F, t47691: F, t47695: F, t47699: F, t47703: F, t47705: F, t48085: F, t48087: F, t48090: F, t48092: F, t48096: F) -> (F, F) {
    let t49127 = -F::cast_from(0.99342e0_f64) * t48085 + F::cast_from(0.99342e0_f64) * t48087 + F::cast_from(0.49671e0_f64) * t48090 - F::cast_from(0.82785e-1_f64) * t48092 - F::cast_from(0.89459259259259259259e0_f64) * t47681 + F::cast_from(0.36230999999999999999e1_f64) * t47686 - F::cast_from(0.60384999999999999999e0_f64) * t47691 - F::cast_from(0.60384999999999999999e0_f64) * t47695 - F::cast_from(0.20128333333333333333e0_f64) * t47699 - F::cast_from(0.543465e1_f64) * t47703 + F::cast_from(0.80513333333333333334e0_f64) * t47705;
    let t49139 = F::cast_from(0.27595e0_f64) * t48096;
    (t49127, t49139)
}
