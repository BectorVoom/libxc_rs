//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1274/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1274<F: Float>(t1240: F, t8087: F, t7284: F, t8054: F, t1170: F, t2121: F, t34237: F, t24574: F, t34251: F, t24826: F, t34292: F, t118111: F, t1209: F, t1244: F, t1246: F, t1734: F, t2144: F, t24812: F, t24813: F, t24833: F, t27470: F, t27491: F, t27497: F, t27507: F, t27536: F, t27549: F, t27550: F, t27724: F, t3242: F, t32451: F, t32465: F, t32466: F, t34291: F, t3502: F, t3961: F, t7373: F, t7375: F, t7376: F) -> (F, F, F, F, F) {
    let t125295 = t1240 * t8087;
    let t125306 = t7284 * t8054;
    let t125311 = t2121 * t1170 * t34237;
    let t125313 = t24574 * t34251;
    let t125351 = t24826 * t34292;
    let t125358 = F::cast_from(0.73108180748810063844e-2_f64) * t27549 * t27550 * t2144 * t3242 * t3961 - F::cast_from(0.16449340668482264365e-1_f64) * t7373 * t27536 * t32465 + F::cast_from(0.54831135561607547883e-2_f64) * t118111 - F::cast_from(0.16449340668482264365e-1_f64) * t7373 * t24833 * t34291 + F::cast_from(0.3289868133696452873e-1_f64) * t24812 * t24813 * t3502 * t2144 * t27491 + F::cast_from(0.16449340668482264365e-1_f64) * t7373 * t7375 * t27724 * t7376 + F::cast_from(0.16449340668482264365e-1_f64) * t7373 * t7375 * t27470 * t7376 - F::cast_from(0.43864908449286038307e-1_f64) * t27507 * t32466 + t1244 * t32451 * t1734 * t1246 + F::cast_from(0.54831135561607547883e-2_f64) * t125351 - F::cast_from(0.16449340668482264365e-1_f64) * t24812 * t24813 * t1209 * t2144 * t27497;
    (t125295, t125306, t125311, t125313, t125358)
}
