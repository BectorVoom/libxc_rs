//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 880/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk880<F: Float>(t774: F, t8305: F, t1364: F, t782: F, t2174: F, t1378: F, t2162: F, t125: F, t3664: F, t3671: F, t8313: F, t1385: F, t8130: F, t2383: F, t3689: F, t2143: F, t3622: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10572 = t8305 * t774;
    let t10573 = t1364 * t782;
    let t10578 = t2174 * t774;
    let t10579 = t1378 * t782;
    let t10584 = t1378 * t2162;
    let t10590 = t125 * t3664;
    let t10600 = 7.0 / 2304.0 * t8313 * t3671;
    let t10617 = t8130 * t1385;
    let t10620 = 7.0 / 576.0 * t2383 * t3689;
    let t10630 = 7.0 / 72.0 * t2143 * t3622;
    (t10572, t10573, t10578, t10579, t10584, t10590, t10600, t10617, t10620, t10630)
}
