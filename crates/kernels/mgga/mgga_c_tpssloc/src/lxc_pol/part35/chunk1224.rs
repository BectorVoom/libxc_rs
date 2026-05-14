//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1224/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1224<F: Float>(t105308: F, t105350: F, t105383: F, t105417: F, t6552: F, t7479: F, t98133: F, t1880: F, t21013: F, t214: F, t225: F, t258: F, t1888: F, t23270: F, t25044: F, t5657: F) -> (F, F, F, F) {
    let t105419 = t105308 + t105350 + t105383 + t105417;
    let t105423 = t6552 * t98133 * t7479;
    let t105428 = t1880 * t214 * t21013 * t225 * t258;
    let t105437 = t1888 * t23270 * t25044 * t5657;
    (t105419, t105423, t105428, t105437)
}
