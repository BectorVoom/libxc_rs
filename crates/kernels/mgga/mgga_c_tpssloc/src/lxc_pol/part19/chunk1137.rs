//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1137/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1137<F: Float>(t39483: F, t40727: F, t40730: F, t40732: F, t40734: F, t40737: F, t40739: F, t40741: F, t40743: F, t40746: F, t40748: F, t40750: F, t39529: F, t40755: F, t40760: F, t40762: F, t40764: F, t40766: F, t40768: F, t40777: F, t40779: F, t40782: F, t40784: F) -> (F, F) {
    let t41245 = t40727 + t40730 - t40732 - t40734 + t40737 + t39483 - t40739 - t40741 - t40743 + t40746 + t40748 + t40750;
    let t41248 = t40755 + t40760 - t40762 + t40764 + t40766 + t40768 + t40777 - t39529 - t40779 + t40782 + t40784;
    (t41245, t41248)
}
