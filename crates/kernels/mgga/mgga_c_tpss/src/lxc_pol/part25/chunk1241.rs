//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1241/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1241<F: Float>(t20690: F, t645: F, t117: F, t20319: F, t1279: F, t1281: F, t1668: F, t1670: F, t1851: F, t1853: F, t20660: F, t20679: F, t20682: F, t20685: F, t4549: F, t4556: F, t4559: F, t547: F, t548: F, t5947: F, t5954: F, t5957: F, t6446: F, t6452: F, t6455: F) -> (F, F, F) {
    let t20691 = t20690 * t645;
    let t20694 = t117 * t20319;
    let t20697 = F::new(6.0) * t1279 * t6452 + F::new(3.0) * t1279 * t6455 + F::new(3.0) * t1281 * t6446 + F::new(6.0) * t1668 * t5954 + F::new(3.0) * t1668 * t5957 + F::new(3.0) * t1670 * t5947 + F::new(6.0) * t1851 * t4556 + F::new(3.0) * t1851 * t4559 + F::new(3.0) * t1853 * t4549 + t20660 * t548 + F::new(6.0) * t20679 * t547 + F::new(6.0) * t20682 * t547 + F::new(6.0) * t20685 * t547 + F::new(6.0) * t20691 * t547 + F::new(3.0) * t20694 * t547;
    (t20691, t20694, t20697)
}
