//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1216/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1216<F: Float>(t1897: F, t3537: F, t645: F, t6540: F, t19575: F, t19578: F, t19584: F, t19586: F, t19599: F, t19603: F, t19607: F, t19608: F, t19612: F, t19616: F, t19618: F, t3493: F, t5991: F, t626: F) -> (F, F, F) {
    let t20950 = t1897 * t3537;
    let t20953 = t6540 * t645;
    let t20956 = -2.0 * t20950 * t626 - 2.0 * t20953 * t626 - 2.0 * t3493 * t5991 - t19575 + t19578 + t19584 - t19586 - t19599 - t19603 + t19607 + t19608 - t19612 + t19616 + t19618;
    (t20950, t20953, t20956)
}
