//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 840/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk840<F: Float>(t1314: F, t2559: F, t1317: F, t535: F, t795: F, t9580: F, t3749: F, t9577: F, t2566: F, t3741: F, t3732: F, t792: F, t782: F, t1365: F, t154: F, t205: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12189 = t2559 * t1314;
    let t12190 = t12189 * t1317;
    let t12194 = 0.16435185185185185185e-1 * t9580 * t535 * t795;
    let t12196 = 0.99999999999999999997e-2 * t9577 * t3749;
    let t12199 = t2566 * t1314;
    let t12200 = t12199 * t3741;
    let t12202 = t792 * t3732;
    let t12211 = t782 * t3732;
    let t12214 = t154 * t1365;
    let t12215 = t205 * t12214;
    (t12189, t12190, t12194, t12196, t12199, t12200, t12202, t12211, t12214, t12215)
}
