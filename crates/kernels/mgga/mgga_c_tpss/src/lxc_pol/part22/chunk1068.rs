//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1068/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1068<F: Float>(t1659: F, t9895: F, t3205: F, t4519: F, t10038: F, t10042: F, t10232: F, t1268: F, t1270: F, t12907: F, t12909: F, t12911: F, t12913: F, t12915: F, t12918: F, t12919: F, t12921: F, t12923: F, t12924: F, t13111: F, t1625: F, t198: F, t3183: F, t3184: F, t3202: F, t4397: F, t4524: F, t509: F, t7979: F, t7988: F, t7992: F) -> (F, F) {
    let t13115 = t1659 * t9895;
    let t13119 = t4519 * t3205;
    let t13129 = t1270 * t13111 * t198 * t509 + 3.0 * t10232 * t1625 * t3183 - 2.0 * t1268 * t13119 * t4524 + 2.0 * t13115 * t3202 * t4524 + 6.0 * t3183 * t3184 * t4397 - t10038 - t10042 - t12907 + t12909 - t12911 + t12913 - t12915 + t12918 + t12919 - t12921 - t12923 + t12924 + t7979 + t7988 + t7992;
    (t13119, t13129)
}
