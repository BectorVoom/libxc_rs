//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 875/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk875<F: Float>(t10021: F, t498: F, t1193: F, t8038: F, t2206: F, t3178: F, t1184: F, t3214: F, t3305: F, t8027: F, t1170: F, t3280: F, t8017: F, t1220: F, t2376: F, t339: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10022 = t10021 * t498;
    let t10028 = 0.10254018858216406658e4 * t1193 * t8038;
    let t10029 = t3178 * t2206;
    let t10031 = t3214 * t1184;
    let t10033 = t3305 * t1184;
    let t10038 = 0.35089341735807877242e1 * t1193 * t8027;
    let t10039 = t1170 * t3280;
    let t10042 = 0.5848223622634646207e0 * t1193 * t8017;
    let t10077 = t339 * t1220 * t2376;
    (t10022, t10028, t10029, t10031, t10033, t10038, t10039, t10042, t10077)
}
