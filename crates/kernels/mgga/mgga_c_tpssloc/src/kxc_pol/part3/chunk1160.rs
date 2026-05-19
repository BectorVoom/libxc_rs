//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 1160/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk1160<F: Float>(t1128: F, t4794: F, t1675: F, t3356: F, t1136: F, t4820: F, t1683: F, t3351: F, t3333: F, t4823: F, t1138: F, t11410: F, t11420: F, t14864: F, t14866: F, t14916: F, t14934: F, t14939: F, t3327: F, t3332: F, t3352: F, t3360: F, t4797: F) -> F {
    let t15141 = t4794 * t1128;
    let t15146 = t1675 * t3356;
    let t15153 = t4820 * t1136;
    let t15156 = t1683 * t3351;
    let t15159 = t4823 * t3333;
    let t15162 = -F::cast_from(0.19751673498613801407e-1_f64) * t14934 - t14864 - t14866 - t14916 + F::new(2.0) * t15141 * t1138 + F::new(1.0) * t4797 * t3352 + F::cast_from(0.32163958997385070134e2_f64) * t15146 * t3360 + F::new(1.0) * t11410 * t1683 + F::new(2.0) * t3327 * t4820 - t14939 - F::new(4.0) * t3332 * t15153 - F::new(2.0) * t3332 * t15156 - F::cast_from(0.19298375398431042081e3_f64) * t11420 * t15159;
    t15162
}
