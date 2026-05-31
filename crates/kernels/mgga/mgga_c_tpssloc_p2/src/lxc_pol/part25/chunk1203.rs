//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1203/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1203<F: Float>(t531: F, t7216: F, t2056: F, t40772: F, t24334: F, t2752: F, t1877: F, t2057: F, t2249: F, t22951: F, t22964: F, t24191: F, t24335: F, t24344: F, t2522: F, t26756: F, t4314: F, t6542: F, t6671: F, t7110: F, t7114: F, t81489: F, t81492: F, t81501: F, t81505: F, t81521: F, t81529: F, t81543: F, t82313: F, t82323: F, t9257: F) -> (F, F, F, F) {
    let t84733 = t531 * t7216;
    let t84766 = t2056 * t40772;
    let t84791 = t24334 * t2752;
    let t84795 = t1877 * t2057 * t9257 / F::cast_from(2.0_f64) - t1877 * t7114 * t82323 / F::cast_from(2.0_f64) + F::cast_from(9.0_f64) * t4314 * t2057 * t81543 + F::cast_from(3.0_f64) * t1877 * t24344 * t81521 + F::cast_from(3.0_f64) * t26756 * t81492 - F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t24191 * t81489 - F::cast_from(3.0_f64) * t1877 * t84766 * t82313 + F::cast_from(9.0_f64) * t4314 * t7110 * t22951 + F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t2522 * t24335 * t6542 + F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t2522 * t2057 * t81501 + F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t2522 * t2057 * t81505 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1877 * t7114 * t81529 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1877 * t7110 * t2249 + F::cast_from(9.0_f64) * t2522 * t7110 * t22964 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1877 * t84791 * t6671;
    (t84733, t84766, t84791, t84795)
}
