//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 806/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk806<F: Float>(t36796: F, t36801: F, t8188: F, t942: F, t36942: F, t290: F, t8291: F, t36983: F, t37017: F, t7922: F, t7928: F, t2019: F, t2323: F, t7926: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t38079 = F::cast_from(0.2439011983326002265e-2_f64) * t36796;
    let t38080 = F::cast_from(0.11709622077411463733e-2_f64) * t36801;
    let t38107 = t942 * t8188;
    let t38123 = F::cast_from(0.26021382394247697185e-3_f64) * t36942;
    let t38125 = t290 * t8291;
    let t38140 = F::cast_from(0.13911401682674235141e-1_f64) * t36983;
    let t38149 = F::cast_from(0.28691693261408173224e-3_f64) * t37017;
    let t38172 = F::cast_from(0.19863479950205658386e-3_f64) * t7922;
    let t38174 = F::cast_from(0.487802396665200453e-2_f64) * t7928;
    let t38312 = t2019 * t7926 * t2323;
    (t38079, t38080, t38107, t38123, t38125, t38140, t38149, t38172, t38174, t38312)
}
