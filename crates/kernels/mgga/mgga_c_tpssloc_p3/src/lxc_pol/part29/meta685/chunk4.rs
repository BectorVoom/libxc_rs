//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2342/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2342<F: Float>(t26012: F, t7255: F, t22527: F, t22549: F, t24514: F, t24517: F, t24520: F, t26009: F, t26090: F, t27298: F, t27303: F, t27332: F, t6495: F, t83722: F, t83778: F, t85463: F, t85480: F, t85501: F, t85536: F, t90080: F, t90114: F, t90137: F, t90141: F) -> F {
    let t96102 = t7255 * t26012;
    let t96105 = F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t27332 * t22527 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6495 * t27303 + F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t24520 * t26090 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t90114 * t24517 + F::cast_from(10.0_f64) * t90137 * t85463 + F::cast_from(35.0_f64) * t85501 * t90141 - F::cast_from(10.0_f64) * t85536 * t26009 - F::cast_from(10.0_f64) * t85480 * t26009 - F::cast_from(5.0_f64) * t24514 * t90080 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t83722 * t27298 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t83778 * t27298 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t22549 * t96102;
    t96105
}
