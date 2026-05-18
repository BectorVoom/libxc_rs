//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1264/1303 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1264<F: Float>(t3701: F, t6324: F, t571: F, t6347: F, t12461: F, t12087: F, t12094: F, t12103: F, t12105: F, t12109: F, t12114: F, t1307: F, t1388: F, t16497: F, t1799: F, t19678: F, t19683: F, t19684: F, t19685: F, t19686: F, t19687: F, t3918: F, t5126: F, t5160: F, t9793: F, t9797: F, t9820: F, t9824: F) -> F {
    let t20077 = t6324 * t3701;
    let t20081 = t571 * t6347;
    let t20085 = t6324 * t12461;
    let t20092 = -F::new(3.0) * t1307 * t20077 * t3918 + F::new(6.0) * t1307 * t20081 * t5126 + F::new(2.0) * t1388 * t20085 * t5160 + F::new(6.0) * t16497 * t1799 * t3918 + t12087 - t12094 + t12103 - t12105 - t12109 - t12114 - t19678 - t19683 + t19684 + t19685 - t19686 + t19687 + t9793 + t9797 - t9820 - t9824;
    t20092
}
