//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1271/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1271<F: Float>(t291: F, t41677: F, t41719: F, t10603: F, t2929: F, t4497: F, t959: F, t10713: F, t2940: F, t2904: F, t952: F, t2924: F) -> (F, F, F, F, F) {
    let t41722 = F::cast_from(0.621814e-1_f64) * (t41677 + t41719) * t291;
    let t41726 = F::cast_from(0.69263436422725855036e2_f64) * t959 * t2929 * t10603 * t4497;
    let t41728 = F::cast_from(0.14035736694323150897e2_f64) * t2940 * t10713;
    let t41732 = F::cast_from(0.46785788981077169656e1_f64) * t959 * t2904 * t10603 * t952;
    let t41733 = t2924 * t2924;
    (t41722, t41726, t41728, t41732, t41733)
}
