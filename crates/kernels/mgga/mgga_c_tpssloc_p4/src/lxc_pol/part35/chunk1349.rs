//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1349/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1349<F: Float>(t1858: F, t8110: F, t29865: F, t580: F, t2169: F, t6483: F, t29884: F, t576: F, t1390: F, t20416: F, t1983: F, t6878: F) -> (F, F, F, F, F) {
    let t105144 = t8110 * t1858;
    let t105146 = t29865 * t580;
    let t105147 = t2169 * t6483;
    let t105150 = t576 * t29884;
    let t105159 = t1390 * t20416;
    let t105162 = F::new(3.0) * t1983 * t6878 * t105159;
    (t105144, t105146, t105147, t105150, t105162)
}
