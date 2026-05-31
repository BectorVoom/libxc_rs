//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2413/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2413<F: Float>(t2610: F, t9541: F, t222: F, t39934: F, t2617: F, t9637: F, t2691: F, t812: F, t815: F) -> (F, F, F, F) {
    let t41086 = t9541 * t2610;
    let t41096 = F::cast_from(455.0_f64) / F::cast_from(243.0_f64) * t39934 * t222;
    let t41107 = t2617 * t9637;
    let t41115 = t812 * t815 * t2691;
    (t41086, t41096, t41107, t41115)
}
