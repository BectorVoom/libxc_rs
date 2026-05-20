//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2005/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2005<F: Float>(t113: F, t12725: F, t12823: F, t1393: F, t1459: F, t1774: F, t1849: F, t1983: F, t2094: F, t22574: F, t23941: F, t24026: F, t24166: F, t24167: F, t24432: F, t24987: F, t26870: F, t26880: F, t26974: F, t27144: F, t27163: F, t27215: F, t3734: F, t4026: F, t4034: F, t510: F, t5161: F, t56198: F, t650: F, t6876: F, t6999: F, t7061: F, t7156: F, t7218: F, t7685: F, t7687: F, t7796: F, t83886: F, t84097: F, t92073: F, t93113: F, t93261: F) -> F {
    let t93275 = -t1983 * t24166 * t5161 - F::new(2.0) * t23941 * t1774 - t92073 * t510 + F::new(2.0) * t27215 * t1393 + t24026 * t1849 - F::new(2.0) * t650 * t26870 - F::new(4.0) * t12725 * t7061 - F::new(2.0) * t12823 * t7796 - F::new(4.0) * t4034 * t27163 + F::new(2.0) * t24987 * t7218 + t7685 * t24167 + F::new(6.0) * t1983 * t3734 * t2094 * t7687 - F::new(6.0) * t22574 * t24432 * t56198 - t113 * (t93113 + t93261) - F::new(6.0) * t83886 * t26974 - F::new(2.0) * t1983 * t27144 * t6999 - F::new(2.0) * t84097 * t1459 - F::new(2.0) * t6876 * t26880 - F::new(2.0) * t4026 * t7156;
    t93275
}
