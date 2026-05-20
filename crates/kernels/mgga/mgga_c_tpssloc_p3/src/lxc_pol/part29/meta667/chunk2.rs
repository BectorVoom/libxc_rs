//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2228/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2228<F: Float>(t16257: F, t26309: F, t5293: F, t80820: F, t5259: F, t80816: F, t16244: F, t22833: F, t5303: F, t16366: F, t16370: F, t91094: F, t91096: F, t91098: F, t91101: F, t91103: F, t91105: F, t91107: F, t91109: F, t91114: F, t91116: F) -> F {
    let t91118 = t26309 * t16257;
    let t91120 = t80820 * t5293;
    let t91121 = F::new(7.0) / F::new(1152.0) * t91120;
    let t91122 = t80816 * t5259;
    let t91124 = t22833 * t16244;
    let t91126 = t80816 * t5303;
    let t91128 = t22833 * t16366;
    let t91130 = t22833 * t16370;
    let t91132 = t91094 / F::new(384.0) + t91096 / F::new(384.0) + t91098 / F::new(768.0) + t91101 / F::new(192.0) - F::new(5.0) / F::new(384.0) * t91103 + t91105 / F::new(256.0) - t91107 / F::new(1536.0) - t91109 / F::new(768.0) - t91114 + t91116 / F::new(384.0) + t91118 / F::new(384.0) + t91121 + t91122 / F::new(192.0) + t91124 / F::new(192.0) + t91126 / F::new(192.0) + t91128 / F::new(192.0) + t91130 / F::new(384.0);
    t91132
}
