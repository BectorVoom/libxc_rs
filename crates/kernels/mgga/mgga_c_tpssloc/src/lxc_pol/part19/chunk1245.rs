//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1245/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1245<F: Float>(t3131: F, t3047: F, t3077: F, t10908: F, t3114: F, t1036: F, t10438: F, t221: F, t339: F, t42813: F, t10283: F, t995: F, t10931: F, t135: F, t973: F, t1021: F, t1046: F, t10501: F, t10998: F, t248: F, t2960: F, t3048: F, t350: F, t42348: F, t42759: F, t43273: F, t43277: F, t43281: F, t43285: F, t43291: F) -> (F, F) {
    let t43292 = t3131 * t3131;
    let t43298 = t3077 * t3047;
    let t43301 = t3114 * t10908;
    let t43303 = t10438 * t1036;
    let t43307 = 5.0 / 486.0 * t339 * t221 * t42813;
    let t43310 = t10283 * t995;
    let t43313 = t973 * t135 * t10931;
    let t43315 = 5.0 / 108.0 * t3048 * t10501 - 2.0 / 9.0 * t2960 * t10998 + t43273 / 36.0 + t43277 / 192.0 - t43281 / 192.0 + t43285 / 1152.0 + t43291 * t248 * t1021 * t42348 * t43292 / 128.0 - t43298 * t1046 / 72.0 + t43301 / 384.0 + 19.0 / 216.0 * t43303 - t43307 + 1309.0 / 486.0 * t42759 * t350 - 154.0 / 243.0 * t43310 - t43313 / 27.0;
    (t43292, t43315)
}
