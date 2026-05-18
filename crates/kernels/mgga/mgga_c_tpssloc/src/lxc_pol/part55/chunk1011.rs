//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1011/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1011<F: Float>(t27602: F, t27648: F, t27679: F, t27719: F, t493: F, t1734: F, t7348: F, t1246: F, t24574: F, t8070: F, t2147: F, t5052: F) -> (F, F, F, F, F, F) {
    let t27721 = t27602 + t27648 + t27679 + t27719;
    let t27722 = t493 * t27721;
    let t27724 = t7348 * t1734;
    let t27725 = t27724 * t1246;
    let t27728 = t24574 * t8070;
    let t27732 = t2147 * t5052;
    (t27721, t27722, t27724, t27725, t27728, t27732)
}
