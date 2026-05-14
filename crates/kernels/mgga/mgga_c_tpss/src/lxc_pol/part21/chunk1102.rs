//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1102/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1102<F: Float>(t1589: F, t9519: F, t1153: F, t12159: F, t12161: F, t12164: F, t12167: F, t12170: F, t12243: F, t12246: F, t12250: F, t12253: F, t12257: F, t12260: F, t12337: F, t12340: F, t12342: F, t12344: F, t12346: F, t12350: F, t12639: F, t198: F, t3147: F, t3151: F, t330: F, t4023: F, t4329: F) -> (F,) {
    let t12645 = t1589 * t9519;
    let t12649 = t1153 * t12639 * t198 * t330 + 2.0 * t12645 * t3151 * t4023 - t3147 * t4023 * t4329 - t12159 + t12161 - t12164 - t12167 - t12170 - t12243 - t12246 + t12250 + t12253 + t12257 + t12260 + t12337 + t12340 - t12342 - t12344 - t12346 - t12350;
    (t12649,)
}
