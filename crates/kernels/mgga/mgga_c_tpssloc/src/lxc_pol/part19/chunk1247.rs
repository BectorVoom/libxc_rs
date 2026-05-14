//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1247/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1247<F: Float>(t10962: F, t3103: F, t3078: F, t3082: F, t3089: F, t1058: F, t3068: F, t3087: F, t363: F, t11065: F, t42387: F, t10408: F, t1041: F, t10485: F, t10877: F, t14172: F, t14228: F, t2250: F, t2770: F, t3070: F, t3071: F, t3073: F, t3134: F, t39097: F, t42468: F, t43317: F, t43322: F, t43325: F, t43332: F, t43336: F, t43341: F, t43343: F, t4582: F, t884: F, t973: F, t974: F) -> (F,) {
    let t43350 = t10962 * t3103;
    let t43352 = t3078 * t3082;
    let t43354 = t3089 * t3082;
    let t43358 = t1058 * t363 * t3087 * t3068;
    let t43361 = t11065 * t42387;
    let t43366 = t973 * t974 * t43317 * t39097 / 6.0 + t43322 * t10485 / 128.0 + 2.0 / 81.0 * t43325 + 5.0 / 1152.0 * t3070 * t10408 * t2770 * t2250 * t14228 + t43332 / 54.0 + t43336 / 1728.0 - 5.0 / 10368.0 * t43341 + t43343 * t3134 / 256.0 - 5.0 / 384.0 * t1041 * t4582 * t14172 * t42468 + t43350 / 384.0 - t43352 / 2304.0 - 19.0 / 1296.0 * t43354 + 19.0 / 216.0 * t43358 * t3073 - t43361 * t3071 * t10877 * t884 / 192.0;
    (t43366,)
}
