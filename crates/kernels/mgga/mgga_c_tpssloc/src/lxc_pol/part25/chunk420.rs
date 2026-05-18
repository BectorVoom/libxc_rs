//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 420/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk420<F: Float>(t5: F, t2233: F, t2235: F, t2240: F, t2241: F, t2307: F, t605: F, t645: F, t86: F, t112: F) -> (F, F) {
    let t7 = piecewise3::<f64>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::new(0.999999999999e0);
    let t2311 = piecewise3::<f64>(t8, F::new(0.0), t2233 * t86 - F::new(8.0) * t2235 * t645 + F::new(20.0) * t2240 * t2241 - F::new(4.0) * t2307 * t605);
    let t2312 = t2311 * t112;
    (t2311, t2312)
}
