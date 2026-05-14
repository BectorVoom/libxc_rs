//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 621/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk621<F: Float>(t6177: F, t974: F, t1196: F, t5398: F, t3555: F, t5392: F, t1653: F, t1735: F, t3578: F, t1174: F, t1726: F, t1737: F, t3577: F, t488: F, t4889: F, t4957: F, t4959: F, t4994: F, t4998: F, t5002: F, t6158: F, t6165: F, t6170: F) -> (F, F, F, F, F, F, F, F) {
    let t6178 = t974 * t6177;
    let t6183 = t1196 * t5398;
    let t6184 = t974 * t6183;
    let t6187 = t3555 * t5392;
    let t6188 = t974 * t6187;
    let t6191 = t1735 * t1653;
    let t6192 = t3578 * t6191;
    let t6197 = -t6158 * t488 / 288.0 + 19.0 / 1728.0 * t6165 * t488 + t6170 * t488 / 3072.0 + t4957 / 2304.0 - t4959 / 432.0 - t4994 / 3456.0 + t4998 / 2304.0 + t1174 * t6178 / 216.0 + t4889 * t1726 / 54.0 - t1174 * t6184 / 288.0 - t1174 * t6188 / 144.0 - t3577 * t6192 / 2304.0 + t5002 * t1737 / 1536.0;
    (t6178, t6183, t6184, t6187, t6188, t6191, t6192, t6197)
}
