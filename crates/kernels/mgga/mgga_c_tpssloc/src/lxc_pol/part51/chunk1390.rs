//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1390/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1390<F: Float>(t114790: F, t23164: F, t7479: F, t114866: F, t1880: F, t7488: F, t23168: F, t33419: F, t112863: F, t114785: F, t114827: F, t118828: F, t118831: F, t118837: F, t118838: F, t118841: F, t1528: F, t23281: F, t24305: F, t25168: F, t25183: F, t26728: F, t7517: F, t7842: F) -> F {
    let t121464 = t23164 * t114790 * t7479;
    let t121467 = t1880 * t114866 * t7488;
    let t121469 = t23168 * t33419;
    let t121479 = F::cast_from(0.82246703342411321825e-2_f64) * t121464 - F::cast_from(0.82246703342411321825e-2_f64) * t121467 + F::cast_from(0.38381794893125283518e-1_f64) * t121469 - F::cast_from(0.41123351671205660912e-2_f64) * t114827 + t118828 - F::new(6.0) * t25168 * t26728 * t25183 + F::new(2.0) * t24305 * t7517 + t118831 + t112863 - t118837 - t23281 * t7842 - t118838 - t118841 - t114785 * t1528;
    t121479
}
