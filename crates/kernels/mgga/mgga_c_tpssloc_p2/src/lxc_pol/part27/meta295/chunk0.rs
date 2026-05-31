//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1354/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1354<F: Float>(t10189: F, t984: F, t2990: F, t2986: F, t271: F, t2775: F, t974: F, t2769: F, t632: F, t698: F, t976: F, t979: F) -> (F, F, F, F, F, F, F) {
    let t10190 = t10189 * t984;
    let t10191 = t10190 * t2990;
    let t10192 = t2986 * t10191;
    let t10213 = F::cast_from(1.0_f64) / t271 / t2775;
    let t10214 = t974 * t10213;
    let t10216 = F::cast_from(1.0_f64) / t2769 / t632;
    let t10224 = t698 * t976;
    let t10225 = t10224 * t979;
    (t10190, t10192, t10213, t10214, t10216, t10224, t10225)
}
