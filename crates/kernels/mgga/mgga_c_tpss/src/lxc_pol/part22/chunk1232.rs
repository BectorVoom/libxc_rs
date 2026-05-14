//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1232/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1232<F: Float>(t1497: F, t1692: F, t18268: F, t18728: F, t18803: F, t18807: F, t20048: F, t20050: F, t20058: F, t20417: F, t20514: F, t20526: F, t2439: F, t2829: F, t5849: F, t6354: F, t64888: F, t64923: F, t64966: F, t64972: F, t64989: F, t65013: F, t66608: F, t66615: F, t66631: F, t66641: F) -> (F,) {
    let t66897 = -3.0 * t20417 * t65013 + 3.0 * t2439 * t5849 * t20058 + t66615 + t1692 * t18803 * t1497 / 2.0 - 3.0 * t18728 * t64989 - t1692 * t20514 * t18268 + t1692 * t6354 * t2829 / 2.0 + t66631 + 2.0 * t66608 * t20048 + t20526 * t64888 - t66641 + 3.0 * t18728 * t64966 - t1692 * t18807 * t20050 + 6.0 * t18728 * t64923 - 3.0 * t18728 * t64972;
    (t66897,)
}
