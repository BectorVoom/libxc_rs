//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1163/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1163<F: Float>(t12012: F, t550: F, t120: F, t12177: F, t12371: F, t16398: F, t12283: F, t12426: F, t12250: F, t1307: F, t3850: F, t12291: F, t12368: F, t12397: F, t12419: F, t12420: F, t1341: F, t1343: F, t1352: F, t16233: F, t16305: F, t3790: F, t3803: F, t3805: F, t3806: F, t3807: F, t3853: F, t40148: F, t40153: F, t40160: F, t40162: F, t40168: F, t40169: F, t820: F) -> (F, F, F) {
    let t40178 = t550 * t12012;
    let t40183 = t120 * t12177;
    let t40188 = t16398 * t12371;
    let t40190 = t12283 * t12426;
    let t40192 = t12250 * t1307;
    let t40197 = t1307 * t3850;
    let t40204 = -F::new(3.0) / F::new(256.0) * t12291 * t1343 * t820 * t40148 - t1341 * t1343 * t820 * t40153 / F::new(3072.0) + F::new(119.0) / F::new(1152.0) * t40160 + F::new(7.0) / F::new(1536.0) * t3790 * t1343 * t820 * t40162 + F::new(5.0) / F::new(32.0) * t3803 * t40168 * t3806 * t40169 - F::new(5.0) / F::new(128.0) * t3803 * t12419 * t12368 * t12420 + t3803 * t3805 * t3806 * t40178 / F::new(192.0) + t3803 * t3805 * t40183 * t3807 / F::new(192.0) + F::new(7.0) / F::new(48.0) * t40188 - F::new(7.0) / F::new(96.0) * t40190 + t16233 * t3805 * t40183 * t40192 / F::new(32.0) + t3803 * t16305 * t1352 * t40197 / F::new(64.0) - t12397 * t3853 / F::new(512.0);
    (t40183, t40197, t40204)
}
