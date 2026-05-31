//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 889/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk889<F: Float>(t1352: F, t32136: F, t31153: F, t31160: F, t31177: F, t31157: F, t31163: F, t31166: F, t31173: F, t31179: F, t553: F, t1332: F, t1336: F, t31621: F, t31629: F, t31633: F, t32130: F, t32132: F, t544: F, t8798: F) -> (F, F, F, F, F, F, F) {
    let t32137 = t32136 * t1352;
    let t32139 = F::cast_from(0.22608743412718618877e-1_f64) * t31153;
    let t32141 = F::cast_from(0.5383034145885385447e-3_f64) * t31160;
    let t32145 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t31177;
    let t32147 = -t32139 - F::cast_from(0.19378922925187387609e-1_f64) * t31157 - t32141 - F::cast_from(0.32298204875312312682e-2_f64) * t31163 + t31166 / F::cast_from(384.0_f64) - t31173 / F::cast_from(384.0_f64) - t32145 - t31179 / F::cast_from(96.0_f64);
    let t32148 = t553 * t32147;
    let t32150 = -t32130 - F::cast_from(0.6579736267392905746e-1_f64) * t31621 - t32132 - F::cast_from(0.3289868133696452873e-1_f64) * t31629 + F::cast_from(0.3289868133696452873e-1_f64) * t31633 + t1332 * t8798 - t1336 * t32137 + t544 * t32148;
    (t32137, t32139, t32141, t32145, t32147, t32148, t32150)
}
