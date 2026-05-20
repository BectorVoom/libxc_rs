//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta483 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1823;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1824;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta483<F: Float>(t1184: F, t52: F, t460: F, t24682: F, t3548: F, t7310: F, t3469: F, t7320: F, t2134: F, t24650: F, t24655: F, t24659: F, t24664: F, t24670: F, t24675: F, t24677: F, t24681: F, t3552: F, t3557: F, t3562: F, t3587: F, t488: F, t7316: F, t7321: F, t7326: F, t7331: F, t7345: F, t2127: F, t3545: F, t3475: F, t2132: F, t607: F, t2136: F, t3535: F, t7338: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t24683, t24684, t24685, t24690, t24698, t24699, t24702) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1823::<F>(t1184, t52, t460, t24682, t3548, t7310, t3469, t7320, t2134, t24650, t24655, t24659, t24664, t24670, t24675, t24677, t24681, t3552, t3557, t3562, t3587, t488, t7316, t7321, t7326, t7331, t7345);
        let (t24704, t24705, t24706, t24712, t24716) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1824::<F>(t2127, t3545, t3475, t460, t7320, t2132, t607, t2136, t3535, t7338);
    (t24683, t24684, t24685, t24690, t24698, t24699, t24702, t24704, t24705, t24706, t24712, t24716)
}
