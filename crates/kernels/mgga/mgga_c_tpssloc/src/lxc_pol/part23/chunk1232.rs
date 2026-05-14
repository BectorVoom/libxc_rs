//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1232/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1232<F: Float>(t13798: F, t17794: F, t17800: F, t17804: F, t17817: F, t17863: F, t2986: F, t42817: F, t4510: F, t4514: F, t4518: F, t4531: F, t48221: F, t61322: F, t69496: F, t69505: F, t69519: F, t69529: F, t69570: F, t69579: F, t76585: F, t76608: F, t76616: F, t76624: F) -> (F,) {
    let t76865 = -0.11111111111111111111e-2 * t69570 + 0.99999999999999999996e-2 * t2986 * t4518 * t76616 + 0.14814814814814814815e-2 * t2986 * t4510 * t76608 + 0.51851851851851851851e-2 * t2986 * t13798 * t76585 - 0.22222222222222222222e-2 * t2986 * t61322 * t17863 - 0.34567901234567901234e-2 * t2986 * t48221 * t69519 - 0.11111111111111111111e-2 * t2986 * t69496 * t4514 - 0.16666666666666666666e-2 * t2986 * t17800 * t17794 - 0.11111111111111111111e-2 * t2986 * t69505 * t4514 - 0.66666666666666666664e-2 * t2986 * t4531 * t69529 + 0.33333333333333333332e-2 * t2986 * t17804 * t17817 - t42817 - 0.11111111111111111111e-2 * t69579 - 0.22222222222222222221e-2 * t2986 * t4518 * t76624;
    (t76865,)
}
