//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 902/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk902<F: Float>(t2650: F, t958: F, t2192: F, t359: F, t361: F, t355: F, t215: F, t334: F, t68: F, t333: F, t979: F, t73: F) -> (F, F, F, F) {
    let t9033 = t958 * t2650;
    let t9036 = t359 * t2192 * t361;
    let t9038 = t355 * t9036 / F::new(10368.0);
    let t9040 = t215 * t68 * t334;
    let t9042 = F::new(5.0) / F::new(1296.0) * t333 * t9040;
    let t9065 = t979 * t979;
    let t9066 = F::new(1.0) / t9065;
    let t9067 = t73 * t9066;
    (t9033, t9038, t9042, t9067)
}
