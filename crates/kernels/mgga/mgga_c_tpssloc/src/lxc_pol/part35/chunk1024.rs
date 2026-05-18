//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1024/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1024<F: Float>(t13769: F, t17863: F, t17800: F, t4514: F, t17794: F, t4531: F, t10339: F, t13896: F, t17764: F, t17770: F, t17827: F, t17850: F, t21410: F, t21413: F, t2986: F, t973: F) -> F {
    let t21416 = t13769 * t17863;
    let t21419 = t17800 * t4514;
    let t21422 = t4531 * t17794;
    let t21429 = -F::new(0.83333333333333333331e-3) * t17827 - F::new(0.22222222222222222221e-2) * t973 * t21410 + F::new(0.11111111111111111111e-2) * t2986 * t21413 - F::new(0.11111111111111111111e-2) * t2986 * t21416 - F::new(0.83333333333333333331e-3) * t2986 * t21419 - F::new(0.83333333333333333331e-3) * t2986 * t21422 - F::new(0.55555555555555555554e-3) * t17764 + F::new(0.27777777777777777777e-3) * t17770 - F::new(0.83333333333333333331e-3) * t17850 + t10339 - F::new(0.18518518518518518518e-3) * t13896;
    t21429
}
