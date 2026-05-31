//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3136/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3136<F: Float>(t50846: F, t50848: F, t50853: F, t63911: F, t63914: F, t63918: F, t63921: F, t63924: F, t63927: F, t63930: F, t63933: F, t63936: F, t63939: F) -> F {
    let t64916 = -F::cast_from(5.0_f64) / F::cast_from(27.0_f64) * t63911 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t63914 + F::cast_from(14.0_f64) / F::cast_from(81.0_f64) * t63918 + t63921 / F::cast_from(9.0_f64) + t63924 / F::cast_from(18.0_f64) + t63927 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t63930 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t63933 - t63936 - F::cast_from(4.0_f64) * t63939 + F::cast_from(80.0_f64) / F::cast_from(81.0_f64) * t50846 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t50848 - F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t50853;
    t64916
}
