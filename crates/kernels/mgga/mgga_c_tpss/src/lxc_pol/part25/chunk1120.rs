//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1120/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1120<F: Float>(t11845: F, t11850: F, t11873: F, t11876: F, t11911: F, t11932: F, t15237: F, t15239: F, t15241: F, t15243: F, t15245: F, t15259: F, t15321: F, t15324: F, t15327: F, t15330: F, t15385: F, t15406: F, t9192: F, t9221: F, t9297: F, t9306: F) -> F {
    let t15408 = -t9297 + F::new(0.91285185185185185187e-1) * t9192 - t11845 + F::new(0.18257037037037037037e0) * t11850 - t9306 + F::new(0.82156666666666666667e-1) * t15237 + F::new(0.66437037037037037037e-1) * t15239 - F::new(0.19931111111111111111e0) * t15241 - F::new(0.99655555555555555557e-1) * t15243 - F::new(0.10954222222222222222e0) * t15245 + t15385 + F::new(0.13287407407407407407e0) * t11873 - t11876 - t11911 + F::new(0.13287407407407407408e0) * t9221 + F::new(0.36514074074074074073e-1) * t11932 + F::new(0.33218518518518518518e0) * t15259 + F::new(0.32862666666666666666e0) * t15321 + F::new(0.49293999999999999999e0) * t15324 + F::new(0.16431333333333333333e0) * t15327 - F::new(0.54771111111111111112e-1) * t15330 + t15406;
    t15408
}
