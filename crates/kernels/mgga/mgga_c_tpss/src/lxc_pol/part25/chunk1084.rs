//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1084/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1084<F: Float>(t14947: F, t967: F, t11456: F, t11459: F, t11462: F, t14902: F, t14908: F, t14913: F, t14917: F, t14922: F, t14925: F, t14928: F, t14931: F, t14935: F, t14939: F, t14943: F, t2731: F, t2748: F, t4996: F, t5005: F, t8456: F, t8472: F, t8577: F, t8588: F, t8976: F, t925: F) -> F {
    let t14948 = t967 * t14947;
    let t14953 = t8976 * t4996 / F::new(576.0) - t14902 / F::new(4608.0) - t8456 / F::new(1296.0) - t8472 / F::new(13824.0) + t967 * t14908 / F::new(768.0) - t967 * t14913 / F::new(1152.0) - t11456 - t11459 + t11462 - t2731 * t14917 / F::new(1536.0) + t8577 * t14922 / F::new(3072.0) - t925 * t14925 / F::new(36.0) + t925 * t14928 / F::new(108.0) + F::new(7.0) / F::new(648.0) * t925 * t14931 - F::new(5.0) / F::new(2304.0) * t967 * t14935 + F::new(5.0) / F::new(6912.0) * t967 * t14939 + F::new(5.0) / F::new(5184.0) * t967 * t14943 + F::new(5.0) / F::new(20736.0) * t14948 + t2748 * t5005 / F::new(432.0) + t8588 / F::new(162.0);
    t14953
}
