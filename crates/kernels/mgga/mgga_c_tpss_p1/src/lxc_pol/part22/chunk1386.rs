//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1386/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1386<F: Float>(t10456: F, t10464: F, t1273: F, t13133: F, t13235: F, t1760: F, t1800: F, t18544: F, t19005: F, t19577: F, t19579: F, t20218: F, t20224: F, t20289: F, t20294: F, t20322: F, t20343: F, t20357: F, t20396: F, t2056: F, t2106: F, t3499: F, t3502: F, t3538: F, t41905: F, t42719: F, t5706: F, t5757: F, t5801: F, t5809: F, t5939: F, t6243: F, t6318: F, t6328: F, t6439: F, t65052: F, t7798: F) -> F {
    let t67674 = -F::new(2.0) * t7798 * t6318 - F::new(4.0) * t10456 * t6318 - F::new(4.0) * t2056 * t20396 - F::new(4.0) * t20294 * t3538 - F::new(2.0) * t5801 * t10464 - t18544 * t6439 - F::new(2.0) * t1760 * t20218 * t5757 + F::new(2.0) * t19579 * t20357 * t65052 - F::new(2.0) * t5706 * t20224 + F::new(2.0) * t20322 * t1273 - F::new(4.0) * t20294 * t3502 - F::new(2.0) * t20289 * t2106 - F::new(2.0) * t41905 * t1800 - F::new(4.0) * t42719 * t1800 - F::new(4.0) * t13133 * t5809 - F::new(2.0) * t13235 * t6328 - F::new(4.0) * t3499 * t20343 - t6243 * t19005 - F::new(2.0) * t19577 * t5939;
    t67674
}
