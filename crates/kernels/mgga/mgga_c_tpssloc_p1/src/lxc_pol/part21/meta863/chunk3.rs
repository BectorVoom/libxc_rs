//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3144/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3144<F: Float>(t11583: F, t17635: F, t11570: F, t17691: F, t15372: F, t4889: F, t11529: F, t1174: F, t6126: F, t11569: F, t15278: F, t15288: F, t15357: F, t15360: F, t18416: F, t3447: F, t3449: F, t3469: F, t3475: F, t460: F, t4919: F, t4934: F, t52216: F, t52220: F, t6144: F, t8034: F) -> F {
    let t65077 = t11583 * t17635;
    let t65087 = t11570 * t17691;
    let t65093 = t4889 * t15372;
    let t65112 = t1174 * t11529 * t6126;
    let t65114 = F::cast_from(0.55555555555555555554e-3_f64) * t3447 * t18416 * t15288 + F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t3449 * t65077 + F::cast_from(0.22222222222222222222e-2_f64) * t3447 * t4919 * t52216 + F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t4919 * t52220 - F::cast_from(0.14814814814814814814e-2_f64) * t3447 * t11569 * t65087 + F::cast_from(0.44444444444444444444e-2_f64) * t4889 * t15360 + F::cast_from(0.2962962962962962963e-2_f64) * t65093 - F::cast_from(0.16666666666666666666e-2_f64) * t1174 * t4934 * t8034 * t15357 - F::cast_from(0.83333333333333333332e-3_f64) * t1174 * t4934 * t6144 * t3469 * t460 - F::cast_from(0.83333333333333333332e-3_f64) * t1174 * t4934 * t6144 * t3475 * t460 + F::cast_from(0.14814814814814814814e-2_f64) * t4889 * t15278 + F::cast_from(0.12345679012345679012e-3_f64) * t65112;
    t65114
}
