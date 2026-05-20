//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2129/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2129<F: Float>(t1041: F, t4589: F, t49850: F, t10969: F, t41687: F, t10868: F, t248: F, t4347: F, t10224: F, t4343: F, t973: F, t3130: F, t4595: F) -> (F, F, F, F, F) {
    let t49852 = t1041 * t49850 * t4589;
    let t49853 = F::new(5.0) / F::new(20736.0) * t49852;
    let t49854 = t10969 * t41687;
    let t49871 = t1041 * t248 * t10868 * t4347;
    let t49872 = t49871 / F::new(6912.0);
    let t49906 = t973 * t10224 * t4343;
    let t49907 = t49906 / F::new(216.0);
    let t49922 = t3130 * t49850 * t4595;
    (t49853, t49854, t49872, t49907, t49922)
}
