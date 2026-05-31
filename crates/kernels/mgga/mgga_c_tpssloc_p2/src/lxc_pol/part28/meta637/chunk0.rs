//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2032/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2032<F: Float>(t531: F, t7939: F, t12550: F, t12557: F, t1442: F, t15857: F, t15904: F, t1983: F, t2036: F, t22574: F, t22584: F, t22596: F, t2314: F, t2363: F, t23938: F, t24176: F, t24428: F, t24432: F, t26161: F, t26558: F, t26905: F, t26977: F, t27219: F, t33899: F, t3929: F, t4073: F, t5107: F, t56120: F, t56194: F, t652: F, t7040: F, t7042: F, t7056: F, t7685: F, t7687: F, t7890: F, t7900: F, t84347: F, t90437: F) -> F {
    let t93966 = t531 * t7939;
    let t93978 = F::cast_from(3.0_f64) * t1983 * t26905 * t22584 - t1442 * t24428 - t2036 * t15857 - F::cast_from(2.0_f64) * t7042 * t12557 + t7900 * t3929 - F::cast_from(3.0_f64) * t22574 * t24432 * t56120 - F::cast_from(4.0_f64) * t23938 * t4073 - F::cast_from(4.0_f64) * t26977 * t4073 - F::cast_from(4.0_f64) * t7042 * t12550 + F::cast_from(2.0_f64) * t26161 * t26558 * t90437 - F::cast_from(2.0_f64) * t652 * t7890 * t2363 - F::cast_from(4.0_f64) * t652 * t5107 * t7056 - F::cast_from(4.0_f64) * t2314 * t27219 - F::cast_from(6.0_f64) * t22574 * t24432 * t56194 + F::cast_from(6.0_f64) * t7685 * t24176 + F::cast_from(6.0_f64) * t1983 * t93966 * t22596 + F::cast_from(3.0_f64) * t1983 * t84347 * t7687 - F::cast_from(2.0_f64) * t7040 * t5107 - F::cast_from(6.0_f64) * t22574 * t33899 * t15904;
    t93978
}
