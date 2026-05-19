//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 517/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk517<F: Float>(t5: F, t2303: F, t72: F, t2245: F, t2252: F, t2255: F, t2284: F, t609: F, t629: F, t642: F, t66: F, t80: F, t2233: F, t2235: F, t2240: F, t2241: F, t605: F, t645: F, t86: F) -> (F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t2304 = t72 * t2303;
    let t2307 = -t2245 * t80 / F::new(12.0) - t2252 * t80 / F::new(12.0) - t2255 * t80 / F::new(6.0) - t609 * t642 / F::new(6.0) + t2284 * t80 / F::new(24.0) + t629 * t642 / F::new(12.0) + t66 * t2304 / F::new(24.0);
    let t2311 = piecewise3::<F>(t8, F::new(0.0), t2233 * t86 - F::new(8.0) * t2235 * t645 + F::new(20.0) * t2240 * t2241 - F::new(4.0) * t2307 * t605);
    (t2304, t2307, t2311)
}
