//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1042/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1042<F: Float>(t11459: F, t14702: F, t18203: F, t18219: F, t18229: F, t21760: F, t21764: F, t21767: F, t21771: F, t21774: F, t21778: F, t423: F) -> F {
    let t21988 = -t11459 + F::new(0.23744444444444444444e-1) * t14702 + F::new(0.11872222222222222222e-1) * t18203 - F::new(0.35616666666666666666e-1) * t18219 - F::new(0.17808333333333333333e-1) * t18229 + F::new(0.19787037037037037037e-1) * t21760 - F::new(0.71233333333333333332e-1) * t21764 - F::new(0.35616666666666666666e-1) * t21767 + F::new(0.10685e0) * t21771 + F::new(0.10685e0) * t21774 + F::new(0.17808333333333333333e-1) * t21778;
    let t21990 = F::new(0.621814e-1) * t21988 * t423;
    t21990
}
