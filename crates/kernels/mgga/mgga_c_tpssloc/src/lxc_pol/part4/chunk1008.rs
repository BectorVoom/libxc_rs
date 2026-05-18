//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 1008/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk1008<F: Float>(t16781: F, t16803: F, t225: F, t10054: F, t5585: F, t13176: F, t1499: F, t1523: F, t1525: F, t16673: F, t16679: F, t16754: F, t16756: F, t16759: F, t16762: F, t255: F, t2617: F, t4162: F, t4166: F, t4286: F, t4291: F, t4296: F, t4298: F, t5645: F, t5648: F, t5653: F, t812: F, t861: F) -> (F, F, F) {
    let t16804 = t16781 + t16803;
    let t16805 = t16804 * t225;
    let t16811 = t10054 * t5585;
    let t16814 = -F::new(2.0) * t13176 * t1523 + F::new(2.0) * t1499 * t4298 + F::new(2.0) * t1525 * t4162 - t16673 * t861 - F::new(2.0) * t16679 * t812 - t16754 * t812 - t16756 * t812 - F::new(2.0) * t16759 * t4291 - F::new(2.0) * t16762 * t4291 + t16805 * t255 + F::new(2.0) * t16811 * t812 + F::new(2.0) * t2617 * t5645 - F::new(2.0) * t2617 * t5648 - t2617 * t5653 - F::new(2.0) * t4166 * t4286 - F::new(2.0) * t4166 * t4296;
    (t16804, t16805, t16814)
}
