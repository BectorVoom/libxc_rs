//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1376/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1376<F: Float>(t1206: F, t1844: F, t6435: F, t9895: F, t1163: F, t13452: F, t13458: F, t13470: F, t13565: F, t13965: F, t1600: F, t1760: F, t1830: F, t18547: F, t18690: F, t18898: F, t19577: F, t19579: F, t19581: F, t20288: F, t20294: F, t20357: F, t21017: F, t21863: F, t21922: F, t25469: F, t4341: F, t4631: F, t4638: F, t4675: F, t485: F, t51635: F, t5706: F, t5801: F, t5820: F, t5895: F, t6309: F, t6439: F, t68989: F, t71374: F) -> F {
    let t72608 = t1206 * t1844;
    let t72633 = t6435 * t9895;
    let t72637 = -F::new(2.0) * t13565 * t5820 - F::new(2.0) * t20288 * t1600 - F::new(2.0) * t6309 * t4341 + F::new(12.0) * t18547 * t20357 * t68989 - F::new(3.0) * t18547 * t18690 * t51635 + F::new(6.0) * t5706 * t21863 + F::new(6.0) * t1760 * t72608 * t21017 - F::new(2.0) * t18898 * t4675 - F::new(2.0) * t20294 * t4675 - F::new(2.0) * t5801 * t13470 - F::new(6.0) * t18547 * t25469 * t13965 - F::new(2.0) * t19577 * t6439 - F::new(2.0) * t71374 * t485 - F::new(2.0) * t21922 * t1163 - t13452 * t1830 - t4631 * t5895 - F::new(2.0) * t13458 * t1830 - F::new(2.0) * t4638 * t5895 + F::new(4.0) * t19579 * t72633 * t19581;
    t72637
}
