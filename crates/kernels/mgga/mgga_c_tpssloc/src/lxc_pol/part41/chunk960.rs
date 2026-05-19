//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 960/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk960<F: Float>(t3749: F, t9577: F, t1314: F, t2566: F, t3741: F, t3732: F, t792: F, t782: F, t1365: F, t154: F, t205: F, t116: F, t547: F) -> (F, F, F, F, F, F, F, F) {
    let t12196 = F::cast_from(0.99999999999999999997e-2_f64) * t9577 * t3749;
    let t12199 = t2566 * t1314;
    let t12200 = t12199 * t3741;
    let t12202 = t792 * t3732;
    let t12211 = t782 * t3732;
    let t12214 = t154 * t1365;
    let t12215 = t205 * t12214;
    let t12225 = t547 * t116;
    (t12196, t12199, t12200, t12202, t12211, t12214, t12215, t12225)
}
