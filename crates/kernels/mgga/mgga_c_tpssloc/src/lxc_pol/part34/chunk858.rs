//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 858/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk858<F: Float>(t11459: F, t14702: F, t18203: F, t18219: F, t18229: F, t21760: F, t21764: F, t21767: F, t21771: F, t21774: F, t21778: F, t423: F, t11310: F, t11365: F, t1148: F, t15126: F, t15136: F, t15207: F, t21827: F, t21901: F, t21907: F, t21939: F, t21942: F, t21947: F, t21952: F, t21956: F, t21958: F, t21960: F, t21963: F, t21975: F, t3357: F, t3401: F, t436: F, t4835: F, t6037: F, t6069: F, t6085: F, t6088: F) -> (F, F) {
    let t21988 = -t11459 + 0.23744444444444444444e-1 * t14702 + 0.11872222222222222222e-1 * t18203 - 0.35616666666666666666e-1 * t18219 - 0.17808333333333333333e-1 * t18229 + 0.19787037037037037037e-1 * t21760 - 0.71233333333333333332e-1 * t21764 - 0.35616666666666666666e-1 * t21767 + 0.10685e0 * t21771 + 0.10685e0 * t21774 + 0.17808333333333333333e-1 * t21778;
    let t21990 = 0.621814e-1 * t21988 * t423;
    let t21991 = -t21901 + 0.17544670867903938621e1 * t4835 * t6085 + 0.51947577317044391276e2 * t15126 * t6088 - 0.10389515463408878255e3 * t11365 * t21907 + 0.5848223622634646207e0 * t1148 * t21939 + 0.10254018858216406658e4 * t11310 * t21942 - 0.35089341735807877242e1 * t15136 * t6069 + 0.35089341735807877242e1 * t3401 * t21947 - 6.0 * t15207 * t6037 + 6.0 * t3357 * t21952 - t21956 - t21958 - t21960 + t21963 - 0.19751673498613801407e-1 * t21827 - 0.310907e-1 * t21975 * t436 + t21990;
    (t21990, t21991)
}
