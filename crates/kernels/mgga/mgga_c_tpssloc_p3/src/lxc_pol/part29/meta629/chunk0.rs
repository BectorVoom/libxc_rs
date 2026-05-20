//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2074/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2074<F: Float>(t2157: F, t43706: F, t24977: F, t576: F, t1395: F, t7426: F, t12521: F, t7467: F, t81440: F, t1453: F, t81439: F, t26129: F, t81442: F) -> (F, F, F, F, F, F, F) {
    let t86524 = t2157 * t43706;
    let t86557 = t576 * t24977;
    let t86559 = t1395 * t7426;
    let t86582 = F::new(0.135e2) * t12521 * t7467;
    let t86583 = F::new(22.0) / F::new(9.0) * t81440;
    let t86586 = t81439 * t1453;
    let t86588 = t81442 * t26129;
    (t86524, t86557, t86559, t86582, t86583, t86586, t86588)
}
