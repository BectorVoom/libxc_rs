//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2928/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2928<F: Float>(t1068: F, t4696: F, t13508: F, t4483: F, t17934: F, t2948: F, t13718: F, t10723: F, t17954: F, t959: F, t17937: F, t2925: F) -> (F, F, F, F, F, F) {
    let t60941 = t4696 * t1068;
    let t60946 = F::cast_from(0.34631718211362927517e2_f64) * t4483 * t13508;
    let t60953 = F::cast_from(0.5848223622634646207e0_f64) * t17934 * t2948;
    let t60955 = F::cast_from(0.11696447245269292414e1_f64) * t4483 * t13718;
    let t60958 = F::cast_from(0.17315859105681463759e2_f64) * t959 * t17954 * t10723;
    let t60961 = F::cast_from(0.11696447245269292414e1_f64) * t959 * t17937 * t2925;
    (t60941, t60946, t60953, t60955, t60958, t60961)
}
