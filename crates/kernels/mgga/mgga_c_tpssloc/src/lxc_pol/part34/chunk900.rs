//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 900/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk900<F: Float>(t13598: F, t13642: F, t17149: F, t17165: F, t17175: F, t17286: F, t17288: F, t17290: F, t21161: F, t21168: F, t21181: F, t21183: F, t21186: F, t21188: F) -> F {
    let t21298 = -F::new(0.27385555555555555556e0) * t13642 + F::new(0.49293999999999999999e0) * t21161 - F::new(0.39862222222222222223e0) * t13598 + F::new(0.19931111111111111111e0) * t17149 - F::new(0.59793333333333333333e0) * t17165 + F::new(0.29896666666666666667e0) * t17175 - F::new(0.82156666666666666668e-1) * t21168 + F::new(0.1898925e1) * t21181 + F::new(0.3071625e0) * t21183 - F::new(0.76790625e-1) * t21186 + F::new(0.142419375e1) * t21188 + F::new(0.5477111111111111111e-1) * t17286 - F::new(0.32862666666666666666e0) * t17288 + F::new(0.16431333333333333333e0) * t17290;
    t21298
}
