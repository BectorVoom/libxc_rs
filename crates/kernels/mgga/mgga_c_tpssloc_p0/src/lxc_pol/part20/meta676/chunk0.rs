//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2550/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2550<F: Float>(t1670: F, t3313: F, t11403: F, t3375: F, t4832: F, t11292: F, t1687: F, t50826: F, t43727: F, t43729: F, t43748: F, t43750: F, t50828: F, t50832: F, t50834: F, t50897: F, t50900: F, t50903: F, t50905: F, t50907: F, t50912: F, t50917: F, t50919: F, t50921: F, t50926: F, t50931: F, t50934: F) -> (F, F, F, F) {
    let t51667 = t3313 * t1670;
    let t51669 = F::new(18.0) * t51667 * t11403;
    let t51677 = t4832 * t3375;
    let t51680 = t1687 * t11292;
    let t51683 = F::cast_from(0.12361111111111111111e-1_f64) * t50826;
    let t51703 = t51683 - F::cast_from(0.92708333333333333334e-2_f64) * t50828 + F::cast_from(0.92708333333333333333e-2_f64) * t50832 - F::cast_from(0.96141975308641975309e-2_f64) * t50834 + F::cast_from(0.61805555555555555556e-2_f64) * t43727 - F::cast_from(0.18541666666666666667e-1_f64) * t43729 - F::cast_from(0.82407407407407407408e-2_f64) * t43748 - F::cast_from(0.34336419753086419753e-2_f64) * t43750 - F::cast_from(0.61805555555555555555e-2_f64) * t50897 - F::cast_from(0.22249999999999999999e0_f64) * t50900 - F::cast_from(0.37083333333333333333e-1_f64) * t50903 - F::cast_from(0.18541666666666666667e-1_f64) * t50905 - F::new(0.55625e-1) * t50907 + F::cast_from(0.30902777777777777778e-1_f64) * t50912 + F::cast_from(0.12361111111111111111e0_f64) * t50917 - F::cast_from(0.82407407407407407408e-2_f64) * t50919 - F::cast_from(0.10300925925925925926e-1_f64) * t50921 - F::cast_from(0.27469135802469135803e-1_f64) * t50926 + F::cast_from(0.55625000000000000001e-1_f64) * t50931 + F::cast_from(0.55625000000000000001e-1_f64) * t50934;
    (t51669, t51677, t51680, t51703)
}
