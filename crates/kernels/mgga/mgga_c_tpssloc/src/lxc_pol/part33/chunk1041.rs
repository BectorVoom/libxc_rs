//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1041/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1041<F: Float>(t21723: F, t3315: F, t11190: F, t11444: F, t14702: F, t18203: F, t18219: F, t18229: F, t21760: F, t21764: F, t21767: F, t21771: F, t21774: F, t21778: F) -> (F, F) {
    let t21961 = t21723 * t3315;
    let t21963 = F::new(0.96491876992155210402e2) * t11190 * t21961;
    let t21975 = -t11444 + F::new(0.2283111111111111111e-1) * t14702 + F::new(0.11415555555555555555e-1) * t18203 - F::new(0.34246666666666666665e-1) * t18219 - F::new(0.17123333333333333333e-1) * t18229 + F::new(0.19025925925925925925e-1) * t21760 - F::new(0.68493333333333333331e-1) * t21764 - F::new(0.34246666666666666665e-1) * t21767 + F::new(0.10274e0) * t21771 + F::new(0.10274e0) * t21774 + F::new(0.17123333333333333333e-1) * t21778;
    (t21963, t21975)
}
