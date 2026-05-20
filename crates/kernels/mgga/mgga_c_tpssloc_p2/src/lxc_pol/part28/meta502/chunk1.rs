//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1737/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1737<F: Float>(t1393: F, t1459: F, t1849: F, t1983: F, t2040: F, t2079: F, t22574: F, t26114: F, t26898: F, t26902: F, t26906: F, t26967: F, t26969: F, t26974: F, t26977: F, t4037: F, t510: F, t5361: F, t650: F, t6876: F, t7042: F, t7166: F, t7218: F, t7685: F, t7890: F, t7900: F, t7941: F) -> F {
    let t26982 = t1393 * t7900 - F::new(2.0) * t1459 * t26977 + t1849 * t7166 + F::new(3.0) * t1983 * t26898 - t1983 * t26902 + F::new(3.0) * t1983 * t26906 + F::new(3.0) * t1983 * t26969 - F::new(2.0) * t2040 * t26114 + t2079 * t5361 - F::new(3.0) * t22574 * t26974 - t26967 * t510 - F::new(2.0) * t4037 * t7042 - t650 * t7890 + t6876 * t7941 + t7218 * t7685;
    t26982
}
