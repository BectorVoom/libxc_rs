//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 755/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk755<F: Float>(t1874: F, t28030: F, t7458: F, t7461: F, t4028: F, t7468: F, t28002: F, t19451: F, t1774: F, t7467: F, t652: F, t2006: F, t6361: F) -> (F, F, F, F, F, F, F, F, F) {
    let t28032 = F::new(2.0) * t28030 * t1874;
    let t28034 = F::new(4.0) * t7458 * t7461;
    let t28036 = F::new(4.0) * t4028 * t7468;
    let t28038 = F::new(4.0) * t28002 * t1874;
    let t28040 = F::new(4.0) * t4028 * t7461;
    let t28042 = F::new(2.0) * t19451 * t1874;
    let t28045 = t1774 * t7467;
    let t28047 = F::new(4.0) * t652 * t28045;
    let t28051 = t6361 * t2006;
    (t28032, t28034, t28036, t28038, t28040, t28042, t28045, t28047, t28051)
}
