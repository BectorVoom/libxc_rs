//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2876/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2876<F: Float>(t41904: F, t59688: F, t59692: F, t59694: F, t59698: F, t59700: F, t59702: F, t59704: F, t59708: F, t59713: F, t59717: F, t59721: F) -> F {
    let t60133 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t59688 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t59692 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t59694 + t41904 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t59698 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t59700 + F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t59702 + F::cast_from(20.0_f64) / F::cast_from(81.0_f64) * t59704 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t59708 - F::cast_from(80.0_f64) / F::cast_from(81.0_f64) * t59713 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t59717 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t59721;
    t60133
}
