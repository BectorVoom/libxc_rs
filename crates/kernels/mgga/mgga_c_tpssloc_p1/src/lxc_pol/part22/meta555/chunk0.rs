//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2055/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2055<F: Float>(t2559: F, t2570: F, t782: F, t9558: F, t2617: F, t9600: F, t786: F, t9569: F, t805: F, t222: F, t39934: F, t9637: F) -> (F, F, F, F, F, F, F) {
    let t41008 = t2559 * t2570;
    let t41011 = t782 * t9558;
    let t41052 = t2617 * t9600;
    let t41083 = t9569 * t786;
    let t41084 = t41083 * t805;
    let t41096 = F::cast_from(455.0_f64) / F::cast_from(243.0_f64) * t39934 * t222;
    let t41107 = t2617 * t9637;
    (t41008, t41011, t41052, t41083, t41084, t41096, t41107)
}
