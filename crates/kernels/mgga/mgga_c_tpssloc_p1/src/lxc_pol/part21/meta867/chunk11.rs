//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3175/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3175<F: Float>(t11668: F, t11692: F, t1214: F, t1227: F, t14706: F, t15470: F, t15474: F, t15560: F, t15564: F, t15594: F, t15615: F, t15681: F, t15740: F, t1735: F, t248: F, t3506: F, t3508: F, t3516: F, t3577: F, t3578: F, t4582: F, t4889: F, t4972: F, t5030: F, t50992: F, t51002: F, t52609: F, t52619: F, t52766: F, t52879: F, t55662: F, t5971: F, t61855: F, t61910: F, t62044: F, t65264: F) -> F {
    let t65764 = -t15594 * t5030 / F::new(1152.0) + t3506 * t248 * t1214 * t65264 * t3508 / F::new(768.0) - t1227 * t4582 * t4972 * t55662 / F::new(2304.0) - t1227 * t4582 * t15615 * t62044 / F::new(768.0) - t52879 * t15560 / F::new(1152.0) + t52766 * t15564 / F::new(2304.0) - t15740 * t15470 / F::new(1152.0) - t15740 * t15474 / F::new(2304.0) - t1227 * t4582 * t15615 * t61910 / F::new(768.0) - t1227 * t4582 * t50992 * t61855 / F::new(192.0) - F::new(5.0) / F::new(15552.0) * t52609 - F::new(2.0) / F::new(81.0) * t4889 * t15681 + F::new(5.0) / F::new(384.0) * t1227 * t4582 * t51002 * t61855 - F::new(5.0) / F::new(13824.0) * t11692 * t11668 * t5971 * t3516 - t52619 / F::new(3456.0) - t3577 * t3578 * t1735 * t14706 / F::new(2304.0);
    t65764
}
