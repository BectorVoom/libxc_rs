//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1122/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1122<F: Float>(t1080: F, t9176: F, t15422: F, t1089: F, t11938: F, t11988: F, t11989: F, t11990: F, t15239: F, t15241: F, t15243: F, t15251: F, t15259: F, t15264: F, t15268: F, t15273: F, t15277: F, t15283: F, t15288: F, t9221: F, t9331: F) -> (F, F) {
    let t15423 = t9176 * t1080;
    let t15424 = t15422 * t15423;
    let t15426 = F::new(0.10254018858216406658e4) * t1089 * t15424;
    let t15440 = -t9331 + F::new(0.41203703703703703703e-2) * t9221 + F::new(0.82407407407407407408e-2) * t11938 + t11988 - t11989 - t11990 + F::new(0.20601851851851851852e-2) * t15239 + F::new(0.10300925925925925926e-1) * t15259 - F::new(0.37083333333333333333e-1) * t15264 - F::new(0.12361111111111111111e-1) * t15268 - F::new(0.61805555555555555557e-2) * t15241 + F::new(0.55625000000000000001e-1) * t15273 + F::new(0.37083333333333333334e-1) * t15277 - F::new(0.30902777777777777778e-2) * t15243 - F::new(0.61805555555555555555e-2) * t15283 + F::new(0.18541666666666666667e-1) * t15288 + F::new(0.92708333333333333333e-2) * t15251;
    (t15426, t15440)
}
