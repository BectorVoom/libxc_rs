//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 658/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk658<F: Float>(t3870: F, t5308: F, t820: F, t1367: F, t5187: F, t1341: F, t1363: F, t1831: F, t3781: F, t3783: F, t3800: F, t3803: F, t3864: F, t3867: F, t5259: F, t5289: F, t5293: F, t5303: F, t5306: F) -> (F, F, F) {
    let t5310 = t3870 * t820 * t5308;
    let t5314 = t1367 * t820 * t5187;
    let t5317 = t3803 * t5259 / F::cast_from(768.0_f64) - t1341 * t5289 / F::cast_from(3072.0_f64) - t3803 * t5293 / F::cast_from(3072.0_f64) - F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t3781 + F::cast_from(7.0_f64) / F::cast_from(4608.0_f64) * t3800 + t3864 + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t3867 - t3783 * t1831 / F::cast_from(768.0_f64) + t3803 * t5303 / F::cast_from(768.0_f64) + F::cast_from(7.0_f64) / F::cast_from(1152.0_f64) * t5306 + F::cast_from(5.0_f64) / F::cast_from(768.0_f64) * t1363 * t5310 - t1363 * t5314 / F::cast_from(768.0_f64);
    (t5310, t5314, t5317)
}
