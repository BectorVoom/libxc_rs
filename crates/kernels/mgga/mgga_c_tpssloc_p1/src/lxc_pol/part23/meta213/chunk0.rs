//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 857/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk857<F: Float>(t13123: F, t2375: F, t1512: F, t9671: F, t2644: F, t820: F, t1509: F, t2632: F, t1500: F, t2693: F, t2642: F, t4166: F) -> (F, F, F, F, F, F) {
    let t13124 = t13123 * t2375;
    let t13182 = t9671 * t1512;
    let t13222 = t2644 * t820;
    let t13228 = t1509 * t2632;
    let t13234 = t1500 * t2693;
    let t13251 = t4166 * t2642;
    (t13124, t13182, t13222, t13228, t13234, t13251)
}
