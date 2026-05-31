//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2553/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2553<F: Float>(t3265: F, t3313: F, t4782: F, t11191: F, t11275: F, t4785: F, t50826: F, t50919: F, t43727: F, t43729: F, t43748: F, t43750: F, t50828: F, t50832: F, t50834: F, t50897: F, t50900: F, t50903: F, t50905: F, t50907: F, t50912: F, t50917: F, t50921: F, t50926: F, t50931: F, t50934: F) -> (F, F, F) {
    let t51741 = F::cast_from(18.0_f64) * t3313 * t4782 * t3265;
    let t51744 = F::cast_from(0.57895126195293126241e3_f64) * t11275 * t4785 * t11191;
    let t51745 = F::cast_from(0.2283111111111111111e-1_f64) * t50826;
    let t51760 = F::cast_from(0.1522074074074074074e-1_f64) * t50919;
    let t51765 = t51745 - F::cast_from(0.17123333333333333333e-1_f64) * t50828 + F::cast_from(0.17123333333333333333e-1_f64) * t50832 - F::cast_from(0.17757530864197530864e-1_f64) * t50834 + F::cast_from(0.11415555555555555555e-1_f64) * t43727 - F::cast_from(0.34246666666666666665e-1_f64) * t43729 - F::cast_from(0.1522074074074074074e-1_f64) * t43748 - F::cast_from(0.63419753086419753085e-2_f64) * t43750 - F::cast_from(0.11415555555555555555e-1_f64) * t50897 - F::cast_from(0.41095999999999999999e0_f64) * t50900 - F::cast_from(0.68493333333333333331e-1_f64) * t50903 - F::cast_from(0.34246666666666666665e-1_f64) * t50905 - F::cast_from(0.10274e0_f64) * t50907 + F::cast_from(0.57077777777777777775e-1_f64) * t50912 + F::cast_from(0.2283111111111111111e0_f64) * t50917 - t51760 - F::cast_from(0.19025925925925925925e-1_f64) * t50921 - F::cast_from(0.50735802469135802467e-1_f64) * t50926 + F::cast_from(0.10274e0_f64) * t50931 + F::cast_from(0.10274e0_f64) * t50934;
    (t51741, t51744, t51765)
}
