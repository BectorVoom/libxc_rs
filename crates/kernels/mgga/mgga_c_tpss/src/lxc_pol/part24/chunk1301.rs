//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1301/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1301<F: Float>(t18547: F, t51622: F, t7029: F, t1689: F, t69069: F, t13478: F, t1663: F, t17916: F, t19457: F, t19667: F, t21532: F, t3537: F, t4641: F, t5514: F, t6228: F, t626: F, t645: F, t69427: F, t69437: F, t69439: F, t69441: F, t69444: F, t69768: F, t69770: F, t69773: F, t69775: F, t69776: F, t69779: F) -> (F,) {
    let t69782 = 3.0 * t18547 * t7029 * t51622;
    let t69784 = 2.0 * t69069 * t1689;
    let t69788 = -2.0 * t21532 * t626 * t645 - 4.0 * t3537 * t6228 * t626 - 4.0 * t13478 * t5514 + 2.0 * t1663 * t19667 - 4.0 * t17916 * t4641 - 4.0 * t19457 * t4641 + t69427 - t69437 - t69439 - t69441 - t69444 + t69768 - t69770 + t69773 + t69775 - t69776 + t69779 - t69782 - t69784;
    (t69788,)
}
