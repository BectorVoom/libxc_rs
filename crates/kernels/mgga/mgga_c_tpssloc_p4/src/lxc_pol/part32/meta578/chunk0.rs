//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1956/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1956<F: Float>(t29560: F, t1932: F, t2133: F, t7573: F, t8027: F, t1737: F, t2136: F, t24681: F, t24704: F, t27578: F, t27592: F, t27599: F, t27609: F, t27614: F, t6203: F, t6211: F, t7345: F) -> (F, F, F, F, F) {
    let t29561 = F::new(1.0) / t29560;
    let t29562 = t29561 * t1932;
    let t29563 = t29562 * t2133;
    let t29569 = t8027 * t7573;
    let t29580 = -t24681 + F::cast_from(0.72670960969452703541e-2_f64) * t29563 * t2136 + t27578 / F::new(1152.0) - t7345 * t6211 / F::new(1152.0) + F::cast_from(0.16149102437656156342e-2_f64) * t29569 * t2136 - t24704 - t27592 / F::new(216.0) - F::cast_from(0.20186378047070195428e-3_f64) * t27609 - t27599 * t1737 / F::new(144.0) + t27614 * t1737 / F::new(768.0) + F::new(5.0) / F::new(6912.0) * t7345 * t6203;
    (t29561, t29562, t29563, t29569, t29580)
}
