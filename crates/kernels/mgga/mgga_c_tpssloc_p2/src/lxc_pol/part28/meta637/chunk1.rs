//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2033/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2033<F: Float>(t12734: F, t12813: F, t1458: F, t16148: F, t16153: F, t16503: F, t1983: F, t2040: F, t2075: F, t2079: F, t2314: F, t23909: F, t23958: F, t24028: F, t24428: F, t24987: F, t24995: F, t26114: F, t26179: F, t26559: F, t27150: F, t27226: F, t4028: F, t4034: F, t4072: F, t652: F, t7050: F, t7156: F, t7170: F, t7171: F, t7685: F, t7802: F, t90023: F, t9016: F, t90370: F, t91669: F, t91753: F) -> F {
    let t94022 = -F::new(4.0) * t90370 * t2040 - F::new(4.0) * t26114 * t7050 - F::new(2.0) * t652 * t24428 * t1458 + F::new(6.0) * t24987 * t7171 + F::new(3.0) * t1983 * t7170 * t90023 + F::new(6.0) * t7685 * t23958 + F::new(6.0) * t24995 * t9016 * t16153 + F::new(12.0) * t24995 * t9016 * t16148 + F::new(4.0) * t91669 * t26559 - F::new(4.0) * t4034 * t27150 - F::new(4.0) * t652 * t7156 * t4072 - F::new(2.0) * t4028 * t23909 - F::new(4.0) * t12734 * t7802 - F::new(4.0) * t2314 * t27226 + t2079 * t16503 - F::new(2.0) * t7685 * t24028 - F::new(2.0) * t652 * t2075 * t12813 - F::new(2.0) * t91753 * t2040 - F::new(4.0) * t26179 * t7050;
    t94022
}
