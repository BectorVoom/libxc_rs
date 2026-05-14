//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1091/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1091<F: Float>(t10471: F, t47840: F, t10479: F, t10375: F, t1612: F, t1041: F, t1539: F, t248: F, t42749: F, t10523: F, t1573: F, t10629: F, t10701: F, t1543: F, t10810: F, t1561: F) -> (F, F, F, F, F, F, F, F) {
    let t48569 = t47840 * t10471;
    let t48570 = t48569 * t10479;
    let t48670 = t1612 * t10375;
    let t48674 = t1041 * t248 * t42749 * t1539;
    let t49099 = t1573 * t10523;
    let t49104 = t1573 * t10629;
    let t49274 = t1543 * t10701;
    let t49285 = t1561 * t10810;
    (t48569, t48570, t48670, t48674, t49099, t49104, t49274, t49285)
}
