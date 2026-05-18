//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1016/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1016<F: Float>(t177: F, t4744: F, t737: F, t189: F, t4573: F, t581: F, t10728: F, t725: F, t2337: F, t10511: F, t3431: F, t3565: F) -> (F, F, F, F, F) {
    let t14055 = t4744 * t177;
    let t14056 = t14055 * t737;
    let t14057 = F::new(0.5848223622634646207e0) * t14056;
    let t14058 = t189 * t4573;
    let t14059 = t14058 * t581;
    let t14061 = F::new(24.0) * t10728 * t14059;
    let t14062 = t725 * t4573;
    let t14063 = t2337 * t14062;
    let t14064 = F::new(12.0) * t14063;
    let t14065 = F::new(0.21687162600603479684e-1) * t10511;
    let t14066 = t3565 * t3431;
    (t14057, t14061, t14064, t14065, t14066)
}
