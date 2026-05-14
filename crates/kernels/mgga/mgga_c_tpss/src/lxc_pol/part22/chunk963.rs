//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 963/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk963<F: Float>(t11207: F, t895: F, t904: F, t912: F, t1448: F, t8772: F, t2622: F, t1411: F, t2480: F, t2483: F, t10954: F, t10956: F, t10957: F, t10963: F, t10965: F, t10968: F, t10970: F, t10972: F, t11103: F, t11123: F, t11124: F, t11146: F, t11149: F, t11155: F, t11160: F, t2807: F, t2811: F, t4023: F, t4024: F, t993: F) -> (F, F, F, F) {
    let t11209 = t895 * t11207 * t904;
    let t11211 = 0.5848223622634646207e0 * t912 * t11209;
    let t11212 = t8772 * t1448;
    let t11213 = t11212 * t2622;
    let t11215 = 0.10389515463408878255e3 * t912 * t11213;
    let t11216 = t1411 * t2480;
    let t11218 = 2.0 * t11216 * t2483;
    let t11219 = 2.0 * t10957 * t2811 * t4023 - 2.0 * t11124 * t4023 * t993 - t2807 * t4023 * t4024 + t10954 - t10956 + t10963 + t10965 + t10968 + t10970 + t10972 + t11103 + t11123 - t11146 - t11149 + t11155 - t11160 - t11211 + t11215 - t11218;
    (t11211, t11215, t11218, t11219)
}
