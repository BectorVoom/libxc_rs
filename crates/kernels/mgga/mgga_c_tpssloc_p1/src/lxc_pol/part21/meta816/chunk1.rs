//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2874/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2874<F: Float>(t48140: F, t48143: F, t55716: F, t41656: F, t47705: F, t47707: F, t47709: F, t47711: F, t47713: F, t47715: F, t47717: F, t47724: F, t47730: F, t47732: F, t47738: F) -> (F, F) {
    let t60091 = t48140 * t48143 * t55716;
    let t60106 = F::new(32.0) / F::new(27.0) * t47705 - F::new(32.0) / F::new(81.0) * t47707 + F::new(8.0) / F::new(27.0) * t47709 + F::new(4.0) / F::new(27.0) * t47711 + F::new(20.0) / F::new(81.0) * t47713 - F::new(8.0) / F::new(9.0) * t47715 - F::new(4.0) / F::new(9.0) * t47717 - F::new(8.0) / F::new(9.0) * t47724 - F::new(16.0) / F::new(27.0) * t47730 + F::new(2.0) / F::new(9.0) * t47732 + F::new(4.0) / F::new(3.0) * t47738 - F::new(4.0) / F::new(27.0) * t41656;
    (t60091, t60106)
}
