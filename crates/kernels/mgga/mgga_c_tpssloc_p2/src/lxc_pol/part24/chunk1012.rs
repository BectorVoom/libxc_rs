//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1012/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1012<F: Float>(t11496: F, t457: F, t460: F, t974: F, t1184: F, t3475: F, t3469: F, t4934: F, t135: F, t3477: F, t1174: F, t11153: F, t461: F) -> (F, F, F, F, F) {
    let t11498 = t457 * t11496 * t460;
    let t11499 = t974 * t11498;
    let t11502 = t3475 * t1184;
    let t11504 = t457 * t11502 * t460;
    let t11505 = t974 * t11504;
    let t11509 = t3469 * t1184 * t460;
    let t11510 = t4934 * t11509;
    let t11513 = t135 * t3477;
    let t11514 = t1174 * t11513;
    let t11516 = t461 * t11153;
    (t11499, t11505, t11510, t11514, t11516)
}
