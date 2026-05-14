//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1233/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1233<F: Float>(t41961: F, t41845: F, t41863: F, t41865: F, t41868: F, t41870: F, t41872: F, t41874: F, t41876: F, t41882: F, t41885: F, t41973: F, t3008: F, t10199: F, t2970: F, t973: F) -> (F, F, F) {
    let t43002 = 220.0 / 81.0 * t41961;
    let t43012 = -t41845 - 4.0 / 3.0 * t41973 - t43002 - 160.0 / 81.0 * t41863 + 8.0 / 9.0 * t41865 - 8.0 / 9.0 * t41868 + 10.0 / 9.0 * t41870 + 10.0 / 27.0 * t41872 - 4.0 / 9.0 * t41874 - 16.0 / 81.0 * t41876 + 14.0 / 81.0 * t41882 + t41885 / 6.0;
    let t43019 = t3008 * t3008;
    let t43028 = t973 * t2970 * t10199;
    (t43012, t43019, t43028)
}
