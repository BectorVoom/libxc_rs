//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2334/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2334<F: Float>(t2154: F, t45349: F, t27776: F, t95772: F, t11147: F, t497: F, t225: F, t27424: F, t27422: F, t24574: F, t27752: F, t1252: F, t14165: F, t15787: F, t15793: F, t24601: F, t24888: F, t27406: F, t27784: F, t27830: F, t3471: F, t3631: F, t466: F, t498: F, t7283: F, t7300: F, t7351: F, t8002: F, t8010: F, t85674: F, t85750: F, t86501: F, t94796: F, t95707: F) -> F {
    let t95884 = t45349 * t2154;
    let t95889 = F::cast_from(0.24369393582936687948e-2_f64) * t95772 * t27776;
    let t95890 = t497 * t11147;
    let t95899 = t27424 * t225;
    let t95902 = t27422 * t225;
    let t95912 = F::cast_from(0.54831135561607547884e-2_f64) * t24574 * t27752;
    let t95913 = -F::cast_from(0.49348022005446793095e-1_f64) * t7283 * t7300 * t85674 * t15793 - F::cast_from(0.27415567780803773942e-2_f64) * t7283 * t85750 * t8002 + F::new(24.0) * t27784 * t95884 * t15793 + t95889 - F::cast_from(0.8529287754027840782e-2_f64) * t94796 * t24601 * t95890 * t14165 + F::cast_from(0.14621636149762012769e-1_f64) * t27406 * t24888 + t466 * t95707 * t498 - F::new(2.0) * t95899 * t1252 - F::new(2.0) * t95902 * t1252 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t3471 * t8010 - t7351 * t15787 - F::cast_from(0.36554090374405031922e-2_f64) * t86501 - t27830 * t3631 - t95912;
    t95913
}
