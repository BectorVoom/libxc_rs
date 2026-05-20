//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2994/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2994<F: Float>(t10390: F, t18041: F, t10422: F, t18024: F, t3070: F, t13969: F, t17733: F, t3130: F, t10214: F, t1041: F, t10883: F, t10937: F, t14080: F, t14187: F, t17596: F, t17697: F, t17712: F, t17998: F, t2960: F, t3039: F, t3041: F, t3117: F, t3121: F, t43248: F, t43253: F, t4582: F, t4585: F, t4588: F, t48496: F, t50272: F, t59751: F, t61798: F, t61855: F, t61910: F, t973: F) -> F {
    let t62682 = t10390 * t18041;
    let t62687 = t3070 * t10422 * t18024;
    let t62704 = t3130 * t13969 * t17733;
    let t62722 = -t50272 / F::new(324.0) + t62682 / F::new(1728.0) - F::new(5.0) / F::new(1296.0) * t10937 * t17998 - t62687 / F::new(864.0) - t43248 / F::new(972.0) - t43253 - F::new(7.0) / F::new(54.0) * t973 * t10214 * t59751 - F::new(2.0) / F::new(81.0) * t2960 * t17596 - t3039 * t4582 * t17712 * t3121 / F::new(3072.0) + t10883 * t4582 * t17712 * t3041 / F::new(3072.0) + t62704 / F::new(576.0) + F::new(5.0) / F::new(6912.0) * t1041 * t4582 * t4588 * t61798 + F::new(5.0) / F::new(2592.0) * t3117 * t17697 + F::new(5.0) / F::new(5184.0) * t1041 * t4582 * t14187 * t61910 + F::new(55.0) / F::new(15552.0) * t1041 * t4582 * t48496 * t61855 + t14080 * t4585 / F::new(108.0);
    t62722
}
