//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 856/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk856<F: Float>(t1307: F, t1799: F, t3870: F, t820: F, t1367: F, t5187: F, t1341: F, t1363: F, t1831: F, t3781: F, t3783: F, t3800: F, t3803: F, t3864: F, t3867: F, t5259: F, t5289: F, t5293: F, t5303: F, t5306: F) -> (F, F, F, F) {
    let t5308 = t1799 * t1307;
    let t5310 = t3870 * t820 * t5308;
    let t5314 = t1367 * t820 * t5187;
    let t5317 = t3803 * t5259 / F::new(768.0) - t1341 * t5289 / F::new(3072.0) - t3803 * t5293 / F::new(3072.0) - F::new(7.0) / F::new(4608.0) * t3781 + F::new(7.0) / F::new(4608.0) * t3800 + t3864 + F::new(7.0) / F::new(1152.0) * t3867 - t3783 * t1831 / F::new(768.0) + t3803 * t5303 / F::new(768.0) + F::new(7.0) / F::new(1152.0) * t5306 + F::new(5.0) / F::new(768.0) * t1363 * t5310 - t1363 * t5314 / F::new(768.0);
    (t5308, t5310, t5314, t5317)
}
