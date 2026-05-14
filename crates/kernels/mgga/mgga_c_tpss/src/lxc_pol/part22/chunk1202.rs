//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1202/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1202<F: Float>(t10897: F, t33: F, t1659: F, t3387: F, t19619: F, t5705: F, t3234: F, t13220: F, t94: F, t1206: F, t19581: F, t1338: F, t2053: F, t3537: F, t623: F, t2049: F, t6076: F, t77: F) -> (F, F, F, F, F, F, F, F, F) {
    let t65030 = t33 * t10897;
    let t65052 = t1659 * t3387;
    let t65056 = t5705 * t19619;
    let t65060 = t1659 * t3234;
    let t65067 = t94 * t13220;
    let t65085 = t19581 * t1206;
    let t65094 = t2053 * t1338;
    let t65097 = t623 * t3537;
    let t65152 = t77 * t6076 * t2049;
    (t65030, t65052, t65056, t65060, t65067, t65085, t65094, t65097, t65152)
}
