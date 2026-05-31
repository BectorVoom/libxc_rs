//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1280/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1280<F: Float>(t120672: F, t120675: F, t120677: F, t120678: F, t120680: F, t120683: F, t120687: F, t120691: F, t120692: F, t120697: F, t120699: F, t120702: F, t120703: F, t120708: F, t120709: F, t120711: F, t24999: F, t25965: F, t6517: F, t6539: F) -> F {
    let t120713 = -F::cast_from(4.0_f64) * t24999 * t6539 - F::cast_from(4.0_f64) * t25965 * t6517 - t120672 + F::cast_from(2.0_f64) * t120675 - t120677 - F::cast_from(4.0_f64) * t120678 - F::cast_from(4.0_f64) * t120680 - t120683 - t120687 - t120691 + F::cast_from(6.0_f64) * t120692 + t120697 + t120699 + t120702 + F::cast_from(6.0_f64) * t120703 - t120708 - F::cast_from(4.0_f64) * t120709 - F::cast_from(4.0_f64) * t120711;
    t120713
}
