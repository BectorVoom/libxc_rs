//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1045/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1045<F: Float>(t14054: F, t14117: F, t14146: F, t14430: F, t1402: F, t2: F, t555: F, t3765: F, t3807: F, t4844: F, t8737: F, t2476: F, t4876: F) -> (F, F, F, F, F) {
    let t14432 = t14054 + t14117 + t14146 + t14430;
    let t14438 = t1402 * t2;
    let t14440 = F::new(2.0) * t14438 * t555;
    let t14447 = F::new(2.0) * t3765 * t3807;
    let t14449 = F::new(2.0) * t8737 * t4844;
    let t14451 = F::new(1.0) * t2476 * t4876;
    (t14432, t14440, t14447, t14449, t14451)
}
