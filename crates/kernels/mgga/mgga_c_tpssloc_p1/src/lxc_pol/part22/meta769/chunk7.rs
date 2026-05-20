//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2617/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2617<F: Float>(t22243: F, t486: F, t1222: F, t22116: F, t1216: F, t1227: F, t15615: F, t1743: F, t18573: F, t22197: F, t3490: F, t3506: F, t3515: F, t4582: F, t488: F, t4978: F, t51002: F, t53271: F, t53273: F, t53274: F, t66449: F, t66452: F, t66458: F, t70330: F, t70339: F) -> F {
    let t73028 = t486 * t22243;
    let t73043 = t22116 * t1222;
    let t73048 = -t1227 * t4582 * t15615 * t70339 / F::new(256.0) + t53271 - t53273 - t53274 / F::new(648.0) - t66449 / F::new(72.0) - t66452 / F::new(48.0) - F::new(2.0) / F::new(81.0) * t66458 + t3506 * t4582 * t73028 * t4978 / F::new(1536.0) - t3515 * t4582 * t73028 * t1216 / F::new(3072.0) + F::new(5.0) / F::new(384.0) * t1227 * t4582 * t51002 * t70330 + F::new(5.0) / F::new(4608.0) * t3490 * t22197 + t73043 / F::new(4608.0) - t18573 * t1743 * t488 / F::new(192.0);
    t73048
}
