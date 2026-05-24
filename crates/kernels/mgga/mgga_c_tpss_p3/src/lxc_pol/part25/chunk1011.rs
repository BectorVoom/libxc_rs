//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1011/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1011<F: Float>(t13614: F, t13625: F, t13954: F, t13972: F, t1165: F, t13133: F, t1338: F, t13452: F, t13458: F, t13546: F, t13554: F, t13565: F, t2056: F, t3493: F, t3537: F, t4347: F, t4674: F, t6234: F, t645: F) -> (F, F) {
    let t13974 = t13614 + t13625 + t13954 + t13972;
    let t14001 = F::new(2.0) * t1165 * t13546 + F::new(4.0) * t13133 * t1338 + F::new(4.0) * t1338 * t13554 + F::new(2.0) * t13565 * t645 + F::new(2.0) * t2056 * t4674 + F::new(4.0) * t3493 * t3537 + F::new(4.0) * t3537 * t6234 + F::new(2.0) * t4347 * t4674 + t13452 + F::new(2.0) * t13458;
    (t13974, t14001)
}
