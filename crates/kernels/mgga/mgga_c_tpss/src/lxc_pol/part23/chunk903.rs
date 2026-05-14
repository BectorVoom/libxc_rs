//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 903/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk903<F: Float>(t200: F, t45: F, t202: F, t57: F, t2112: F, t2334: F, t2239: F, t680: F, t37: F, t691: F, t157: F, t36: F, t2435: F, t255: F) -> (F, F, F, F, F, F, F) {
    let t8050 = 1.0 / t200 / t45;
    let t8061 = 1.0 / t202 / t57;
    let t8077 = t2112 * t2334;
    let t8079 = t680 * t2239;
    let t8082 = t37 * t691;
    let t8087 = t36 * t157;
    let t8096 = 1.0 / t2435 / t255;
    (t8050, t8061, t8077, t8079, t8082, t8087, t8096)
}
