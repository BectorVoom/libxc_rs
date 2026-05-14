//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1116/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1116<F: Float>(t39537: F, t761: F, t2531: F, t9494: F, t39344: F, t39362: F, t2427: F, t9868: F, t2749: F, t2751: F, t12908: F, t9682: F, t39494: F, t152: F, t185: F, t39097: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t40760 = 0.12304822629859687989e5 * t761 * t39537;
    let t40761 = t2531 * t9494;
    let t40762 = 0.4101607543286562663e4 * t40761;
    let t40764 = 0.46785788981077169656e1 * t761 * t39344;
    let t40766 = 0.62337092780453269531e3 * t761 * t39362;
    let t40767 = t2427 * t9868;
    let t40768 = 48.0 * t40767;
    let t40769 = t2749 * t2749;
    let t40771 = t2751 * t2751;
    let t40772 = 1.0 / t40771;
    let t40777 = 144.0 * t12908 * t9682;
    let t40779 = 0.51947577317044391277e2 * t761 * t39494;
    let t40782 = 24.0 * t39097 * t152 * t185;
    (t40760, t40762, t40764, t40766, t40768, t40769, t40772, t40777, t40779, t40782)
}
