//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1293/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1293<F: Float>(t104235: F, t104239: F, t1417: F, t1932: F, t2121: F, t2130: F, t2133: F, t2136: F, t2140: F, t21749: F, t22115: F, t22154: F, t22214: F, t22301: F, t22309: F, t22314: F, t24741: F, t27604: F, t27629: F, t29594: F, t3448: F, t488: F, t6169: F, t6192: F, t6207: F, t6211: F, t7345: F, t8040: F, t8048: F, t86146: F, t86164: F, t86171: F, t86278: F, t95687: F) -> (F,) {
    let t109593 = -t95687 * t6192 / 384.0 - t24741 * t22154 / 768.0 - t2121 * t3448 * t21749 / 48.0 + t86171 * t22301 / 1536.0 + t27604 * t6207 / 144.0 - 0.30279567070605293142e-3 * t104239 * t8040 - 0.30279567070605293142e-3 * t27629 * t29594 + 0.60559134141210586284e-3 * t104235 * t8040 + t27604 * t6211 / 72.0 - t7345 * t22214 / 2304.0 + t86146 * t22309 / 256.0 - t86164 * t22314 / 256.0 + t86278 + t22115 * t2140 * t488 / 1536.0 - t6169 * t8048 * t488 / 96.0 - 0.72670960969452703541e-1 / t2130 / t1417 * t1932 * t2133 * t2136;
    (t109593,)
}
