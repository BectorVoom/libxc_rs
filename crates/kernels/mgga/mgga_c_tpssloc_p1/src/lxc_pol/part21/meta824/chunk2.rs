//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2897/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2897<F: Float>(t41656: F, t41658: F, t41675: F, t41684: F, t41863: F, t41870: F, t41872: F, t47738: F, t48103: F, t48116: F, t59655: F, t60091: F, t60150: F, t60153: F, t60156: F) -> F {
    let t60465 = F::cast_from(0.11958666666666666667e1_f64) * t47738 + F::cast_from(0.48685432098765432099e0_f64) * t48103 - F::cast_from(0.13287407407407407408e0_f64) * t41656 - F::cast_from(0.88582716049382716053e-1_f64) * t41658 + F::cast_from(0.26574814814814814816e0_f64) * t41675 + F::cast_from(0.62007901234567901237e0_f64) * t41684 + F::cast_from(0.486854320987654321e0_f64) * t41863 - F::cast_from(0.91285185185185185187e-1_f64) * t41870 - F::cast_from(0.30428395061728395062e-1_f64) * t41872 + F::cast_from(0.48685432098765432097e-1_f64) * t48116 - F::new(0.197176e1) * t60091 - F::cast_from(0.71752000000000000001e1_f64) * t59655 + F::new(0.3071625e0) * t60150 + F::cast_from(0.65725333333333333333e0_f64) * t60153 - F::cast_from(0.1460562962962962963e0_f64) * t60156;
    t60465
}
