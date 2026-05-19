//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1032/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1032<F: Float>(t225: F, t33823: F, t115339: F, t115341: F, t115354: F, t117133: F, t122145: F, t122150: F, t122152: F, t122160: F, t122164: F, t1386: F, t16030: F, t16460: F, t2092: F, t27009: F, t27068: F, t33844: F, t3758: F, t7214: F, t8794: F, t93341: F) -> F {
    let t124019 = t33823 * t225;
    let t124040 = -t117133 - t124019 * t1386 - t3758 * t33844 + F::cast_from(0.15352717957250113407e0_f64) * t115339 + F::cast_from(0.76763589786250567037e-1_f64) * t115341 + F::cast_from(0.6579736267392905746e-1_f64) * t122145 - F::new(2.0) * t93341 * t2092 + F::new(2.0) * t16460 * t8794 + F::cast_from(0.6579736267392905746e-1_f64) * t122150 - F::cast_from(0.76763589786250567037e-1_f64) * t122152 - F::new(2.0) * t27009 * t7214 - F::new(2.0) * t27068 * t7214 + F::new(2.0) * t16030 * t8794 + F::cast_from(0.3289868133696452873e-1_f64) * t115354 + F::cast_from(0.3289868133696452873e-1_f64) * t122160 - F::cast_from(0.3289868133696452873e-1_f64) * t122164;
    t124040
}
