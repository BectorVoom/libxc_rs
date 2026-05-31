//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2079/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2079<F: Float>(t39537: F, t761: F, t2531: F, t9494: F, t39344: F, t39362: F, t2427: F, t9868: F, t2751: F, t39494: F, t153: F, t157: F, t39842: F) -> (F, F, F, F, F, F, F, F) {
    let t40760 = F::cast_from(0.12304822629859687989e5_f64) * t761 * t39537;
    let t40761 = t2531 * t9494;
    let t40764 = F::cast_from(0.46785788981077169656e1_f64) * t761 * t39344;
    let t40766 = F::cast_from(0.62337092780453269531e3_f64) * t761 * t39362;
    let t40767 = t2427 * t9868;
    let t40771 = t2751 * t2751;
    let t40772 = F::cast_from(1.0_f64) / t40771;
    let t40779 = F::cast_from(0.51947577317044391277e2_f64) * t761 * t39494;
    let t40784 = t153 * t157 * t39842;
    (t40760, t40761, t40764, t40766, t40767, t40772, t40779, t40784)
}
