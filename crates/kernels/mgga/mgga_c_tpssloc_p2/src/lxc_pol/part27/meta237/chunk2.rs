//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1136/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1136<F: Float>(t5: F, t1860: F, t1865: F, t6486: F, t6490: F, t6492: F, t6495: F, t6506: F, t6510: F, t112: F) -> (F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t6514 = piecewise3::<F>(t8, F::cast_from(0.0_f64), -t6486 * t1865 / F::cast_from(6.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t6490 * t6492 + t6495 * t1865 / F::cast_from(3.0_f64) - t1860 * t6506 / F::cast_from(6.0_f64) - t1860 * t6510 / F::cast_from(6.0_f64));
    let t6515 = t6514 * t112;
    (t6514, t6515)
}
