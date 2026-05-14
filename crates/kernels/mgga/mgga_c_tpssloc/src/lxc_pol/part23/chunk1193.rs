//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1193/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1193<F: Float>(t39408: F, t39411: F, t39463: F, t39468: F, t39472: F, t39476: F, t40714: F, t40716: F, t40721: F, t75864: F, t75865: F, t39483: F, t40732: F, t40741: F, t40743: F, t40748: F, t40760: F, t75872: F, t75874: F, t75884: F, t75885: F, t75886: F, t75887: F) -> (F, F) {
    let t76009 = t39408 + t39411 - t40714 + t40716 + t75864 - t75865 + t39463 - t39468 - t40721 - t39472 - t39476;
    let t76010 = -t40732 + t75872 + t75874 + t39483 + t75884 - t75885 + t75886 - t40741 - t40743 + t40748 + t40760 + t75887;
    (t76009, t76010)
}
