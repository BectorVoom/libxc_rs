//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 494/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk494<F: Float>(t4118: F, t1022: F, t4056: F, t4062: F, t4064: F, t4074: F, t4077: F, t4080: F, t4083: F, t4089: F, t4101: F, t4106: F, t4108: F, t4111: F, t4585: F, t5372: F, t5375: F, t5376: F, t5377: F, t5380: F, t5381: F, t5382: F) -> (F, F) {
    let t5383 = F::new(24.0) * t4118;
    let t5384 = -t4056 + t4062 + F::new(0.186546e0) * t1022 * t5372 - t4064 + t5375 + t5376 - t4074 + F::new(0.373092e0) * t4585 * t5377 - t4077 - t4080 + t4083 + t5380 + t4089 - t4101 + t4106 + t4108 + t4111 + t5381 - t5382 - t5383;
    (t5383, t5384)
}
