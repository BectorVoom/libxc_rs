//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1225/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1225<F: Float>(t21106: F, t509: F, t1270: F, t1760: F, t13955: F, t1778: F, t19631: F, t6245: F, t4570: F, t84: F, t77: F, t1290: F, t3418: F, t1313: F, t1317: F, t4626: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t21107 = t509 * t21106;
    let t21108 = t21107 * t1270;
    let t21109 = t1760 * t21108;
    let t21110 = t1778 * t13955;
    let t21111 = t1760 * t21110;
    let t21112 = t19631 * t6245;
    let t21114 = 6.0 * t1760 * t21112;
    let t21115 = t84 * t4570;
    let t21116 = t77 * t21115;
    let t21123 = t3418 * t1290;
    let t21128 = t1313 * t1317;
    let t21129 = t77 * t21128;
    let t21132 = t84 * t4626;
    (t21107, t21108, t21109, t21110, t21111, t21112, t21114, t21115, t21116, t21123, t21128, t21129, t21132)
}
