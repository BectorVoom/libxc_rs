//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 465/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk465<F: Float>(t1172: F, t1175: F, t1187: F, t1192: F, t1195: F, t1196: F, t1270: F, t1615: F, t1617: F, t1625: F, t1659: F, t198: F, t509: F, t654: F, t679: F) -> (F,) {
    let t1663 = t1270 * t1659 * t198 * t509 + 3.0 * t1196 * t1625 * t198 - t1172 - t1175 + t1187 - t1192 - t1195 + t1615 + t1617 + t654 + t679;
    (t1663,)
}
