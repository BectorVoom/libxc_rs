//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1995/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1995<F: Float>(t100664: F, t100705: F, t100708: F, t100766: F, t100788: F, t101226: F, t18196: F, t1877: F, t2057: F, t24191: F, t24339: F, t2522: F, t25898: F, t25901: F, t25938: F, t26563: F, t28795: F, t29106: F, t29157: F, t46341: F, t6841: F, t6848: F, t7656: F, t7845: F, t92276: F, t92319: F) -> F {
    let t102048 = -F::new(3.0) / F::new(2.0) * t24191 * t100664 - F::new(3.0) * t24191 * t100705 + F::new(3.0) * t2522 * t7845 * t25938 + F::new(3.0) * t46341 * t29157 - t1877 * t92276 * t7656 - F::new(3.0) * t24191 * t100766 + F::new(6.0) * t26563 * t100708 - F::new(6.0) * t26563 * t100788 + F::new(3.0) * t2522 * t7845 * t25901 - F::new(3.0) * t92319 * t25898 + t1877 * t2057 * t18196 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t2522 * t29106 * t6841 - t1877 * t101226 * t6848 / F::new(2.0) - t1877 * t24339 * t28795 / F::new(2.0);
    t102048
}
