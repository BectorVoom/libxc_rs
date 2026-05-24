//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 847/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk847<F: Float>(t1338: F, t1830: F, t1812: F, t6120: F, t5826: F, t5829: F, t6124: F, t6126: F, t6128: F) -> (F, F, F) {
    let t6328 = t1830 * t1338;
    let t6331 = t1812 * t6120;
    let t6337 = -t5826 - t6124 / F::new(24.0) - t6126 / F::new(768.0) - t5829 - t6128 / F::new(192.0);
    (t6328, t6331, t6337)
}
