//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2609/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2609<F: Float>(t18231: F, t3961: F, t1222: F, t22169: F, t11539: F, t1174: F, t21745: F, t11546: F, t11692: F, t1227: F, t15654: F, t18321: F, t18342: F, t19083: F, t22284: F, t3440: F, t3578: F, t45134: F, t4582: F, t4733: F, t4987: F, t4989: F, t5005: F, t5033: F, t52893: F, t52919: F, t6230: F, t70316: F, t70330: F, t70339: F, t71133: F, t71197: F) -> (F, F) {
    let t72788 = t18231 * t3961;
    let t72798 = t22169 * t1222;
    let t72815 = t1174 * t11539 * t21745;
    let t72823 = F::new(5.0) / F::new(768.0) * t1227 * t4582 * t15654 * t70339 - t52893 * t3578 * t72788 / F::new(256.0) + t45134 * t22284 / F::new(1536.0) + t11692 * t3578 * t6230 * t4733 / F::new(1536.0) + F::new(19.0) / F::new(864.0) * t72798 + F::new(5.0) / F::new(4608.0) * t1227 * t4582 * t4987 * t70316 + F::new(55.0) / F::new(15552.0) * t1227 * t4582 * t52919 * t70330 - F::new(5.0) / F::new(432.0) * t19083 * t4989 + F::new(5.0) / F::new(2304.0) * t5005 * t18342 + F::new(11.0) / F::new(81.0) * t18321 * t5033 + t72815 / F::new(216.0) + t1174 * t3440 * t71197 / F::new(6.0) - F::new(7.0) / F::new(54.0) * t1174 * t11546 * t71133;
    (t72788, t72823)
}
