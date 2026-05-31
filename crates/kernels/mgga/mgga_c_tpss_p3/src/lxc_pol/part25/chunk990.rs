//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 990/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk990<F: Float>(t30: F, t33: F, t1197: F, t13334: F, t13646: F, t13651: F, t1989: F, t4380: F, t580: F, t5335: F, t9936: F, t3225: F, t5059: F, t1006: F, t1201: F, t13603: F, t4388: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t13657 = piecewise3::<F>(t31, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t13646 * t580 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t4380 * t1989 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t13651 * t580 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1197 * t13334);
    let t13658 = t9936 * t5335;
    let t13663 = t3225 * t5059;
    let t13669 = piecewise3::<F>(t34, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t13658 * t1006 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t4388 * t1989 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t13663 * t1006 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1201 * t13603);
    (t13657, t13669)
}
