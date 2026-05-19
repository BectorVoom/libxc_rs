//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1344/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1344<F: Float>(t5: F, t71386: F, t71411: F, t71431: F, t71460: F, t71487: F, t71499: F, t71520: F, t71544: F, t117: F, t1338: F, t13546: F, t13565: F, t1799: F, t18898: F, t20289: F, t20294: F, t25232: F, t3537: F, t42710: F, t4674: F, t50656: F, t5801: F, t5815: F, t645: F, t67541: F, t69023: F, t71308: F, t71344: F, t71374: F) -> (F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t71548 = piecewise3::<F>(t8, F::new(0.0), t71386 + t71411 + t71431 + t71460 + t71487 + t71499 + t71520 + t71544);
    let t71549 = t71548 * t117;
    let t71574 = F::new(4.0) * t1338 * t67541 + F::new(4.0) * t1338 * t71344 + F::new(2.0) * t13546 * t5801 + F::new(2.0) * t13565 * t5815 + F::new(2.0) * t1799 * t42710 + F::new(2.0) * t1799 * t50656 + F::new(4.0) * t1799 * t69023 + F::new(2.0) * t18898 * t4674 + F::new(4.0) * t20289 * t3537 + F::new(2.0) * t20294 * t4674 + F::new(4.0) * t25232 * t3537 + F::new(2.0) * t645 * t71308 + F::new(2.0) * t71374 + t71549;
    (t71549, t71574)
}
