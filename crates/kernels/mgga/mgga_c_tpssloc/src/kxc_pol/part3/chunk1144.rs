//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1144/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1144<F: Float>(t14704: F, t14710: F, t14722: F, t11215: F, t11217: F, t14720: F, t14733: F, t14738: F, t14742: F, t14746: F, t14751: F, t14755: F, t14766: F) -> (F, F, F) {
    let t14868 = F::new(0.19931111111111111111e0) * t14704;
    let t14870 = F::new(0.10954222222222222222e0) * t14710;
    let t14886 = F::new(0.39862222222222222222e0) * t14722;
    let t14887 = -F::new(0.10954222222222222222e0) * t11215 - F::new(0.54771111111111111111e-1) * t11217 + F::new(0.91285185185185185185e-1) * t14766 + F::new(0.13287407407407407408e0) * t14720 - F::new(0.39862222222222222222e0) * t14738 - F::new(0.19931111111111111111e0) * t14742 - F::new(0.11958666666666666667e1) * t14733 + F::new(0.11958666666666666667e1) * t14751 + F::new(0.59793333333333333334e0) * t14755 + F::new(0.17938e1) * t14746 - t14886;
    (t14868, t14870, t14887)
}
