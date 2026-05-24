//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 500/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk500<F: Float>(t99: F, t107: F, t680: F, t691: F, t205: F, t256: F, t21: F, t65: F, t64: F, t159: F, t216: F, t756: F, t760: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2083 = F::new(1.0) / t99;
    let t2091 = F::new(1.0) / t107;
    let t2112 = t680 * t691;
    let t2115 = t205 * t256;
    let t2138 = F::new(1.0) / t65 / t21;
    let t2139 = t64 * t2138;
    let t2140 = t2139 * t159;
    let t2142 = F::new(35.0) / F::new(432.0) * t2140 * t216;
    let t2143 = t756 * t760;
    (t2083, t2091, t2112, t2115, t2138, t2139, t2140, t2142, t2143)
}
