//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1117/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1117<F: Float>(t153: F, t157: F, t39842: F, t10140: F, t10143: F, t2374: F, t39354: F, t193: F, t202: F, t2522: F, t39529: F, t40760: F, t40762: F, t40764: F, t40766: F, t40768: F, t40769: F, t40772: F, t40777: F, t40779: F, t40782: F, t776: F) -> (F, F, F) {
    let t40784 = t153 * t157 * t39842;
    let t40785 = t10140 * t10143;
    let t40790 = 0.21687162600603479684e-1 * t2374 * t39354;
    let t40791 = -6.0 * t193 * t202 * t40769 * t40772 + 24.0 * t2522 * t40785 * t776 - t39529 + t40760 - t40762 + t40764 + t40766 + t40768 + t40777 - t40779 + t40782 + t40784 + t40790;
    (t40784, t40790, t40791)
}
