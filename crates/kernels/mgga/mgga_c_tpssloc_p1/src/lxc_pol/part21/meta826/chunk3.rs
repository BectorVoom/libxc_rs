//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2915/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2915<F: Float>(t10756: F, t10765: F, t10828: F, t13716: F, t14271: F, t14276: F, t14425: F, t14429: F, t14432: F, t14436: F, t17492: F, t17499: F, t17535: F, t2905: F, t2906: F, t2924: F, t2930: F, t42111: F, t42113: F, t4416: F, t4438: F, t4475: F, t48789: F, t49427: F, t49430: F, t5774: F, t5791: F, t60722: F, t60741: F, t60744: F, t60748: F, t60750: F, t60752: F) -> F {
    let t60763 = -F::cast_from(0.11696447245269292414e1_f64) * t2905 * t5791 * t2924 - F::cast_from(0.10389515463408878255e3_f64) * t10828 * t17492 * t2906 + F::cast_from(0.17315859105681463759e2_f64) * t2930 * t17492 * t2924 + F::cast_from(0.10254018858216406658e4_f64) * t10756 * t60722 * t2906 + F::cast_from(0.34631718211362927518e2_f64) * t2930 * t4475 * t13716 + F::cast_from(0.10254018858216406658e4_f64) * t10756 * t17499 * t2924 + F::cast_from(0.91082604192152556044e5_f64) * t42111 * t5774 * t42113 * t2906 - F::new(4.0) * t14276 * t14429 - F::cast_from(0.38596750796862084161e3_f64) * t49430 * t14432 - t60741 + t60744 - t60748 - t60750 - t60752 - F::new(8.0) * t49427 * t4416 + F::cast_from(0.12865583598954028054e3_f64) * t48789 * t4438 - F::new(8.0) * t14276 * t14425 + F::cast_from(0.12865583598954028054e3_f64) * t14271 * t14436 + F::new(12.0) * t10765 * t17535;
    t60763
}
