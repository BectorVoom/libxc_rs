//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1235/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1235<F: Float>(t21569: F, t547: F, t1670: F, t1784: F, t21546: F, t21555: F, t21557: F, t21559: F, t21562: F, t21565: F, t21568: F, t5474: F, t5477: F, t548: F, t6284: F, t3205: F, t5753: F) -> (F, F) {
    let t21571 = 3.0 * t547 * t21569;
    let t21572 = 6.0 * t1670 * t6284 + 6.0 * t1784 * t5474 + 3.0 * t1784 * t5477 + t21546 * t548 + t21555 + t21557 + t21559 + t21562 + t21565 + t21568 + t21571;
    let t22964 = t3205 * t5753;
    (t21572, t22964)
}
