//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1232/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1232<F: Float>(t7754: F, t8690: F, t113: F, t33096: F, t33098: F, t33100: F, t33101: F, t33127: F, t33131: F, t33134: F, t33139: F, t33158: F, t33162: F, t33747: F, t33748: F, t33756: F) -> F {
    let t33758 = t8690 * t7754;
    let t33759 = -t113 * t33756 - t33096 - t33098 - t33100 - F::new(2.0) * t33101 + t33127 + t33131 + t33134 - t33139 - t33158 - t33162 + t33747 + F::new(3.0) * t33748 + t33758;
    t33759
}
