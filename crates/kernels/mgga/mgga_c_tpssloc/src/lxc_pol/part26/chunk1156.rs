//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1156/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1156<F: Float>(t23146: F, t9653: F, t23145: F, t2617: F, t2649: F, t6605: F, t815: F, t9958: F, t23109: F, t23110: F, t232: F, t236: F, t2678: F, t23102: F, t80782: F, t23113: F) -> (F, F, F, F, F) {
    let t81863 = t23146 * t9653;
    let t81865 = t2617 * t23145;
    let t81866 = t81865 * t2649;
    let t81869 = t6605 * t815 * t9958;
    let t81874 = t23109 * t23110 * t236 * t2678 * t232;
    let t81876 = t23102 * t80782;
    let t81877 = t81876 * t23113;
    (t81863, t81866, t81869, t81874, t81877)
}
