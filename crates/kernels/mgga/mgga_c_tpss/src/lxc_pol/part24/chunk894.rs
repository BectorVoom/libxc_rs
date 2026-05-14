//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 894/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk894<F: Float>(t735: F, t8027: F, t256: F, t750: F, t7813: F, t7875: F, t7878: F, t200: F, t45: F, t202: F, t57: F, t37: F, t691: F, t157: F, t36: F, t2435: F, t255: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8029 = 0.35089341735807877242e1 * t735 * t8027;
    let t8030 = t256 * t750;
    let t8038 = t7875 * t7813 * t7878;
    let t8040 = 0.10254018858216406658e4 * t735 * t8038;
    let t8050 = 1.0 / t200 / t45;
    let t8061 = 1.0 / t202 / t57;
    let t8082 = t37 * t691;
    let t8087 = t36 * t157;
    let t8096 = 1.0 / t2435 / t255;
    (t8029, t8030, t8038, t8040, t8050, t8061, t8082, t8087, t8096)
}
