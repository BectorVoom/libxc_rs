//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1221/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1221<F: Float>(t105531: F, t105543: F, t105547: F, t105551: F, t105561: F, t105565: F, t20857: F, t28997: F, t29010: F, t4166: F, t812: F, t84953: F, t87068: F, t87080: F, t98330: F, t98342: F, t98345: F, t98356: F, t98363: F, t98374: F, t98380: F) -> F {
    let t108164 = -F::cast_from(0.3289868133696452873e-1_f64) * t105531 - F::cast_from(0.69087230807625510332e0_f64) * t98330 - F::cast_from(0.49348022005446793095e-1_f64) * t105543 - F::cast_from(0.29608813203268075857e0_f64) * t105547 - F::cast_from(0.19739208802178717238e0_f64) * t105551 - F::cast_from(0.15626873635058151147e0_f64) * t87068 - F::cast_from(0.24674011002723396548e-1_f64) * t98342 + F::cast_from(0.9869604401089358619e-1_f64) * t98345 + F::cast_from(0.49348022005446793095e-1_f64) * t98356 + F::cast_from(0.16449340668482264365e-1_f64) * t105561 - F::new(6.0) * t812 * t84953 * t20857 - F::new(6.0) * t4166 * t28997 - F::cast_from(0.49348022005446793095e-1_f64) * t105565 - F::cast_from(0.49348022005446793095e-1_f64) * t98363 - F::cast_from(0.11514538467937585055e0_f64) * t98374 + F::cast_from(0.38381794893125283518e0_f64) * t87080 + F::cast_from(0.11514538467937585055e0_f64) * t98380 - F::new(3.0) * t4166 * t29010;
    t108164
}
