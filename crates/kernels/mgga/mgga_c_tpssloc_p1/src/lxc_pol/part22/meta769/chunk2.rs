//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2612/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2612<F: Float>(t11665: F, t11719: F, t11721: F, t1174: F, t1196: F, t1215: F, t1227: F, t15740: F, t18300: F, t18346: F, t18360: F, t18965: F, t19068: F, t22154: F, t44725: F, t44863: F, t45002: F, t4582: F, t4987: F, t5005: F, t5011: F, t52766: F, t53034: F, t66241: F, t66255: F, t67060: F, t70458: F, t72445: F, t974: F) -> F {
    let t72911 = F::new(5.0) / F::new(4608.0) * t5005 * t19068 + t53034 + F::new(5.0) / F::new(768.0) * t5005 * t18346 + t66241 / F::new(768.0) + F::new(5.0) / F::new(13824.0) * t1227 * t4582 * t4987 * t70458 + F::new(3.0) / F::new(512.0) * t11719 * t4582 * t18300 * t11721 * t5011 + t44863 * t4582 * t72445 * t44725 * t1215 / F::new(128.0) - t1174 * t974 * t1196 * t67060 / F::new(288.0) - t15740 * t18360 / F::new(768.0) - t11665 * t22154 / F::new(1536.0) + t45002 / F::new(10368.0) + t52766 * t18965 / F::new(1536.0) - t66255 / F::new(768.0);
    t72911
}
