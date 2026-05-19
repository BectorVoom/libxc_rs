//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 882/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk882<F: Float>(t2020: F, t8607: F, t2085: F, t225: F, t567: F, t214: F, t1985: F, t8463: F, t8468: F) -> (F, F, F, F, F) {
    let t8608 = t8607 * t2020;
    let t8611 = t2085 * t225 * t567;
    let t8612 = t214 * t8611;
    let t8613 = t1985 * t8612;
    let t8617 = F::cast_from(0.16149102437656156341e-2_f64) * t8463 + t8468 / F::new(768.0);
    (t8608, t8611, t8612, t8613, t8617)
}
