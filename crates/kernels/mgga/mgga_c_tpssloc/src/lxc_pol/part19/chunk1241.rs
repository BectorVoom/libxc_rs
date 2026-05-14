//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1241/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1241<F: Float>(t10510: F, t3109: F, t1023: F, t10309: F, t10390: F, t10398: F, t10408: F, t10410: F, t10413: F, t10419: F, t10493: F, t10858: F, t10886: F, t10937: F, t2776: F, t3041: F, t3070: F, t3071: F, t3117: F, t43186: F, t43200: F, t43206: F, t43211: F, t43214: F, t43219: F, t884: F) -> (F,) {
    let t43221 = t3109 * t10510;
    let t43223 = t3070 * t3071 * t10858 * t884 / 1152.0 + t43186 / 288.0 - t10390 * t10419 / 192.0 - 5.0 / 576.0 * t3070 * t10408 * t10309 * t1023 + t10413 * t3071 * t3041 * t2776 / 384.0 - t43200 / 1728.0 - t10937 * t10398 / 72.0 - 5.0 / 216.0 * t10937 * t10410 - t43206 / 288.0 + t3117 * t10493 / 192.0 - t43211 * t10886 / 144.0 + t43214 / 324.0 + t43219 / 2592.0 + t43221 / 216.0;
    (t43223,)
}
