//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 866/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk866<F: Float>(t509: F, t5753: F, t1270: F, t1760: F, t1268: F, t3205: F, t1778: F, t1163: F, t118: F, t1273: F, t1684: F, t1753: F, t1757: F, t485: F, t544: F, t5512: F, t5514: F, t5519: F, t5521: F, t5524: F, t5534: F, t5536: F, t5692: F, t5702: F, t5707: F, t5712: F, t624: F, t626: F, t646: F) -> (F, F, F, F, F) {
    let t5754 = t509 * t5753;
    let t5755 = t5754 * t1270;
    let t5756 = t1760 * t5755;
    let t5757 = t3205 * t1268;
    let t5758 = t1778 * t5757;
    let t5759 = t1760 * t5758;
    let t5760 = -t1163 * t1684 - t118 * t5692 + t1273 * t1757 - t1753 * t624 - t485 * t5512 + t544 * t5702 - 2.0 * t5514 * t646 - 2.0 * t5536 * t626 - t5519 - t5521 - t5524 - t5534 + t5707 + t5712 + t5756 - t5759;
    (t5754, t5755, t5757, t5758, t5760)
}
