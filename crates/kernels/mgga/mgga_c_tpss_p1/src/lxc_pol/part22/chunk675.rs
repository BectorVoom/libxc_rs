//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 675/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk675<F: Float>(t1173: F, t1186: F, t1206: F, t198: F, t2281: F, t2285: F, t2310: F, t3180: F, t3182: F, t3183: F, t3184: F, t3189: F, t3192: F, t3194: F, t3196: F, t3199: F, t3201: F, t3202: F, t3205: F, t509: F) -> (F, F) {
    let t3209 = F::new(8.0) * t1173 * t1186;
    let t3210 = -t198 * t3202 * t3205 * t509 + F::new(6.0) * t1206 * t3183 * t3184 - t2281 - t2285 + t2310 - t3180 - t3182 + t3189 - t3192 + t3194 - t3196 + t3199 - t3201 - t3209;
    (t3209, t3210)
}
