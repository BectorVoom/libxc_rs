//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2184/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2184<F: Float>(t23270: F, t2379: F, t25039: F, t87642: F, t1880: F, t23218: F, t25224: F, t6562: F, t6572: F, t86893: F, t23171: F, t23228: F, t7488: F) -> (F, F, F, F) {
    let t87765 = t87642 * t23270 * t25039 * t2379;
    let t87773 = t1880 * t25224 * t23218;
    let t87776 = t6562 * t86893 * t6572;
    let t87777 = F::cast_from(0.82246703342411321824e-2_f64) * t87776;
    let t87779 = t23171 * t23228 * t7488;
    (t87765, t87773, t87777, t87779)
}
