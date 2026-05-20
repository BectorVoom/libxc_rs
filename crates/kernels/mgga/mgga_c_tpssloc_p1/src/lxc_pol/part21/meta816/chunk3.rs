//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2876/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2876<F: Float>(t41904: F, t59688: F, t59692: F, t59694: F, t59698: F, t59700: F, t59702: F, t59704: F, t59708: F, t59713: F, t59717: F, t59721: F) -> F {
    let t60133 = F::new(8.0) / F::new(27.0) * t59688 + F::new(4.0) / F::new(3.0) * t59692 - F::new(4.0) / F::new(27.0) * t59694 + t41904 + F::new(2.0) / F::new(3.0) * t59698 - F::new(8.0) / F::new(9.0) * t59700 + F::new(8.0) / F::new(27.0) * t59702 + F::new(20.0) / F::new(81.0) * t59704 - F::new(10.0) / F::new(27.0) * t59708 - F::new(80.0) / F::new(81.0) * t59713 + F::new(4.0) / F::new(3.0) * t59717 - F::new(4.0) / F::new(9.0) * t59721;
    t60133
}
