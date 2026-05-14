//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1333/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1333<F: Float>(t1214: F, t1227: F, t1230: F, t1737: F, t19033: F, t19051: F, t19083: F, t22214: F, t22284: F, t248: F, t3515: F, t3585: F, t44836: F, t475: F, t5024: F, t52766: F, t6203: F, t6207: F, t6227: F, t6232: F, t65963: F, t65966: F, t72363: F, t72936: F, t72959: F, t77973: F, t77977: F, t78757: F, t79018: F) -> (F,) {
    let t79251 = -t72936 / 288.0 + t52766 * t22284 / 384.0 - t1227 * t248 * t1230 * t77977 / 192.0 + t72363 * t1737 / 768.0 - t44836 * t248 * t1214 * t79018 * t475 / 3072.0 + t19083 * t6207 / 72.0 + t5024 * t22214 / 216.0 + t65963 * t6227 / 256.0 - t65966 * t6232 / 512.0 + 5.0 / 2304.0 * t19051 * t6203 + 5.0 / 384.0 * t1227 * t248 * t3585 * t77973 - t3515 * t248 * t1214 * t78757 * t475 / 1024.0 + 95.0 / 1296.0 * t19033 * t6203 - t72959 / 576.0;
    (t79251,)
}
