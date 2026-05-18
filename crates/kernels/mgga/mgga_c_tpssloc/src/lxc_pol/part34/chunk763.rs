//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 763/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk763<F: Float>(t535: F, t795: F, t9580: F, t3749: F, t9577: F, t1314: F, t2566: F, t3732: F, t792: F, t782: F, t1365: F, t154: F) -> (F, F, F, F, F, F) {
    let t12194 = F::new(0.16435185185185185185e-1) * t9580 * t535 * t795;
    let t12196 = F::new(0.99999999999999999997e-2) * t9577 * t3749;
    let t12199 = t2566 * t1314;
    let t12202 = t792 * t3732;
    let t12211 = t782 * t3732;
    let t12214 = t154 * t1365;
    (t12194, t12196, t12199, t12202, t12211, t12214)
}
