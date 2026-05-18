//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1387/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1387<F: Float>(t1163: F, t13131: F, t13133: F, t13136: F, t13225: F, t1339: F, t1830: F, t1834: F, t18544: F, t18547: F, t18690: F, t18711: F, t18717: F, t18898: F, t18930: F, t19579: F, t20137: F, t20319: F, t20357: F, t20374: F, t2056: F, t20642: F, t3493: F, t3499: F, t3538: F, t41839: F, t43998: F, t4541: F, t485: F, t5706: F, t5801: F, t5820: F, t5905: F, t6243: F, t626: F, t6437: F, t67538: F, t67552: F) -> F {
    let t67715 = F::new(4.0) * t19579 * t20357 * t43998 - F::new(4.0) * t2056 * t20374 - F::new(4.0) * t3499 * t20374 - F::new(4.0) * t626 * t1163 * t20319 - F::new(4.0) * t18898 * t3538 - F::new(2.0) * t67552 * t1339 + F::new(6.0) * t6243 * t18711 + F::new(3.0) * t6243 * t18717 + F::new(6.0) * t5706 * t20137 + F::new(2.0) * t5905 * t4541 + t1834 * t13131 - F::new(2.0) * t13136 * t1830 - F::new(2.0) * t626 * t485 * t67538 - F::new(4.0) * t13133 * t5820 - F::new(4.0) * t3493 * t18930 + t18544 * t6437 - F::new(3.0) * t18547 * t18690 * t41839 - F::new(2.0) * t5706 * t20642 - F::new(4.0) * t5801 * t13225;
    t67715
}
