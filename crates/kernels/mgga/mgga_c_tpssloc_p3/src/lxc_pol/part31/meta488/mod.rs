//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta488 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1666;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1667;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1668;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta488<F: Float>(t24175: F, t7687: F, t6999: F, t7940: F, t532: F, t7939: F, t6879: F, t12571: F, t7025: F, t23967: F, t7432: F, t7032: F, t7435: F, t2032: F, t23975: F, t26055: F, t26063: F, t26067: F, t26070: F, t26073: F, t26076: F, t26090: F, t6492: F, t6495: F, t7026: F, t7035: F, t7782: F, t2031: F, t26024: F, t7428: F, t26012: F, t7031: F, t7445: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t26898, t26902, t26905, t26906, t26911) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1666::<F>(t24175, t7687, t6999, t7940, t532, t7939, t6879, t12571, t7025);
        let (t26920, t26936, t26938) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1667::<F>(t23967, t7432, t7032, t7435, t2032, t23975, t26055, t26063, t26067, t26070, t26073, t26076, t26090, t26911, t6492, t6495, t7026, t7035, t7782);
        let (t26945, t26948, t26954, t26959) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1668::<F>(t2031, t26024, t7032, t7428, t26012, t7031, t7445);
    (t26898, t26902, t26905, t26906, t26911, t26920, t26936, t26938, t26945, t26948, t26954, t26959)
}
