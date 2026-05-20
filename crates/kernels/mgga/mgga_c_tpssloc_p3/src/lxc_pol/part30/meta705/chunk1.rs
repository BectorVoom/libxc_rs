//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2309/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2309<F: Float>(t5936: F, t6743: F, t1022: F, t5392: F, t6800: F, t23518: F, t5928: F, t17843: F, t1949: F, t23346: F, t23604: F, t23633: F, t25554: F, t28602: F, t28610: F, t3180: F, t5844: F, t6687: F, t6805: F, t83239: F, t83240: F, t83245: F, t884: F, t89256: F, t89292: F, t89294: F, t89296: F) -> (F, F) {
    let t100231 = t6743 * t5936;
    let t100236 = t5392 * t1022 * t6800;
    let t100240 = t23518 * t5928;
    let t100253 = t89256 + F::new(2.0) * t3180 * t28602 + F::cast_from(0.27415567780803773942e-2_f64) * t23633 * t100231 * t25554 + F::cast_from(0.36554090374405031923e-2_f64) * t83239 * t83240 * t100236 - F::cast_from(0.27415567780803773942e-2_f64) * t83245 * t100240 * t23604 * t884 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t17843 * t1949 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t5844 * t6805 - t89292 + t89294 - t89296 - F::cast_from(0.14621636149762012769e-1_f64) * t23346 * t28610;
    (t100236, t100253)
}
