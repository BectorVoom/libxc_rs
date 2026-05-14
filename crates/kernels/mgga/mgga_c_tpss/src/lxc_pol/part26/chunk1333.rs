//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1333/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1333<F: Float>(t116: F, t22175: F, t645: F, t6479: F, t1339: F, t13452: F, t13473: F, t13974: F, t1897: F, t1899: F, t20957: F, t3502: F, t4631: F, t4674: F, t5463: F, t5986: F, t6054: F, t6058: F, t626: F, t646: F, t68156: F, t68801: F, t68808: F, t68810: F, t68814: F, t68817: F, t68822: F, t68826: F, t68830: F, t68833: F) -> (F, F, F) {
    let t72774 = t22175 * t116;
    let t72781 = t6479 * t645;
    let t72790 = -2.0 * t4674 * t6054 * t626 - 4.0 * t1339 * t68156 - 4.0 * t1339 * t72781 - t13452 * t1897 - 4.0 * t13473 * t5986 + t13974 * t1899 - 4.0 * t20957 * t3502 - t4631 * t6054 + t5463 * t6058 - 2.0 * t646 * t72774 + t68801 - t68808 - t68810 + t68814 + t68817 - t68822 + t68826 - t68830 + t68833;
    (t72774, t72781, t72790)
}
