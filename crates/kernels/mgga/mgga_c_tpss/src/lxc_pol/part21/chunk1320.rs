//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1320/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1320<F: Float>(t19308: F, t5522: F, t5753: F, t9895: F, t19579: F, t19581: F, t118: F, t1322: F, t1338: F, t1339: F, t1600: F, t17916: F, t18287: F, t18375: F, t19315: F, t3499: F, t3502: F, t4341: F, t5512: F, t61897: F, t626: F, t63751: F, t63753: F, t63756: F, t64869: F, t65049: F, t65055: F, t65058: F, t65059: F, t65063: F, t65066: F, t65069: F) -> (F,) {
    let t65071 = 4.0 * t19308 * t5522;
    let t65076 = t5753 * t9895;
    let t65079 = 4.0 * t19579 * t65076 * t19581;
    let t65080 = -t63751 - t63753 - 2.0 * t61897 * t1339 - 4.0 * t63756 * t1339 - 4.0 * t17916 * t3502 - 4.0 * t3499 * t19315 - 2.0 * t626 * t18287 * t1338 - t118 * (t64869 + t65049) + t65055 + t65058 + t65059 - t65063 - t65066 - t65069 - t65071 - t1322 * t18287 - t18375 * t1600 - 2.0 * t5512 * t4341 + t65079;
    (t65080,)
}
