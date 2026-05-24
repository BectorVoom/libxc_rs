//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1061/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1061<F: Float>(t39785: F, t39796: F, t39800: F, t39808: F, t39752: F, t39754: F, t39756: F, t39758: F, t39760: F, t39764: F, t39771: F, t39773: F, t39777: F, t39781: F, t39789: F, t39792: F, t39804: F, t5019: F, t9402: F) -> F {
    let t43135 = F::cast_from(0.60975299583150056624e-3_f64) * t39785;
    let t43138 = F::cast_from(0.60975299583150056624e-3_f64) * t39796;
    let t43139 = F::cast_from(0.60975299583150056624e-3_f64) * t39800;
    let t43141 = F::cast_from(0.86737941314158990616e-4_f64) * t39808;
    let t43144 = -F::cast_from(0.15323255961587222184e-3_f64) * t39752 - F::cast_from(0.1702583995731913576e-4_f64) * t39754 - F::cast_from(0.1064114997332445985e-4_f64) * t39756 - F::cast_from(0.212822999466489197e-4_f64) * t39758 + F::cast_from(0.5107751987195740728e-4_f64) * t39760 + F::cast_from(0.5107751987195740728e-4_f64) * t39764 + F::cast_from(0.2553875993597870364e-4_f64) * t39771 + F::cast_from(0.3405167991463827152e-4_f64) * t39773 + F::cast_from(0.3405167991463827152e-4_f64) * t39777 + F::cast_from(0.1702583995731913576e-4_f64) * t39781 - t43135 - F::cast_from(0.30487649791575028312e-3_f64) * t39789 - F::cast_from(0.39032073591371545778e-3_f64) * t39792 - t43138 - t43139 - F::cast_from(0.30487649791575028312e-3_f64) * t39804 + t43141 - F::cast_from(0.47896966807455234256e0_f64) * t5019 * t9402;
    t43144
}
