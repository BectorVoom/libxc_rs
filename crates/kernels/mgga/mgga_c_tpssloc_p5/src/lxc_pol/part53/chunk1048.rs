//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1048/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1048<F: Float>(t102344: F, t117084: F, t121210: F, t1388: F, t1393: F, t2036: F, t2040: F, t2075: F, t2096: F, t22574: F, t2314: F, t24995: F, t25988: F, t26161: F, t26558: F, t26870: F, t26967: F, t27188: F, t32235: F, t33363: F, t33793: F, t33857: F, t33928: F, t38018: F, t4034: F, t4037: F, t4072: F, t4077: F, t5308: F, t652: F, t6876: F, t7040: F, t7050: F, t7056: F, t7156: F, t7220: F, t7801: F, t7890: F, t7939: F, t8774: F) -> F {
    let t124472 = -F::cast_from(2.0_f64) * t26967 * t2075 - F::cast_from(3.0_f64) * t22574 * t117084 * t25988 - F::cast_from(2.0_f64) * t7040 * t7890 - F::cast_from(2.0_f64) * t2036 * t26870 - F::cast_from(6.0_f64) * t24995 * t38018 * t5308 - t6876 * t33793 + t33928 * t1393 - F::cast_from(2.0_f64) * t32235 * t4037 + F::cast_from(4.0_f64) * t26161 * t26558 * t7939 * t1388 - F::cast_from(4.0_f64) * t2314 * t33857 - F::cast_from(4.0_f64) * t4034 * t33857 - F::cast_from(4.0_f64) * t652 * t7156 * t7801 - F::cast_from(2.0_f64) * t32235 * t4077 - F::cast_from(2.0_f64) * t652 * t8774 * t4072 + F::cast_from(2.0_f64) * t121210 * t2096 - F::cast_from(2.0_f64) * t33363 * t7220 - F::cast_from(4.0_f64) * t652 * t7890 * t7056 - F::cast_from(4.0_f64) * t102344 * t2040 - F::cast_from(4.0_f64) * t27188 * t7050;
    t124472
}
