//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2282/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2282<F: Float>(t1222: F, t18574: F, t11789: F, t1227: F, t248: F, t5975: F, t18321: F, t3548: F, t15437: F, t15502: F, t15506: F, t4965: F, t5023: F) -> (F, F, F, F, F, F) {
    let t65681 = t18574 * t1222;
    let t65689 = t1227 * t248 * t11789 * t5975;
    let t65691 = t18321 * t3548;
    let t65703 = t15437 * t15502;
    let t65706 = t15437 * t15506;
    let t65709 = t4965 * t5023;
    (t65681, t65689, t65691, t65703, t65706, t65709)
}
