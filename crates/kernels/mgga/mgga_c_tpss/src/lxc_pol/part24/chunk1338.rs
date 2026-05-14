//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1338/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1338<F: Float>(t19610: F, t65533: F, t13965: F, t18547: F, t24790: F, t118: F, t1322: F, t1339: F, t13452: F, t13974: F, t1753: F, t1757: F, t19462: F, t20078: F, t3491: F, t3502: F, t4541: F, t4631: F, t5692: F, t6228: F, t6239: F, t65490: F, t69037: F, t70797: F, t70978: F, t70986: F, t70989: F, t70991: F, t70994: F, t70999: F, t71002: F, t71010: F) -> (F,) {
    let t71012 = 6.0 * t65533 * t19610;
    let t71017 = 6.0 * t18547 * t24790 * t13965;
    let t71019 = -t118 * (t70797 + t70978) - 2.0 * t3491 * t6228 - 2.0 * t1322 * t20078 + t70986 - t70989 - t70991 - t70994 - t13452 * t1753 - t4631 * t5692 + t70999 - t71002 - 4.0 * t65490 * t1339 - 4.0 * t69037 * t1339 - 4.0 * t19462 * t3502 - t71010 - t71012 + 2.0 * t6239 * t4541 - t71017 + t1757 * t13974;
    (t71019,)
}
