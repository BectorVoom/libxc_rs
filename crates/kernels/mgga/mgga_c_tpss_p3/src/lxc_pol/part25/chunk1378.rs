//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1378/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1378<F: Float>(t1163: F, t13463: F, t13478: F, t13546: F, t1830: F, t18547: F, t18690: F, t18898: F, t19305: F, t19308: F, t19620: F, t20294: F, t20358: F, t20386: F, t20396: F, t20407: F, t21236: F, t21786: F, t21871: F, t4641: F, t485: F, t51631: F, t5706: F, t5801: F, t5816: F, t6103: F, t6243: F, t626: F, t6318: F, t6324: F, t68950: F, t68967: F, t71549: F) -> F {
    let t72721 = -F::cast_from(2.0_f64) * t21236 * t5816 - F::cast_from(4.0_f64) * t19305 * t6318 - F::cast_from(4.0_f64) * t19308 * t6318 - F::cast_from(4.0_f64) * t6103 * t20396 - F::cast_from(2.0_f64) * t626 * t1830 * t13546 - F::cast_from(4.0_f64) * t18898 * t4641 - F::cast_from(4.0_f64) * t20294 * t4641 - F::cast_from(4.0_f64) * t5801 * t13478 - F::cast_from(2.0_f64) * t5801 * t13463 - F::cast_from(4.0_f64) * t19305 * t6324 - F::cast_from(4.0_f64) * t19308 * t6324 - F::cast_from(4.0_f64) * t6103 * t20386 - F::cast_from(6.0_f64) * t19620 * t18690 * t51631 - t5706 * t21871 - F::cast_from(6.0_f64) * t18547 * t18690 * t68950 + F::cast_from(4.0_f64) * t68967 * t20358 + F::cast_from(6.0_f64) * t6243 * t20407 - t71549 * t485 - t21786 * t1163;
    t72721
}
