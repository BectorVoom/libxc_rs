//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2355/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2355<F: Float>(t1396: F, t1398: F, t1404: F, t1852: F, t1858: F, t24955: F, t24977: F, t27908: F, t27930: F, t85403: F, t85407: F, t85412: F, t86557: F, t86559: F, t96300: F, t96303: F, t96308: F, t96327: F, t96337: F) -> F {
    let t96340 = t1852 * t24977 + t96300 + t86557 + t85412 + F::cast_from(2.0_f64) * t86559 + t96303 + t24955 * t1858 + t85407 + F::cast_from(2.0_f64) * t27908 * t1404 + t96308 + t85403 + F::cast_from(2.0_f64) * t1396 * t27930 + t1398 * (t96327 + t96337);
    t96340
}
