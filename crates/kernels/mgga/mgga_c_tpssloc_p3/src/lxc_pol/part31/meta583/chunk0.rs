//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1823/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1823<F: Float>(t22704: F, t5336: F, t80798: F, t22724: F, t26436: F, t81066: F, t26423: F, t81159: F, t215: F, t22839: F, t562: F, t80854: F) -> (F, F, F, F, F, F) {
    let t90898 = t22704 * t80798 * t5336;
    let t90900 = t22724 * t26436;
    let t90903 = F::cast_from(0.16449340668482264365e-1_f64) * t81066;
    let t90912 = t81159 * t26423;
    let t90914 = t22839 * t215;
    let t90915 = t80854 * t562;
    (t90898, t90900, t90903, t90912, t90914, t90915)
}
