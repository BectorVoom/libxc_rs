//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2697/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2697<F: Float>(t40041: F, t544: F, t68: F, t1332: F, t16046: F, t1352: F, t3850: F, t12169: F, t12178: F, t12259: F, t12273: F, t12435: F, t1336: F, t16033: F, t16068: F, t16132: F, t16433: F, t1814: F, t1838: F, t19810: F, t3777: F, t3851: F, t3856: F, t40118: F, t5234: F, t5287: F, t5335: F, t5344: F, t5348: F) -> (F, F, F) {
    let t54963 = t544 * t68 * t40041;
    let t54976 = t1332 * t16046;
    let t55003 = t1352 * t3850;
    let t55012 = -t12178 * t1336 * t5348 - F::new(3.0) * t12259 * t1336 * t5287 - F::new(3.0) * t1336 * t16132 * t3851 - F::new(3.0) * t1336 * t16132 * t3856 - F::new(3.0) * t5335 * t5344 * t55003 - t12169 * t5234 - F::new(3.0) * t12273 * t19810 + t12435 * t1814 - F::new(6.0) * t16033 * t16068 - F::new(3.0) * t16433 * t3777 - t1838 * t40118;
    (t54963, t54976, t55012)
}
