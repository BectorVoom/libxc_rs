//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2342/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2342<F: Float>(t29827: F, t3640: F, t103164: F, t103213: F, t103258: F, t103279: F, t103303: F, t103341: F, t103377: F, t103415: F, t103457: F, t103488: F, t104508: F, t104534: F, t104564: F, t104596: F, t104631: F, t104669: F, t1254: F, t1256: F, t1763: F, t19262: F, t193: F, t24905: F, t24909: F, t27838: F, t27843: F, t336: F, t4700: F, t5091: F, t6270: F, t6274: F, t7398: F, t86517: F, t86524: F, t95921: F, t95925: F) -> F {
    let t104677 = t29827 * t3640;
    let t104708 = t193 * t336 * (t103164 + t103213 + t103258 + t103279 + t103303 + t103341 + t103377 + t103415 + t103457 + t103488 + t104508 + t104534 + t104564 + t104596 + t104631 + t104669) * t1256 - t4700 * t104677 * t1254 - F::cast_from(2.0_f64) * t4700 * t95921 * t1763 + F::cast_from(4.0_f64) * t4700 * t95925 * t27843 - F::cast_from(2.0_f64) * t4700 * t27838 * t5091 + F::cast_from(2.0_f64) * t4700 * t86517 * t6274 - F::cast_from(6.0_f64) * t4700 * t86524 * t6274 * t1254 + F::cast_from(4.0_f64) * t4700 * t24909 * t1763 * t5091 - t4700 * t24905 * t6270 + F::cast_from(2.0_f64) * t4700 * t24909 * t6270 * t1254 - t4700 * t7398 * t19262;
    t104708
}
