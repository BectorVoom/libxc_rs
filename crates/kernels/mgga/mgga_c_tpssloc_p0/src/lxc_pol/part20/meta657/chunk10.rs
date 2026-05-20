//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2438/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2438<F: Float>(t1009: F, t13939: F, t1011: F, t1019: F, t1041: F, t10868: F, t248: F, t4347: F, t14134: F, t3117: F, t10863: F, t4571: F) -> (F, F, F, F, F) {
    let t49864 = t13939 * t1009;
    let t49866 = t49864 * t1011 * t1019;
    let t49871 = t1041 * t248 * t10868 * t4347;
    let t49872 = t49871 / F::new(6912.0);
    let t49873 = t3117 * t14134;
    let t49877 = t10863 * t4571;
    (t49864, t49866, t49872, t49873, t49877)
}
