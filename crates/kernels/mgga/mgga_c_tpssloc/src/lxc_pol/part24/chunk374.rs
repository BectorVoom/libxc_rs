//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 374/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk374<F: Float>(t1274: F, t1276: F, t1286: F, t1288: F, t1290: F, t1293: F, t1296: F, t225: F, t680: F, t705: F, t557: F, t68: F, t1307: F, t546: F, t548: F) -> (F, F, F, F) {
    let t1345 = (t680 + t705 + t1274 - t1276 + t1286 + t1288 + t1290 - t1293 - t1296) * t225;
    let t1347 = t68 * t557;
    let t1348 = t1347 * t1307;
    let t1351 = -t1345 * t548 + 3.0 * t1348 * t546;
    (t1345, t1347, t1348, t1351)
}
