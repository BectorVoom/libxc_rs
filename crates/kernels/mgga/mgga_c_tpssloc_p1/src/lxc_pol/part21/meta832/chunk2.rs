//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2934/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2934<F: Float>(t10254: F, t17691: F, t41831: F, t41863: F, t41870: F, t41872: F, t48087: F, t48096: F, t48098: F, t48103: F, t48116: F, t60091: F, t60153: F, t60156: F) -> (F, F) {
    let t61103 = t10254 * t17691;
    let t61124 = -F::new(4.0) / F::new(3.0) * t48087 - F::new(10.0) / F::new(27.0) * t41831 + F::new(20.0) / F::new(27.0) * t48096 - F::new(2.0) / F::new(9.0) * t48098 - F::new(80.0) / F::new(81.0) * t48103 - F::new(80.0) / F::new(81.0) * t41863 + F::new(5.0) / F::new(27.0) * t41870 + F::new(5.0) / F::new(81.0) * t41872 - F::new(8.0) / F::new(81.0) * t48116 + F::new(4.0) * t60091 - F::new(4.0) / F::new(3.0) * t60153 + F::new(8.0) / F::new(27.0) * t60156;
    (t61103, t61124)
}
