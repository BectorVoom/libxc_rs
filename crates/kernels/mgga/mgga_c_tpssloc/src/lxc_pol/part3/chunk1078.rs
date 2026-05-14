//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1078/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1078<F: Float>(t14687: F, t15856: F, t3701: F, t5356: F, t3719: F, t5127: F, t5168: F, t588: F, t592: F, t5166: F, t5187: F, t571: F, t11975: F, t11977: F, t11981: F, t2528: F, t5154: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t15857 = t14687 + t15856;
    let t15868 = t5356 * t3701;
    let t15872 = t5127 * t3719;
    let t15875 = t588 * t5168;
    let t15876 = 8.0 * t15875;
    let t15877 = t592 * t5168;
    let t15878 = 8.0 * t15877;
    let t15880 = 8.0 * t588 * t5166;
    let t15883 = t571 * t5187;
    let t15887 = 4.0 * t11975;
    let t15888 = 4.0 * t11977;
    let t15889 = 32.0 * t11981;
    let t15890 = t5154 * t2528;
    (t15857, t15868, t15872, t15876, t15878, t15880, t15883, t15887, t15888, t15889, t15890)
}
