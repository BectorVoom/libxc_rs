//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3190/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3190<F: Float>(t13969: F, t19057: F, t3506: F, t11546: F, t11665: F, t11668: F, t11692: F, t1174: F, t1227: F, t15434: F, t15622: F, t15627: F, t15737: F, t18360: F, t18584: F, t3243: F, t44996: F, t45002: F, t4582: F, t4889: F, t4984: F, t52601: F, t52813: F, t53023: F, t53026: F, t53033: F, t53238: F, t61855: F, t6192: F, t6230: F, t63415: F) -> F {
    let t66241 = t3506 * t13969 * t19057;
    let t66254 = -t53023 / F::new(1728.0) + t15737 * t15622 / F::new(768.0) + t53238 * t15627 / F::new(256.0) - F::new(5.0) / F::new(3888.0) * t53026 - F::new(7.0) / F::new(648.0) * t1174 * t11546 * t63415 + F::new(14.0) / F::new(243.0) * t4889 * t15434 - t44996 * t6192 / F::new(2304.0) - t11665 * t18584 / F::new(1152.0) - t11665 * t18360 / F::new(1152.0) + t53033 / F::new(2592.0) + t66241 / F::new(1152.0) + t52813 * t4984 / F::new(144.0) + t45002 / F::new(5184.0) - F::new(5.0) / F::new(432.0) * t1227 * t4582 * t52601 * t61855 - F::new(5.0) / F::new(13824.0) * t11692 * t11668 * t6230 * t3243;
    t66254
}
