//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 982/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk982<F: Float>(t13546: F, t485: F, t1163: F, t1322: F, t13452: F, t13458: F, t13463: F, t13470: F, t13473: F, t13478: F, t1600: F, t2056: F, t3491: F, t3499: F, t4341: F, t4631: F, t4638: F, t4641: F, t4675: F, t5314: F, t624: F, t626: F) -> (F, F) {
    let t13547 = t485 * t13546;
    let t13551 = -t1163 * t4631 - F::new(2.0) * t1163 * t4638 - F::new(2.0) * t1322 * t4341 - t13452 * t485 - F::new(2.0) * t13458 * t485 - F::new(2.0) * t13463 * t626 - F::new(2.0) * t13470 * t626 - F::new(4.0) * t13473 * t626 - F::new(4.0) * t13478 * t626 - F::new(2.0) * t13547 * t626 - F::new(2.0) * t1600 * t3491 - F::new(2.0) * t2056 * t4675 - F::new(4.0) * t3499 * t4641 - F::new(2.0) * t3499 * t4675 - t5314 * t624;
    (t13547, t13551)
}
