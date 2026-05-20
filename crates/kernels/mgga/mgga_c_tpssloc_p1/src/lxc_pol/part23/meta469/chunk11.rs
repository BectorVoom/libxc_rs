//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1397/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1397<F: Float>(t1070: F, t193: F, t336: F, t43637: F, t76668: F, t76671: F, t76674: F, t76675: F, t76715: F, t76997: F, t77001: F, t77003: F, t77006: F, t77009: F, t77012: F, t77014: F, t77016: F, t77913: F) -> F {
    let t77918 = t76668 - t76671 + t76674 - F::new(6.0) * t193 * t336 * t76675 * t43637 + t193 * t336 * (t76715 + t77913) * t1070 - t76997 + t77001 + t77003 + t77006 + t77009 - t77012 - t77014 - t77016;
    t77918
}
