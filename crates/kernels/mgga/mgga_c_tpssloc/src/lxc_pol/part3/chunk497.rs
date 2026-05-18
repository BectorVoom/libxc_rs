//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 497/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk497<F: Float>(t1814: F, t554: F, t1274: F, t1276: F, t1288: F, t1293: F, t1296: F, t1789: F, t1791: F, t225: F, t680: F, t705: F) -> (F, F) {
    let t1815 = t1814 * t554;
    let t1819 = (t680 + t705 - t1274 - t1276 + t1789 + t1288 + t1791 - t1293 - t1296) * t225;
    (t1815, t1819)
}
