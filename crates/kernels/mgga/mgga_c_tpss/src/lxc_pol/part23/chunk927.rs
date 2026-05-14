//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 927/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk927<F: Float>(t2192: F, t359: F, t361: F, t355: F, t215: F, t334: F, t68: F, t333: F, t219: F, t2769: F, t979: F, t73: F, t8549: F, t8552: F, t8548: F, t2717: F, t328: F) -> (F, F, F, F, F, F) {
    let t9036 = t359 * t2192 * t361;
    let t9038 = t355 * t9036 / 10368.0;
    let t9040 = t215 * t68 * t334;
    let t9042 = 5.0 / 1296.0 * t333 * t9040;
    let t9058 = t2769 * t219;
    let t9065 = t979 * t979;
    let t9066 = 1.0 / t9065;
    let t9067 = t73 * t9066;
    let t9076 = t8549 * t8552;
    let t9077 = t8548 * t9076;
    let t9080 = 1.0 / t2717 / t328;
    (t9038, t9042, t9058, t9067, t9077, t9080)
}
