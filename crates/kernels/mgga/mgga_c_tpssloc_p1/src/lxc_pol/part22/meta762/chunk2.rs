//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2566/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2566<F: Float>(t51246: F, t5989: F, t1098: F, t21988: F, t1119: F, t50834: F, t51257: F, t63291: F, t63306: F, t63308: F, t63841: F, t63843: F, t63845: F, t71333: F, t71335: F, t71337: F) -> (F, F, F) {
    let t71876 = F::cast_from(6.0_f64) * t51246 * t5989;
    let t71877 = t21988 * t1098;
    let t71879 = F::cast_from(1.0_f64) * t71877 * t1119;
    let t71902 = -F::cast_from(0.103295e1_f64) * t63291 + F::cast_from(0.34431666666666666666e0_f64) * t63306 - F::cast_from(0.5738611111111111111e0_f64) * t63308 - F::cast_from(0.34731666666666666667e-1_f64) * t71333 + F::cast_from(0.69463333333333333333e-1_f64) * t71335 - F::cast_from(0.41678e0_f64) * t71337 + t51257 - F::cast_from(0.16068111111111111111e1_f64) * t50834 - F::cast_from(0.9261777777777777778e-1_f64) * t63841 - F::cast_from(0.41678e0_f64) * t63843 + F::cast_from(0.69463333333333333333e-1_f64) * t63845;
    (t71876, t71879, t71902)
}
