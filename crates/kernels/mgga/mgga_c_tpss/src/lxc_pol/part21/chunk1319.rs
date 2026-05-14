//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1319/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1319<F: Float>(t1659: F, t3387: F, t19579: F, t19580: F, t19619: F, t5705: F, t19621: F, t18534: F, t6243: F, t3234: F, t18547: F, t7029: F, t44070: F, t13220: F, t94: F, t1689: F) -> (F, F, F, F, F, F) {
    let t65052 = t1659 * t3387;
    let t65055 = 2.0 * t19579 * t19580 * t65052;
    let t65056 = t5705 * t19619;
    let t65058 = 12.0 * t65056 * t19621;
    let t65059 = t6243 * t18534;
    let t65060 = t1659 * t3234;
    let t65063 = 3.0 * t18547 * t7029 * t65060;
    let t65066 = 6.0 * t18547 * t7029 * t44070;
    let t65067 = t94 * t13220;
    let t65069 = 2.0 * t65067 * t1689;
    (t65055, t65058, t65059, t65063, t65066, t65069)
}
