//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2215/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2215<F: Float>(t22986: F, t23270: F, t865: F, t98253: F, t1528: F, t2597: F, t28311: F, t866: F, t86951: F, t86968: F, t86988: F, t92432: F, t98234: F, t98237: F, t98239: F, t98248: F, t98251: F) -> F {
    let t98256 = t22986 * t23270 * t98253 * t865;
    let t98258 = F::cast_from(0.49348022005446793095e-1_f64) * t98234 - F::cast_from(0.24674011002723396548e-1_f64) * t98237 - F::new(2.0) * t98239 * t866 - F::new(6.0) * t2597 * t28311 + t86951 - F::new(2.0) * t86988 * t1528 + t92432 + F::cast_from(0.3289868133696452873e-1_f64) * t98248 - t86968 + F::cast_from(0.3289868133696452873e-1_f64) * t98251 + F::cast_from(0.16449340668482264365e-1_f64) * t98256;
    t98258
}
