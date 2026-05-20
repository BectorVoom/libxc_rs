//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2116/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2116<F: Float>(t5: F, t96409: F, t96441: F, t96478: F, t96509: F, t96545: F, t96579: F, t96605: F, t96649: F, t112: F, t5456: F, t6514: F, t19534: F, t88: F) -> (F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t96653 = piecewise3::<F>(t8, F::new(0.0), t96409 + t96441 + t96478 + t96509 + t96545 + t96579 + t96605 + t96649);
    let t96654 = t96653 * t112;
    let t96655 = t6514 * t5456;
    let t96657 = t88 * t19534;
    (t96654, t96655, t96657)
}
