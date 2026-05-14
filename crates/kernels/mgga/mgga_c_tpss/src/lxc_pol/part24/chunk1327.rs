//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1327/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1327<F: Float>(t21440: F, t2814: F, t1485: F, t15202: F, t18196: F, t18200: F, t198: F, t19960: F, t19965: F, t330: F, t4019: F, t4023: F, t5039: F, t5043: F, t5652: F, t61588: F, t61595: F, t64731: F, t64735: F, t70486: F, t70541: F, t70597: F, t70651: F, t993: F, t995: F) -> (F,) {
    let t70657 = t21440 * t2814;
    let t70688 = t198 * t330 * (t70486 + t70541 + t70597 + t70651) * t995 - t4023 * t70657 * t993 - 2.0 * t4023 * t64731 * t1485 + 4.0 * t4023 * t64735 * t19965 - 2.0 * t4023 * t19960 * t4019 + 2.0 * t4023 * t61588 * t5043 - 6.0 * t4023 * t61595 * t5043 * t993 + 4.0 * t4023 * t18200 * t1485 * t4019 - t4023 * t18196 * t5039 + 2.0 * t4023 * t18200 * t5039 * t993 - t4023 * t5652 * t15202;
    (t70688,)
}
