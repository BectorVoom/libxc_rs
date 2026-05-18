//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 316/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk316<F: Float>(t1339: F, t240: F, t1336: F, t241: F, t557: F, t67: F, t1274: F, t1276: F, t1286: F, t1288: F, t1290: F, t1293: F, t1296: F, t225: F, t680: F, t705: F) -> (F, F, F, F, F) {
    let t1340 = t1339 * t240;
    let t1341 = t1336 * t1340;
    let t1342 = t241 * t557;
    let t1343 = t1342 * t67;
    let t1345 = (t680 + t705 + t1274 - t1276 + t1286 + t1288 + t1290 - t1293 - t1296) * t225;
    (t1340, t1341, t1342, t1343, t1345)
}
