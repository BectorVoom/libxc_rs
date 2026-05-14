//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 781/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk781<F: Float>(t1390: F, t2018: F, t584: F, t16: F, t2: F, t591: F, t9: F, t21: F, t587: F, t14: F, t598: F, t2230: F, t594: F, t2229: F, t3: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8945 = t2018 * t1390;
    let t9211 = 0.1044e2 * t584;
    let t9212 = t2 * t16;
    let t9213 = 0.4332e2 * t9212;
    let t9214 = t9 * t591;
    let t9215 = 0.9288e2 * t9214;
    let t9216 = t587 * t21;
    let t9217 = 0.3912e3 * t9216;
    let t9218 = t14 * t598;
    let t9219 = 0.12804e4 * t9218;
    let t9220 = t594 * t2230;
    let t9221 = 0.170856e4 * t9220;
    let t9222 = t2229 * t3;
    let t9223 = 1.0 / t9222;
    (t8945, t9211, t9212, t9213, t9214, t9215, t9216, t9217, t9218, t9219, t9221, t9222, t9223)
}
