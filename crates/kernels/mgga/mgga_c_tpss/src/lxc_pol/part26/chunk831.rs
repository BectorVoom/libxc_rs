//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 831/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk831<F: Float>(t509: F, t5753: F, t1270: F, t1760: F, t1268: F, t3205: F, t1778: F, t1279: F, t1786: F, t116: F, t1688: F) -> (F, F, F, F, F, F, F, F) {
    let t5754 = t509 * t5753;
    let t5755 = t5754 * t1270;
    let t5756 = t1760 * t5755;
    let t5757 = t3205 * t1268;
    let t5758 = t1778 * t5757;
    let t5759 = t1760 * t5758;
    let t5771 = 3.0 * t1279 * t1786;
    let t5772 = t116 * t1688;
    (t5754, t5755, t5756, t5757, t5758, t5759, t5771, t5772)
}
