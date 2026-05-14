//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1290/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1290<F: Float>(t1760: F, t21017: F, t23794: F, t19597: F, t6103: F, t2056: F, t21203: F, t3499: F, t16037: F, t1688: F, t626: F, t13866: F, t1705: F, t935: F, t21060: F, t5570: F) -> (F, F, F, F, F, F, F) {
    let t69427 = 6.0 * t1760 * t23794 * t21017;
    let t69437 = 4.0 * t6103 * t19597;
    let t69439 = 2.0 * t2056 * t21203;
    let t69441 = 2.0 * t3499 * t21203;
    let t69444 = 2.0 * t626 * t16037 * t1688;
    let t69452 = t1705 * t13866 * t935;
    let t69458 = t21060 * t5570;
    (t69427, t69437, t69439, t69441, t69444, t69452, t69458)
}
