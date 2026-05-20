//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1982/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1982<F: Float>(t3701: F, t6995: F, t7752: F, t1390: F, t22811: F, t2233: F, t2239: F, t601: F, t9238: F, t85: F, t24: F, t12019: F, t566: F) -> (F, F, F, F, F, F, F, F) {
    let t31035 = t3701 * t6995;
    let t33136 = t3701 * t7752;
    let t34475 = t6995 * t1390;
    let t39041 = F::new(1.0) / t22811;
    let t39049 = t2233 * t2239;
    let t39054 = t601 * t9238;
    let t39061 = t85 * t85;
    let t39063 = t24 / t39061;
    let t40590 = F::new(1.0) / t12019 / t566;
    (t31035, t33136, t34475, t39041, t39049, t39054, t39063, t40590)
}
