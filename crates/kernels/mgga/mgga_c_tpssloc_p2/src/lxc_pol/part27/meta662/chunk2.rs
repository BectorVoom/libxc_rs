//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2323/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2323<F: Float>(t16257: F, t26309: F, t5293: F, t80820: F, t5259: F, t80816: F, t16244: F, t22833: F, t5303: F, t16366: F, t16370: F, t91094: F, t91096: F, t91098: F, t91101: F, t91103: F, t91105: F, t91107: F, t91109: F, t91114: F, t91116: F) -> F {
    let t91118 = t26309 * t16257;
    let t91120 = t80820 * t5293;
    let t91121 = F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t91120;
    let t91122 = t80816 * t5259;
    let t91124 = t22833 * t16244;
    let t91126 = t80816 * t5303;
    let t91128 = t22833 * t16366;
    let t91130 = t22833 * t16370;
    let t91132 = t91094 / F::cast_from(384.0_f64) + t91096 / F::cast_from(384.0_f64) + t91098 / F::cast_from(768.0_f64) + t91101 / F::cast_from(192.0_f64) - F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t91103 + t91105 / F::cast_from(256.0_f64) - t91107 / F::cast_from(1536.0_f64) - t91109 / F::cast_from(768.0_f64) - t91114 + t91116 / F::cast_from(384.0_f64) + t91118 / F::cast_from(384.0_f64) + t91121 + t91122 / F::cast_from(192.0_f64) + t91124 / F::cast_from(192.0_f64) + t91126 / F::cast_from(192.0_f64) + t91128 / F::cast_from(192.0_f64) + t91130 / F::cast_from(384.0_f64);
    t91132
}
