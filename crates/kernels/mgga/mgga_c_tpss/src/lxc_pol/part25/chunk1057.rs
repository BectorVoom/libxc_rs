//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1057/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1057<F: Float>(t5161: F, t9176: F, t1080: F, t11873: F, t11938: F, t12145: F, t12146: F, t15239: F, t15241: F, t15243: F, t15251: F, t15259: F, t15264: F, t15268: F, t15273: F, t15277: F, t15283: F, t15288: F, t9221: F, t9477: F) -> (F, F) {
    let t15770 = t5161 * t9176;
    let t15771 = t15770 * t1080;
    let t15788 = -t9477 + 0.76103703703703703703e-2 * t9221 + 0.1522074074074074074e-1 * t11938 + 0.761037037037037037e-2 * t11873 - t12145 - t12146 + 0.3805185185185185185e-2 * t15239 + 0.19025925925925925925e-1 * t15259 - 0.68493333333333333331e-1 * t15264 - 0.2283111111111111111e-1 * t15268 - 0.11415555555555555555e-1 * t15241 + 0.10274e0 * t15273 + 0.68493333333333333332e-1 * t15277 - 0.57077777777777777777e-2 * t15243 - 0.11415555555555555555e-1 * t15283 + 0.34246666666666666666e-1 * t15288 + 0.17123333333333333333e-1 * t15251;
    (t15771, t15788)
}
