//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2474/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2474<F: Float>(t11091: F, t1637: F, t43637: F, t4700: F, t49082: F, t49084: F, t49086: F, t49088: F, t49090: F, t49092: F, t49095: F, t49535: F, t49538: F, t49540: F) -> F {
    let t50771 = -F::new(6.0) * t11091 * t1637 * t43637 * t4700 - t49082 + t49084 - t49086 + t49088 - t49090 + t49092 - t49095 + t49535 + t49538 - t49540;
    t50771
}
