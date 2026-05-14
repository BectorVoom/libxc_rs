//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1138/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1138<F: Float>(t120705: F, t22574: F, t8643: F, t33085: F, t6535: F, t22461: F, t7461: F, t120672: F, t120675: F, t120677: F, t120678: F, t120680: F, t120683: F, t120687: F, t120691: F, t120692: F, t120697: F, t120699: F, t120702: F, t120703: F, t24999: F, t25965: F, t6517: F, t6539: F) -> (F,) {
    let t120708 = 6.0 * t22574 * t8643 * t120705;
    let t120709 = t33085 * t6535;
    let t120711 = t22461 * t7461;
    let t120713 = -4.0 * t24999 * t6539 - 4.0 * t25965 * t6517 - t120672 + 2.0 * t120675 - t120677 - 4.0 * t120678 - 4.0 * t120680 - t120683 - t120687 - t120691 + 6.0 * t120692 + t120697 + t120699 + t120702 + 6.0 * t120703 - t120708 - 4.0 * t120709 - 4.0 * t120711;
    (t120713,)
}
