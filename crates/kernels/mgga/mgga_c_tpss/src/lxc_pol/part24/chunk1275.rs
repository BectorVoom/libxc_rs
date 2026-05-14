//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1275/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1275<F: Float>(t21108: F, t5706: F, t19632: F, t6243: F, t1268: F, t21011: F, t18547: F, t19580: F, t1273: F, t13470: F, t1600: F, t17916: F, t19431: F, t19457: F, t19462: F, t2056: F, t21208: F, t21234: F, t3499: F, t3538: F, t3542: F, t4341: F, t4674: F, t4675: F, t5514: F, t5692: F, t6096: F, t626: F, t68961: F, t68969: F, t68973: F, t68976: F, t68977: F) -> (F,) {
    let t68980 = t5706 * t21108;
    let t68988 = 6.0 * t6243 * t19632;
    let t68989 = t21011 * t1268;
    let t68992 = 12.0 * t18547 * t19580 * t68989;
    let t69002 = -2.0 * t4674 * t5692 * t626 + t1273 * t21234 - 2.0 * t13470 * t5514 - 2.0 * t1600 * t19431 - 2.0 * t17916 * t4675 - 2.0 * t19457 * t4675 - 4.0 * t19462 * t3538 - 4.0 * t19462 * t3542 - 2.0 * t2056 * t21208 - 2.0 * t21208 * t3499 - 2.0 * t4341 * t6096 + t68961 + t68969 + t68973 + t68976 + t68977 + t68980 + t68988 + t68992;
    (t69002,)
}
