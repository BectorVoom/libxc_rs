//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2559/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2559<F: Float>(t21886: F, t3359: F, t11350: F, t1136: F, t11420: F, t15126: F, t15136: F, t15146: F, t18609: F, t18612: F, t18616: F, t18619: F, t18647: F, t18650: F, t18651: F, t21854: F, t21855: F, t21887: F, t21890: F, t3332: F, t3357: F, t44177: F, t44179: F, t44361: F, t4819: F, t4840: F, t4862: F, t51604: F, t51680: F, t6056: F, t63454: F, t63602: F) -> F {
    let t71729 = t21886 * t3359;
    let t71752 = F::cast_from(0.1929837539843104208e3_f64) * t15146 * t18647 + F::cast_from(0.62071215503128080361e4_f64) * t51604 * t18651 + F::cast_from(0.11579025239058625248e4_f64) * t11350 * t21855 * t1136 - F::cast_from(0.57895126195293126243e3_f64) * t11420 * t6056 * t4819 - F::cast_from(0.24828486201251232145e5_f64) * t44361 * t21890 * t1136 - F::cast_from(2.0_f64) * t3332 * t21887 * t1136 + F::cast_from(0.32163958997385070134e2_f64) * t3357 * t71729 * t1136 + F::cast_from(0.6207121550312808036e4_f64) * t11350 * t18650 * t4819 + F::cast_from(0.19964560303604640732e6_f64) * t44177 * t21854 * t44179 * t1136 - F::cast_from(0.35089341735807877242e1_f64) * t63454 * t4840 + F::cast_from(0.51947577317044391276e2_f64) * t63602 * t4862 - F::cast_from(0.35089341735807877242e1_f64) * t15136 * t18612 + F::cast_from(0.51947577317044391276e2_f64) * t15126 * t18616 - F::cast_from(0.31168546390226634765e3_f64) * t51680 * t18609 + F::cast_from(0.10389515463408878255e3_f64) * t15126 * t18619;
    t71752
}
