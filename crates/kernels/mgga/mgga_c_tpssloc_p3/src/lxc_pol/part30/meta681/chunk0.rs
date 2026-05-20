//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2138/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2138<F: Float>(t1351: F, t1992: F, t550: F, t6434: F, t6976: F, t22704: F, t22705: F, t28167: F, t26331: F, t26421: F, t26446: F, t5187: F) -> (F, F, F) {
    let t96986 = t1992 * t6976 * t6434 * t1351 * t550;
    let t96989 = t22704 * t22705 * t28167;
    let t96993 = t26331 * t26446 * t26421 * t5187;
    (t96986, t96989, t96993)
}
