//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1208/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1208<F: Float>(t27700: F, t95588: F, t18975: F, t7345: F, t18332: F, t7310: F, t1222: F, t29606: F, t29787: F, t85639: F, t1170: F, t2121: F, t29726: F, t24574: F, t29557: F, t29551: F) -> (F, F, F, F, F, F, F, F) {
    let t104425 = t95588 * t27700;
    let t104435 = t7345 * t18975;
    let t104441 = t7310 * t18332;
    let t104445 = t29606 * t1222;
    let t104469 = t85639 * t29787;
    let t104480 = t2121 * t1170 * t29726;
    let t104502 = t24574 * t29557;
    let t104504 = t24574 * t29551;
    (t104425, t104435, t104441, t104445, t104469, t104480, t104502, t104504)
}
