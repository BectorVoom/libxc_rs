//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 520/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk520<F: Float>(t2187: F, t2190: F, t2193: F, t2197: F, t2199: F, t2202: F, t676: F, t657: F) -> (F, F, F) {
    let t2299 = -F::new(0.42198333333333333333e0) * t2187 + F::new(0.84396666666666666666e0) * t2190 + F::new(0.39862222222222222223e0) * t2193 + F::new(0.68258333333333333333e-1) * t2197 + F::new(0.13651666666666666667e0) * t2199 + F::new(0.13692777777777777778e0) * t2202;
    let t2300 = t2299 * t676;
    let t2302 = F::new(1.0) * t657 * t2300;
    (t2299, t2300, t2302)
}
