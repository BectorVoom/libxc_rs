//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1954/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1954<F: Float>(t671: F, t7039: F, t2035: F, t2363: F, t2319: F, t7786: F, t2032: F, t24001: F, t26076: F, t7026: F, t7035: F, t7435: F, t84174: F, t84196: F, t84198: F, t84200: F, t84203: F, t84205: F, t84207: F, t84220: F, t90160: F, t90297: F) -> (F, F, F, F) {
    let t91854 = t7039 * t671;
    let t91857 = t2035 * t2363;
    let t91870 = t7786 * t2319;
    let t91888 = -F::new(160.0) / F::new(9.0) * t84174 + F::new(80.0) / F::new(9.0) * t84196 + F::new(80.0) / F::new(9.0) * t84198 + F::new(40.0) / F::new(9.0) * t84200 + F::new(32.0) / F::new(9.0) * t84203 + F::new(16.0) / F::new(9.0) * t84205 + F::new(32.0) / F::new(9.0) * t84207 - F::new(80.0) / F::new(3.0) * t84220 - F::new(2.0) / F::new(3.0) * t90160 * t2032 - F::new(4.0) / F::new(3.0) * t26076 * t7035 - F::new(2.0) / F::new(3.0) * t7435 * t24001 - F::new(5.0) / F::new(3.0) * t7026 * t90297;
    (t91854, t91857, t91870, t91888)
}
