//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 573/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk573<F: Float>(t1573: F, t942: F, t1581: F, t950: F, t2766: F, t2824: F, t2912: F, t2919: F, t4335: F, t4340: F, t4345: F, t4349: F, t4363: F, t4371: F, t4379: F, t4381: F, t4384: F, t4387: F, t4390: F, t4393: F) -> (F, F, F) {
    let t4449 = t1573 * t942;
    let t4454 = t1581 * t950;
    let t4471 = -F::new(0.1294625e1) * t4363 + F::new(0.258925e1) * t4371 + t2912 + F::new(0.10064166666666666667e0) * t2766 + F::new(0.10064166666666666667e0) * t4335 - F::new(0.20128333333333333333e0) * t4340 + F::new(0.60385e0) * t4345 - F::new(0.301925e0) * t4349 + F::new(0.82524375e-1) * t4379 + F::new(0.16504875e0) * t4381 + t2919 + F::new(0.5519e-1) * t2824 + F::new(0.5519e-1) * t4384 - F::new(0.27595e-1) * t4387 + F::new(0.16557e0) * t4390 - F::new(0.82785e-1) * t4393;
    (t4449, t4454, t4471)
}
