//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1364/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1364<F: Float>(t33: F, t259: F, t479: F, t70784: F, t73577: F, t1289: F, t13335: F, t1893: F, t20936: F, t22100: F, t3431: F, t4579: F, t57: F, t581: F, t6048: F, t6534: F, t70965: F, t118: F, t1273: F, t1338: F, t13470: F, t13546: F, t1897: F, t19261: F, t2056: F, t20706: F, t20944: F, t22114: F, t22182: F, t3499: F, t3537: F, t4675: F, t5986: F, t626: F, t6540: F, t70986: F, t70989: F, t70991: F, t70994: F, t70999: F, t71002: F, t71010: F, t71012: F, t71017: F, t73155: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F,) {
    let t34 = t33 <= zeta_threshold;
    let t386 = rho1 <= dens_threshold || t34;
    let t480 = t259 < t479;
    let t73578 = piecewise3(t480, t73577, t70784);
    let t73590 = piecewise3(t386, t70965, t73578 * t57 / 2.0 - t22100 * t581 / 2.0 - t20936 * t1289 - t6534 * t3431 - t6048 * t4579 / 2.0 - t1893 * t13335 / 2.0);
    let t73601 = -2.0 * t626 * t1897 * t13546 - 2.0 * t19261 * t4675 - 2.0 * t20706 * t4675 - 2.0 * t5986 * t13470 - 4.0 * t626 * t6540 * t3537 - t118 * (t73155 + t73590) + t70986 - t70989 - t70991 - t70994 + t70999 - t71002 - t71010 - t71012 + t22182 * t1273 - t71017 - 4.0 * t2056 * t22114 - 4.0 * t3499 * t22114 - 4.0 * t626 * t20944 * t1338;
    (t73601,)
}
