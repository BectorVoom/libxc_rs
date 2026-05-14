//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1262/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1262<F: Float>(t116: F, t20319: F, t117: F, t1279: F, t1281: F, t13220: F, t13265: F, t1668: F, t1853: F, t19041: F, t19044: F, t19047: F, t2061: F, t20660: F, t20682: F, t20685: F, t20690: F, t20691: F, t2105: F, t3403: F, t3410: F, t4549: F, t4556: F, t547: F, t5947: F, t5953: F, t5954: F, t5957: F, t6323: F, t6446: F, t645: F, t6452: F, t67538: F) -> (F,) {
    let t67816 = t116 * t20319;
    let t67843 = 3.0 * t547 * t117 * t67538 + 12.0 * t1279 * t20682 + 6.0 * t4549 * t5957 + 3.0 * t6446 * t3410 + 6.0 * t3403 * t6452 + 12.0 * t1279 * t20685 + 12.0 * t1668 * t19041 + 12.0 * t547 * t67816 * t645 + 12.0 * t4549 * t5954 + 6.0 * t20660 * t1281 + 3.0 * t13265 * t1853 + 12.0 * t5947 * t4556 + 6.0 * t547 * t5953 * t13220 + 6.0 * t1668 * t19044 + 3.0 * t1668 * t19047 + 6.0 * t547 * t2061 * t6323 + 6.0 * t547 * t20690 * t2105 + 12.0 * t1279 * t20691;
    (t67843,)
}
