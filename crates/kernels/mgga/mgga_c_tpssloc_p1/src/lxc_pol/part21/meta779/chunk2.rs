//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2704/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2704<F: Float>(t12189: F, t6375: F, t40138: F, t6396: F, t12283: F, t19951: F, t19991: F, t40281: F, t12407: F, t12429: F, t16224: F, t16225: F, t16305: F, t16306: F, t16311: F, t16366: F, t16370: F, t16394: F, t19871: F, t19921: F, t19926: F, t19976: F, t19981: F, t19989: F, t3783: F, t3803: F, t3805: F, t5246: F, t5303: F, t53973: F, t54013: F, t54162: F, t54202: F) -> F {
    let t56953 = t12189 * t6375;
    let t56959 = t40138 * t6396;
    let t56961 = t12283 * t19951;
    let t56963 = t12283 * t19991;
    let t56993 = t40281 * t6396;
    let t56996 = -F::new(35.0) / F::new(216.0) * t56953 - F::new(5.0) / F::new(192.0) * t3803 * t16224 * t16225 * t19989 - F::new(7.0) / F::new(288.0) * t56959 - F::new(7.0) / F::new(288.0) * t56961 - F::new(7.0) / F::new(288.0) * t56963 + t3803 * t3805 * t19871 * t12407 / F::new(768.0) + t54162 * t5303 / F::new(192.0) + t16394 * t16366 / F::new(192.0) + t16394 * t16370 / F::new(384.0) + t3803 * t16305 * t16306 * t19989 / F::new(192.0) + t5246 * t54013 * t16311 * t53973 / F::new(128.0) - F::new(5.0) / F::new(64.0) * t3783 * t19921 + F::new(5.0) / F::new(192.0) * t3783 * t19926 - t12429 * t19976 / F::new(1536.0) + t12429 * t19951 / F::new(192.0) - F::new(5.0) / F::new(384.0) * t12429 * t19981 + F::new(119.0) / F::new(1728.0) * t56993 + F::new(7.0) / F::new(144.0) * t54202;
    t56996
}
