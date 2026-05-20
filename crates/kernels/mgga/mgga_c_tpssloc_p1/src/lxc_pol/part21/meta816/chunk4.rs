//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2877/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2877<F: Float>(t47787: F, t59727: F, t59732: F, t59735: F, t59738: F, t59744: F, t59748: F, t59753: F, t59757: F, t59759: F, t59761: F, t59765: F, t59769: F) -> F {
    let t60147 = -F::new(10.0) / F::new(27.0) * t59727 + F::new(4.0) / F::new(3.0) * t59732 - F::new(40.0) / F::new(27.0) * t59735 + F::new(16.0) / F::new(3.0) * t59738 + F::new(56.0) / F::new(81.0) * t47787 + F::new(4.0) / F::new(3.0) * t59744 - F::new(2.0) * t59748 + F::new(40.0) / F::new(9.0) * t59753 - F::new(8.0) * t59757 + F::new(4.0) / F::new(3.0) * t59759 - F::new(8.0) / F::new(9.0) * t59761 - F::new(2.0) * t59765 + F::new(4.0) / F::new(3.0) * t59769;
    t60147
}
