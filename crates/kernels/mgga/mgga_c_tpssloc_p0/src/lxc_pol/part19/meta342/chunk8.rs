//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1227/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1227<F: Float>(t39529: F, t40755: F, t40760: F, t40762: F, t40764: F, t40766: F, t40768: F, t40777: F, t40779: F, t40782: F, t40784: F, t39549: F, t40790: F, t40793: F, t40795: F, t40797: F, t40799: F, t40801: F, t40803: F, t40805: F, t40807: F, t40809: F, t40811: F) -> (F, F) {
    let t41248 = t40755 + t40760 - t40762 + t40764 + t40766 + t40768 + t40777 - t39529 - t40779 + t40782 + t40784;
    let t41249 = t40790 + t40793 + t40795 + t40797 + t40799 + t40801 - t40803 - t40805 + t40807 + t40809 + t40811 + t39549;
    (t41248, t41249)
}
