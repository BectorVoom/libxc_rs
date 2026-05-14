//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1353/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1353<F: Float>(t1270: F, t1760: F, t509: F, t65710: F, t65778: F, t65843: F, t65892: F, t1777: F, t30367: F, t1659: F, t3202: F, t19579: F, t18409: F, t3493: F, t19597: F, t2056: F) -> (F, F, F, F) {
    let t65897 = t1760 * t509 * (t65710 + t65778 + t65843 + t65892) * t1270;
    let t65898 = t1777 * t30367;
    let t65899 = t1659 * t3202;
    let t65902 = 6.0 * t19579 * t65898 * t65899;
    let t65904 = 4.0 * t3493 * t18409;
    let t65906 = 4.0 * t2056 * t19597;
    (t65897, t65902, t65904, t65906)
}
