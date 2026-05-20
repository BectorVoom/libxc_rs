//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2139/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2139<F: Float>(t10199: F, t2970: F, t973: F, t10203: F, t10254: F, t10913: F, t697: F, t976: F, t984: F, t2986: F, t2990: F, t10189: F, t3008: F) -> (F, F, F, F, F, F, F) {
    let t43028 = t973 * t2970 * t10199;
    let t43038 = t973 * t2970 * t10203;
    let t43043 = t10254 * t10913;
    let t43052 = t697 * t976;
    let t43053 = t43052 * t984;
    let t43055 = t2986 * t43053 * t2990;
    let t43057 = t10189 * t3008;
    (t43028, t43038, t43043, t43052, t43053, t43055, t43057)
}
