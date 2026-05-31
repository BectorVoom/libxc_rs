//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1330/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1330<F: Float>(t17079: F, t17108: F, t2752: F, t5660: F, t13105: F, t16685: F, t16688: F, t16691: F, t16692: F, t16695: F, t16696: F, t1877: F, t193: F, t202: F, t4303: F, t4307: F, t868: F, t870: F, t9789: F, t9793: F, t9797: F, t9820: F, t9824: F, t9876: F, t9884: F, t9887: F, t9890: F) -> (F, F) {
    let t17109 = t17079 + t17108;
    let t17116 = t5660 * t2752;
    let t17119 = t17109 * t193 * t202 * t870 - t17116 * t1877 * t868 - F::cast_from(2.0_f64) * t1877 * t4303 * t4307 + t13105 + t16685 + t16688 + t16691 + t16692 + t16695 + t16696 - t9789 + t9793 + t9797 - t9820 - t9824 - t9876 - t9884 + t9887 + t9890;
    (t17109, t17119)
}
