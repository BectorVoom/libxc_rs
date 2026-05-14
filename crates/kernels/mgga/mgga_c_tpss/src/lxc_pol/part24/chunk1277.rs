//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1277/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1277<F: Float>(t1165: F, t68888: F, t6095: F, t645: F, t1688: F, t42710: F, t50656: F, t13565: F, t5531: F, t69023: F, t69026: F, t1338: F, t13546: F, t17916: F, t19457: F, t19462: F, t24587: F, t3537: F, t4674: F, t5514: F, t65490: F, t69012: F) -> (F, F) {
    let t69032 = 2.0 * t1165 * t68888;
    let t69037 = t6095 * t645;
    let t69051 = 2.0 * t42710 * t1688;
    let t69053 = 2.0 * t50656 * t1688;
    let t69055 = 2.0 * t13565 * t5531;
    let t69057 = 4.0 * t69023 * t1688;
    let t69059 = 4.0 * t69026 * t1688;
    let t69060 = 4.0 * t1338 * t65490 + 4.0 * t1338 * t69037 + 2.0 * t13546 * t5514 + 2.0 * t17916 * t4674 + 2.0 * t19457 * t4674 + 4.0 * t19462 * t3537 + 4.0 * t24587 * t3537 + 2.0 * t645 * t69012 + t69032 + t69051 + t69053 + t69055 + t69057 + t69059;
    (t69037, t69060)
}
