//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 530/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk530<F: Float>(t21: F, t65: F, t64: F, t159: F, t216: F, t756: F, t760: F, t764: F, t238: F, t210: F, t124: F, t2116: F, t762: F, t2133: F, t227: F, t767: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2138 = 1.0 / t65 / t21;
    let t2139 = t64 * t2138;
    let t2140 = t2139 * t159;
    let t2142 = 35.0 / 432.0 * t2140 * t216;
    let t2143 = t756 * t760;
    let t2144 = t2143 * t764;
    let t2146 = t159 * t238;
    let t2147 = t210 * t2146;
    let t2148 = t124 * t2116;
    let t2149 = t762 * t2148;
    let t2153 = t762 * t124 * t2133;
    let t2157 = 1.0 / t767 / t227;
    (t2138, t2139, t2140, t2142, t2143, t2144, t2146, t2147, t2149, t2153, t2157)
}
