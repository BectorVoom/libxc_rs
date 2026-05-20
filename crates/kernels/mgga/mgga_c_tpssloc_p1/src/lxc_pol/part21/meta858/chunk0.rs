//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3115/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3115<F: Float>(t14961: F, t4869: F, t18915: F, t3415: F, t14858: F, t4875: F, t15838: F, t19267: F, t3633: F, t4700: F, t63280: F, t64446: F, t64447: F, t64454: F, t64456: F, t64458: F, t64460: F, t64462: F, t64464: F) -> (F, F, F, F) {
    let t64466 = F::cast_from(0.46785788981077169656e1_f64) * t4869 * t14961;
    let t64470 = F::cast_from(0.11696447245269292414e1_f64) * t18915 * t3415;
    let t64472 = F::cast_from(0.46785788981077169656e1_f64) * t14858 * t4875;
    let t64473 = F::new(8.0) * t15838 * t4700 * t64447 - t19267 * t3633 * t4700 + t63280 + t64446 - t64454 - t64456 - t64458 - t64460 - t64462 - t64464 + t64466 + t64470 + t64472;
    (t64466, t64470, t64472, t64473)
}
