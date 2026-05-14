//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1256/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1256<F: Float>(t1600: F, t1760: F, t18295: F, t1845: F, t18544: F, t18628: F, t18687: F, t18694: F, t18710: F, t18898: F, t18903: F, t19604: F, t20289: F, t20358: F, t20368: F, t20407: F, t2056: F, t2065: F, t2105: F, t3499: F, t3537: F, t3542: F, t41867: F, t5706: F, t5895: F, t5909: F, t6103: F, t6243: F, t626: F, t63710: F, t6399: F, t6413: F, t6436: F, t646: F, t65501: F, t67541: F, t9909: F) -> (F,) {
    let t67633 = 3.0 * t1760 * t5909 * t65501 - 4.0 * t2056 * t20368 - 4.0 * t3499 * t20368 - 4.0 * t626 * t5895 * t3537 + 6.0 * t6243 * t18687 + 6.0 * t5706 * t20407 - t1760 * t1845 * t41867 + 3.0 * t18544 * t6413 + 6.0 * t1760 * t18710 * t19604 - t1760 * t6436 * t9909 - 4.0 * t67541 * t646 - 4.0 * t20289 * t2065 - 2.0 * t6243 * t18694 - 2.0 * t626 * t6399 * t2105 - 4.0 * t18898 * t3542 - 2.0 * t18903 * t1600 + 4.0 * t63710 * t20358 + 2.0 * t1760 * t6436 * t18295 - 2.0 * t6103 * t18628;
    (t67633,)
}
