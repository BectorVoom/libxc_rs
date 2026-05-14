//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 813/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk813<F: Float>(t10108: F, t68: F, t261: F, t2751: F, t1053: F, t1887: F, t337: F, t615: F, t134: F, t976: F, t984: F, t271: F, t2775: F, t974: F, t2769: F, t632: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10109 = 1.0 / t10108;
    let t10110 = t68 * t10109;
    let t10143 = 1.0 / t2751 / t261;
    let t10163 = t1053 * t1053;
    let t10164 = 1.0 / t10163;
    let t10165 = t68 * t10164;
    let t10186 = t615 * t337 * t1887;
    let t10189 = t134 * t976;
    let t10190 = t10189 * t984;
    let t10213 = 1.0 / t271 / t2775;
    let t10214 = t974 * t10213;
    let t10216 = 1.0 / t2769 / t632;
    (t10110, t10143, t10165, t10186, t10189, t10190, t10213, t10214, t10216)
}
