//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1987/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1987<F: Float>(t16465: F, t225: F, t12250: F, t1824: F, t1799: F, t3791: F, t3850: F, t16028: F, t1372: F, t5286: F, t3879: F, t16205: F, t562: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t53866 = t16465 * t225;
    let t54014 = t1824 * t12250;
    let t54068 = t1799 * t3791;
    let t54153 = t1824 * t3850;
    let t54165 = t1799 * t3850;
    let t54258 = t1824 * t3791;
    let t54825 = t16028 * t225;
    let t54840 = t1372 * t5286;
    let t54854 = t3879 * t1824;
    let t54883 = t562 * t16205;
    (t53866, t54014, t54068, t54153, t54165, t54258, t54825, t54840, t54854, t54883)
}
