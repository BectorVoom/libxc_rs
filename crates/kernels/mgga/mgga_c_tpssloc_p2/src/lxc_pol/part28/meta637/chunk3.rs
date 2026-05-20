//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2035/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2035<F: Float>(t12725: F, t12734: F, t1983: F, t2040: F, t2096: F, t22574: F, t22578: F, t22607: F, t2314: F, t23953: F, t24175: F, t24432: F, t24442: F, t24990: F, t24995: F, t26558: F, t26878: F, t26898: F, t27163: F, t3652: F, t4028: F, t45632: F, t5361: F, t55934: F, t652: F, t6876: F, t7050: F, t7166: F, t7685: F, t7796: F, t7801: F, t7806: F, t7940: F, t7941: F, t86672: F, t91565: F, t91603: F, t91695: F, t9348: F) -> F {
    let t94103 = F::new(6.0) * t1983 * t24175 * t24990 + t86672 * t2096 - F::new(2.0) * t4028 * t24442 - F::new(2.0) * t9348 * t7806 - F::new(2.0) * t652 * t3652 * t7801 - t1983 * t7940 * t22578 - F::new(2.0) * t45632 * t2040 - F::new(4.0) * t12734 * t7796 - F::new(4.0) * t2314 * t27163 - F::new(6.0) * t24995 * t24432 * t91695 + F::new(12.0) * t22574 * t26558 * t91565 - F::new(4.0) * t55934 * t2040 - F::new(4.0) * t12725 * t7050 + F::new(2.0) * t7166 * t5361 - F::new(3.0) * t22574 * t24432 * t91603 + t22607 * t7941 + F::new(6.0) * t6876 * t26898 + F::new(3.0) * t7685 * t23953 - F::new(2.0) * t6876 * t26878;
    t94103
}
