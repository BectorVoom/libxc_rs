//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 997/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk997<F: Float>(t3205: F, t5451: F, t1270: F, t1268: F, t1625: F, t10038: F, t10042: F, t1206: F, t12913: F, t12915: F, t12918: F, t12924: F, t13812: F, t13813: F, t13814: F, t13815: F, t13816: F, t13817: F, t3183: F, t3184: F, t4519: F, t4524: F, t4525: F, t5366: F, t7979: F, t7988: F, t7992: F) -> (F, F, F) {
    let t13955 = t5451 * t3205;
    let t13958 = t5451 * t1270;
    let t13965 = t1625 * t1268;
    let t13972 = 3.0 * t1206 * t13958 * t3183 - t1268 * t13955 * t4524 - 6.0 * t13965 * t3183 * t4525 + 3.0 * t3183 * t3184 * t5366 - 2.0 * t4519 * t4524 * t4525 - t10038 - t10042 - t12913 - t12915 + t12918 - t12924 + t13812 - t13813 + t13814 + t13815 - t13816 - t13817 + t7979 + t7988 + t7992;
    (t13955, t13965, t13972)
}
